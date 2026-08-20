import placo
import numpy as np
from ischedule import schedule, run_loop
from placo_utils.visualization import robot_viz, frame_viz, point_viz
import webbrowser
import argparse


parser = argparse.ArgumentParser(description="Choose gait type")
parser.add_argument("gait_type", type=str, choices=["walk", "trot"], help="gait type")
args = parser.parse_args()


# ============================================================
# Outils
# ============================================================
def rot_z(a):
    ca, sa = np.cos(a), np.sin(a)
    return np.array([
        [ca, -sa, 0.0],
        [sa,  ca, 0.0],
        [0.0, 0.0, 1.0]
    ])


def wrap_to_pi(a):
    return (a + np.pi) % (2.0 * np.pi) - np.pi


def eqt_walk(x0, xf, t, T, H):
    u = t / T
    x = x0 + (xf - x0) * (3 * u ** 2 - 2 * u ** 3)
    z = H * np.sin(np.pi * u)
    return np.array([x, z])


def eqt_trot(x0, xf, t, T, H):
    s = t / T
    x = (1 - s) ** 2 * x0 + 2 * (1 - s) * s * ((x0 + xf) / 2.0) + s ** 2 * xf
    z = 4.0 * H * (1 - s) * s
    return np.array([x, z])


def yaw_from_rotation(R):
    return np.arctan2(R[1, 0], R[0, 0])


# ============================================================
# Paramètres robot
# ============================================================
URDF_PATH = "/home/mtouri/Desktop/Maha_folder/spot_descritpion/model.urdf"
BASE_FRAME = "base"
FEET = ["fl.foot", "fr.foot", "hl.foot", "hr.foot"]

dt = 0.01

# marche
H = 0.09
T = 0.7
X_SWING = 0.08

# rotation max par phase
YAW_STEP_MAX = 0.18

# objectif monde
X_GOAL = 2
Y_GOAL = 5
GOAL_TOL = 0.08
N_PHASES_MAX = 300

walk_order = ["hl.foot", "fr.foot", "hr.foot", "fl.foot"]
trot_order = [["fl.foot", "hr.foot"], ["fr.foot", "hl.foot"]]

# Posture "accroupie" type Spot
spot_posture = {
    "fl.hy": 0.85,  "fl.kn": -1.55,
    "fr.hy": 0.85,  "fr.kn": -1.55,
    "hl.hy": 0.85,  "hl.kn": -1.55,
    "hr.hy": 0.85,  "hr.kn": -1.55,
}


# ============================================================
# Chargement robot
# ============================================================
robot = placo.RobotWrapper(URDF_PATH, placo.Flags.ignore_collisions)
robot.update_kinematics()

# ------------------------------------------------------------
# Appliquer une posture initiale accroupie
# ------------------------------------------------------------
for joint, value in spot_posture.items():
    robot.set_joint(joint, value)

robot.update_kinematics()

# ------------------------------------------------------------
# Poser les pieds sur le ground
# ------------------------------------------------------------
T_world_base = robot.get_T_world_frame(BASE_FRAME).copy()
T_world_foot = robot.get_T_world_frame("hl.foot")
T_world_base[2, 3] -= T_world_foot[2, 3] - 0.05

robot.set_T_world_frame(BASE_FRAME, T_world_base)
robot.update_kinematics()

# ============================================================
# Solveur IK
# ============================================================
solver = placo.KinematicsSolver(robot)
solver.dt = dt
solver.mask_fbase(False)
solver.enable_joint_limits(True)
solver.enable_velocity_limits(True)
solver.add_regularization_task(1e-4)

# ============================================================
# Etat initial
# ============================================================
T_world_base_0 = robot.get_T_world_frame(BASE_FRAME).copy()
goal_world = np.array([X_GOAL, Y_GOAL, T_world_base_0[2, 3]])

foot_targets_world = {}
for foot in FEET:
    foot_targets_world[foot] = robot.get_T_world_frame(foot)[:3, 3].copy()

# orientation locale initiale de chaque pied par rapport à la base
foot_R_base_ref = {}
R_world_base_0 = T_world_base_0[:3, :3].copy()
for foot in FEET:
    R_world_foot_0 = robot.get_T_world_frame(foot)[:3, :3].copy()
    foot_R_base_ref[foot] = R_world_base_0.T @ R_world_foot_0

# ============================================================
# Tâches IK
# ============================================================

# Hauteur base seulement
base_pos_task = solver.add_position_task(BASE_FRAME, T_world_base_0[:3, 3].copy())
base_pos_task.configure("base_pos_task", "soft", 1.0)
base_pos_task.mask.set_axises("z")

# Roll / pitch stabilisés
base_rp_task = solver.add_orientation_task(BASE_FRAME, T_world_base_0[:3, :3].copy())
base_rp_task.configure("base_rp_task", "soft", 1.0)
base_rp_task.mask.set_axises("xy")

# Yaw piloté
base_yaw_task = solver.add_orientation_task(BASE_FRAME, T_world_base_0[:3, :3].copy())
base_yaw_task.configure("base_yaw_task", "soft", 1.0)
base_yaw_task.mask.set_axises("z")

# Pieds : position
foot_tasks = {}
for foot in FEET:
    task = solver.add_position_task(foot, foot_targets_world[foot].copy())
    task.configure(f"{foot}_task", "hard", 1.0)
    foot_tasks[foot] = task

# Pieds : orientation qui suit le yaw de la base
foot_ori_tasks = {}
for foot in FEET:
    ori_task = solver.add_orientation_task(foot, robot.get_T_world_frame(foot)[:3, :3].copy())
    ori_task.configure(f"{foot}_ori_task", "soft", 1.0)
    foot_ori_tasks[foot] = ori_task

# Posture des jambes
joints_task = solver.add_joints_task()
joints_task.configure("spot_posture", "soft", 0.2)
joints_task.set_joints(spot_posture)

# ============================================================
# Visualisation
# ============================================================
viz = robot_viz(robot)
webbrowser.open(viz.viewer.url())

# ============================================================
# Planificateur de marche
# ============================================================
phase_time = 0.0
phase_index = 0
done = False

swing_feet = []
swing_start = {}
swing_target = {}

current_yaw_goal = 0.0
current_yaw_step = 0.0


def prepare_next_phase():
    global phase_time, phase_index, done
    global swing_feet, swing_start, swing_target
    global current_yaw_goal, current_yaw_step

    phase_time = 0.0

    # état base courant
    T_world_base = robot.get_T_world_frame(BASE_FRAME).copy()
    p_base = T_world_base[:3, 3].copy()
    R_base = T_world_base[:3, :3].copy()
    yaw_base = yaw_from_rotation(R_base)

    # distance restante
    dxy = goal_world[:2] - p_base[:2]
    dist_goal = np.linalg.norm(dxy)

    if dist_goal < GOAL_TOL or phase_index >= N_PHASES_MAX:
        done = True
        return

    # yaw désiré vers la cible
    current_yaw_goal = np.arctan2(dxy[1], dxy[0])

    # variation de yaw autorisée pendant la phase
    yaw_error = wrap_to_pi(current_yaw_goal - yaw_base)
    current_yaw_step = np.clip(yaw_error, -YAW_STEP_MAX, YAW_STEP_MAX)

    # choix des pieds swing
    if args.gait_type == "walk":
        swing_feet = [walk_order[phase_index % len(walk_order)]]
    else:
        swing_feet = trot_order[phase_index % len(trot_order)]

    swing_start = {}
    swing_target = {}

    # pas avant limité par la distance restante
    step_len = min(X_SWING, dist_goal)

    # direction d'avance selon yaw désiré
    d_forward = np.array([
        step_len * np.cos(current_yaw_goal),
        step_len * np.sin(current_yaw_goal)
    ])

    # pour chaque pied swing :
    # on prend sa position relative actuelle à la base
    # on la fait tourner d'un petit angle current_yaw_step
    # puis on ajoute une translation d'avance
    R_step = rot_z(current_yaw_step)[:2, :2]

    for foot in swing_feet:
        p_foot = foot_targets_world[foot].copy()
        swing_start[foot] = p_foot.copy()

        r_xy = p_foot[:2] - p_base[:2]
        r_xy_rot = R_step @ r_xy

        p_target_xy = p_base[:2] + r_xy_rot + d_forward

        swing_target[foot] = p_foot.copy()
        swing_target[foot][0] = p_target_xy[0]
        swing_target[foot][1] = p_target_xy[1]

    phase_index += 1


prepare_next_phase()

# ============================================================
# Boucle
# ============================================================
t = 0.0


@schedule(interval=dt)
def loop():
    global t, phase_time, done, current_yaw_goal
    t += dt

    T_world_base = robot.get_T_world_frame(BASE_FRAME).copy()
    p_base = T_world_base[:3, 3].copy()
    R_base = T_world_base[:3, :3].copy()

    # yaw désiré vers la cible
    dxy = goal_world[:2] - p_base[:2]
    dist_goal = np.linalg.norm(dxy)

    if dist_goal > 1e-9:
        current_yaw_goal = np.arctan2(dxy[1], dxy[0])

    R_goal = rot_z(current_yaw_goal)

    # base : orientation désirée
    base_rp_task.R_world_frame = R_goal
    base_yaw_task.R_world_frame = R_goal

    # pieds : orientation = yaw base désiré + orientation locale initiale
    for foot in FEET:
        foot_ori_tasks[foot].R_world_frame = R_goal @ foot_R_base_ref[foot]

    if not done:
        phase_time += dt
        tau = min(phase_time, T)

        for foot in FEET:
            if foot in swing_feet:
                delta_xy = swing_target[foot][:2] - swing_start[foot][:2]
                path_len = np.linalg.norm(delta_xy)

                if path_len < 1e-9:
                    s_horiz = 0.0
                    z_up = 0.0
                else:
                    if args.gait_type == "walk":
                        sz = eqt_walk(0.0, path_len, tau, T, H)
                    else:
                        sz = eqt_trot(0.0, path_len, tau, T, H)

                    s_horiz = sz[0]
                    z_up = sz[1]

                direction_xy = delta_xy / path_len if path_len > 1e-9 else np.zeros(2)

                p = swing_start[foot].copy()
                p[:2] = swing_start[foot][:2] + direction_xy * s_horiz
                p[2] = swing_start[foot][2] + z_up
                foot_tasks[foot].target_world = p
            else:
                foot_tasks[foot].target_world = foot_targets_world[foot]

        if phase_time >= T:
            for foot in swing_feet:
                foot_targets_world[foot] = swing_target[foot].copy()
            prepare_next_phase()

    else:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

    # Solve IK
    solver.solve(True)
    robot.update_kinematics()

    # Affichage
    viz.display(robot.state.q)

    for foot in FEET:
        T_vis = np.eye(4)
        T_vis[:3, 3] = foot_tasks[foot].target_world
        frame_viz(f"{foot}_target", T_vis, 0.10)

    frame_viz("base_frame", robot.get_T_world_frame(BASE_FRAME), 0.2)

    T_goal = np.eye(4)
    T_goal[:3, 3] = goal_world
    frame_viz("goal_frame", T_goal, 0.2)
    point_viz("goal_point", goal_world, radius=0.04, color=0x0000FF)


run_loop()
