import placo
import numpy as np
from ischedule import schedule, run_loop
from placo_utils.visualization import robot_viz, frame_viz, point_viz, robot_frame_viz
import webbrowser
from placo_utils.tf import tf
import meshcat.geometry as g
import meshcat.transformations as tfm


#Fonctions utiles------------------------

# Eqt de Bézier+ parabole, utiliser pour lisser la courbe, pas de vitesses brusque et avoir Z(0)et Z(T)=0
def eqt_trot(x0, xf, t, T, H):
    s = t / T
    x = (1 - s) ** 2 * x0 + 2 * (1 - s) * s * ((x0 + xf) / 2.0) + s ** 2 * xf
    z = 4.0 * H * (1 - s) * s
    return np.array([x, z])


def rot_x(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[1, 0, 0],
                     [0, c, -s],
                     [0, s,  c]])


def rot_y(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[ c, 0, s],
                     [ 0, 1, 0],
                     [-s, 0, c]])


def rot_z(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[c, -s, 0],
                     [s,  c, 0],
                     [0,  0, 1]])


def rpy_to_R(roll, pitch, yaw):
    return rot_z(yaw) @ rot_y(pitch) @ rot_x(roll)


def smooth(s):
    s = np.clip(s, 0.0, 1.0)
    return 3 * s**2 - 2 * s**3


def rpy_to_quat(roll, pitch, yaw):
    cr, sr = np.cos(roll / 2), np.sin(roll / 2)
    cp, sp = np.cos(pitch / 2), np.sin(pitch / 2)
    cy, sy = np.cos(yaw / 2), np.sin(yaw / 2)
    qw = cr * cp * cy + sr * sp * sy
    qx = sr * cp * cy - cr * sp * sy
    qy = cr * sp * cy + sr * cp * sy
    qz = cr * cp * sy - sr * sp * cy
    return np.array([qx, qy, qz, qw])

#sachant que j'ai une position du cercle (avec son theta), je veux que mon tcp s'y retrouve avec une orientation 
# (output de cette fonction) mais avec les contraintes recpecté(tangent au cercle)
# def tcp_rotation_tangent(theta, direction=1.0):
#     z_tcp = direction * np.array([-np.sin(theta),0.0,np.cos(theta)])
#     z_tcp = z_tcp / np.linalg.norm(z_tcp)
#     y_ref = np.array([0.0, 1.0, 0.0])
#     x_tcp = np.cross(y_ref, z_tcp)
#     x_tcp = x_tcp / np.linalg.norm(x_tcp)
#     y_tcp = np.cross(z_tcp, x_tcp)
#     y_tcp = y_tcp / np.linalg.norm(y_tcp)
#     return np.column_stack([x_tcp, y_tcp, z_tcp])

def tcp_rotation_tangent(theta, direction=-1.0):
    # Rayon du cercle dans le plan ZY
    radial = np.array([
        0.0,
        np.sin(theta),
        np.cos(theta)
    ])
    radial /= np.linalg.norm(radial)

    # Le TCP pointe vers le centre du cercle
    z_tcp = direction * radial
    z_tcp /= np.linalg.norm(z_tcp)

    # Axe du cercle = X
    x_tcp = np.array([1.0, 0.0, 0.0])

    # Compléter la base orthonormée
    y_tcp = np.cross(z_tcp, x_tcp)
    y_tcp /= np.linalg.norm(y_tcp)

    x_tcp = np.cross(y_tcp, z_tcp)
    x_tcp /= np.linalg.norm(x_tcp)

    return np.column_stack([x_tcp, y_tcp, z_tcp])

def orientation_error_rad(R_target, R_cur):
    R_diff = R_target.T @ R_cur
    cos_a = np.clip((np.trace(R_diff) - 1.0) / 2.0, -1.0, 1.0)
    return abs(np.arccos(cos_a))


def z_axis_alignment_error(R_target, R_cur):
    z_target = R_target[:, 2]
    z_cur    = R_cur[:, 2]
    c = np.clip(np.dot(z_target, z_cur), -1.0, 1.0)
    return abs(np.arccos(c))


def circle_target(theta, cylinder_center, radius):
    return cylinder_center + np.array([
        0.0,
        radius * np.sin(theta),
        radius * np.cos(theta)
    ])


def candidate_rank(res):
    return (res["pos_err"], res["ori_err"], -res["manip"])


def active_joint_names(robot):
    return list(robot.joint_names())


def set_joint_dict(robot, joint_dict):
    for name, value in joint_dict.items():
        robot.set_joint(name, float(value))


def zero_joint_dict(joint_names):
    return {name: 0.0 for name in joint_names}


def q_to_joint_dict(robot, q, joint_names):
    out = {}
    for joint_name in joint_names:
        jid = robot.model.getJointId(joint_name)
        j   = robot.model.joints[jid]
        out[joint_name] = float(q[j.idx_q])
    return out


def manipulability_score(robot_w, tcp_frame):
    robot_w.update_kinematics()
    J    = robot_w.frame_jacobian(tcp_frame, "local_world_aligned")
    JJt  = J @ J.T
    detv = np.linalg.det(JJt)
    return float(np.sqrt(max(detv, 0.0)))

def open_gripper():
    arm.set_joint("left_finger_joint", 0.0185)
    arm.set_joint("right_finger_joint", -0.0185)
    arm.update_kinematics()

# Paramètres/ variables ------------------


SPOT_URDF    = "/home/mtouri/Desktop/Maha_folder/spot_descritpion/model.urdf"
BASE_FRAME   = "base"
FEET         = ["fl.foot", "fr.foot", "hl.foot", "hr.foot"]

ARM_MJCF       = "/home/mtouri/Desktop/Maha_folder/robot_descriptions_maha/mjcf_description/nb-55-v7_motor_gripper.xml"
ARM_BASE_FRAME = "lower_end_0"
ARM_TCP        = "gripper_tcp"


CYLINDER_CENTER = np.array([1.5, 0.1, 0.6])
CYLINDER_RADIUS = 0.11

TARGET_START = CYLINDER_CENTER + np.array([0.0, 0.0, CYLINDER_RADIUS])
TARGET_END    = CYLINDER_CENTER + np.array([0.0, 0.0, CYLINDER_RADIUS])
TARGET_BACK   = CYLINDER_CENTER + np.array([-CYLINDER_RADIUS, 0.0, 0.0])
TARGET_BOTTOM = CYLINDER_CENTER + np.array([0.0, 0.0, -CYLINDER_RADIUS])

RETREAT_DX              = -0.25
RETREAT_DURATION        = 2.0
RETREAT_PITCH_FORWARD   = 0.2
RETREAT_PITCH_DURATION  = 2

ARM_OPEN_DURATION = 2.0
GRIPPER_OPEN_VALUE = 0.0185
GRIPPER_CLOSED_VALUE = 0.0

ARM_OPEN_TARGET = CYLINDER_CENTER + np.array([0.0,0.0,CYLINDER_RADIUS + 0.05])

ARM_OPEN_R = tcp_rotation_tangent(0.0, direction=-1.0)

T_spot_base_arm_mount = np.eye(4)
T_spot_base_arm_mount[0, 3] = 0.35
T_spot_base_arm_mount[1, 3] = 0.0
T_spot_base_arm_mount[2, 3] = 0.08
T_arm_mount_inv = np.linalg.inv(T_spot_base_arm_mount)

X_MIN, X_MAX = CYLINDER_CENTER[0] - 0.2, CYLINDER_CENTER[0] + 0.2
Y_MIN, Y_MAX = CYLINDER_CENTER[1] - 0.2, CYLINDER_CENTER[1] + 0.2
Z_MIN, Z_MAX = CYLINDER_CENTER[2] - 0.2, CYLINDER_CENTER[2] + 0.2

ROLL_MIN,  ROLL_MAX  = -0.52,    0.52
PITCH_MIN, PITCH_MAX = -0.52,    0.52
YAW_MIN,   YAW_MAX   = -np.pi,   np.pi

BASE_CLEARANCE = 0.4

SA_STEPS = 500
IK_ITERS = 500
POS_TOL  = 0.01
ORI_TOL  = 0.10

dt                   = 0.01
H                    = 0.08
T_TROT               = 0.4
X_SWING              = 0.2
ROTATION_DURATION    = 1.5
ROTATION_DURATION_RPY = 0.4
Z_TRANSITION_DURATION= 1.5

TRAJ_DURATION   = 6.0
TRAJ_DURATION_2 = 6.0
TRAJ_DURATION_3 = 6.0

PRINT_JOINTS = True


# Algo Simulated annealing pour la recherche de la base opti pour le bras

robot_search = placo.RobotWrapper(ARM_MJCF, placo.Flags.mjcf | placo.Flags.ignore_collisions)
solver_search = placo.KinematicsSolver(robot_search)
solver_search.mask_fbase(True)
solver_search.dt = dt
solver_search.enable_joint_limits(True)
solver_search.enable_velocity_limits(False)
solver_search.add_regularization_task(1e-4)

joint_names_search = active_joint_names(robot_search)

tcp_position_task_search = solver_search.add_position_task(ARM_TCP, robot_search.get_T_world_frame(ARM_TCP)[:3, 3].copy())
tcp_position_task_search.configure("tcp_pos_search", "soft", 100.0)

tcp_orientation_task_search = solver_search.add_orientation_task(ARM_TCP, robot_search.get_T_world_frame(ARM_TCP)[:3, :3].copy())
tcp_orientation_task_search.configure("tcp_ori_search", "soft", 100.0)
tcp_orientation_task_search.mask.set_axises("xyz", "local")

seed_task_search = solver_search.add_joints_task()
seed_task_search.configure("seed_search", "soft", 1.0)
seed_task_search.set_joints(zero_joint_dict(joint_names_search))


def base_is_valid(base_pose):
    x, y, z, roll, pitch, yaw = base_pose
    if not (X_MIN <= x <= X_MAX): return False
    if not (Y_MIN <= y <= Y_MAX): return False
    if not (Z_MIN <= z <= Z_MAX): return False
    if not (ROLL_MIN  <= roll  <= ROLL_MAX):  return False
    if not (PITCH_MIN <= pitch <= PITCH_MAX): return False
    if not (YAW_MIN   <= yaw   <= YAW_MAX):   return False
    dx = x - CYLINDER_CENTER[0]
    dz = z - CYLINDER_CENTER[2]
    if np.sqrt(dx * dx + dz * dz) < (CYLINDER_RADIUS + BASE_CLEARANCE):
        return False
    return True


def solve_ik_for_base(base_pose, target, R_target):
    x, y, z, roll, pitch, yaw = base_pose
    T_base = np.eye(4)
    T_base[:3, 3]  = [x, y, z]
    T_base[:3, :3] = rpy_to_R(roll, pitch, yaw)

    q_current = q_to_joint_dict(robot_search, robot_search.state.q, joint_names_search)
    robot_search.set_T_world_frame(ARM_BASE_FRAME, T_base)
    robot_search.update_kinematics()

    tcp_position_task_search.target_world  = target
    tcp_orientation_task_search.R_world_frame = R_target
    seed_task_search.set_joints(q_current)

    for _ in range(IK_ITERS):
        solver_search.solve(True)
        robot_search.update_kinematics()
        T_tcp   = robot_search.get_T_world_frame(ARM_TCP).copy()
        pos_err = np.linalg.norm(target - T_tcp[:3, 3])
        ori_err = z_axis_alignment_error(R_target, T_tcp[:3, :3])
        if pos_err < POS_TOL and ori_err < ORI_TOL:
            break

    T_tcp   = robot_search.get_T_world_frame(ARM_TCP).copy()
    pos_err = float(np.linalg.norm(target - T_tcp[:3, 3]))
    ori_err = float(z_axis_alignment_error(R_target, T_tcp[:3, :3]))
    manip   = manipulability_score(robot_search, ARM_TCP)
    q_sol   = robot_search.state.q.copy()

    return {
        "base_pose": np.array(base_pose),
        "q":         q_sol,
        "joint_dict": q_to_joint_dict(robot_search, q_sol, joint_names_search),
        "T_tcp":     T_tcp,
        "pos_err":   pos_err,
        "ori_err":   ori_err,
        "manip":     manip,
        "ok":        (pos_err <= POS_TOL and ori_err <= ORI_TOL),
    }


def run_sa_for_orientation(target, R_target, start_base):
    current = np.array(start_base, dtype=float)
    if not base_is_valid(current):
        current = np.array([X_MIN, CYLINDER_CENTER[1], CYLINDER_CENTER[2], 0., 0., 0.], dtype=float)

    current_res  = solve_ik_for_base(current, target, R_target)
    best_res     = current_res
    current_cost = current_res["pos_err"] + 0.3 * current_res["ori_err"] - 0.01 * current_res["manip"]
    T_ann        = 0.08

    for _ in range(SA_STEPS):
        step     = np.array([0.03, 0.03, 0.03, 0.08, 0.08, 0.15]) * np.sqrt(T_ann)
        proposal = current + np.random.normal(size=6) * step
        proposal[0] = np.clip(proposal[0], X_MIN, X_MAX)
        proposal[1] = np.clip(proposal[1], Y_MIN, Y_MAX)
        proposal[2] = np.clip(proposal[2], Z_MIN, Z_MAX)
        proposal[3] = np.clip(proposal[3], ROLL_MIN,  ROLL_MAX)
        proposal[4] = np.clip(proposal[4], PITCH_MIN, PITCH_MAX)
        proposal[5] = np.clip(proposal[5], YAW_MIN,   YAW_MAX)

        if not base_is_valid(proposal):
            T_ann *= 0.95
            continue

        res  = solve_ik_for_base(proposal, target, R_target)
        cost = res["pos_err"] + 0.3 * res["ori_err"] - 0.01 * res["manip"]

        accept = cost < current_cost
        if not accept:
            p = np.exp(-(cost - current_cost) / max(T_ann, 1e-6))
            if np.random.rand() < p:
                accept = True

        if accept:
            current      = proposal.copy()
            current_res  = res
            current_cost = cost

        if candidate_rank(res) < candidate_rank(best_res):
            best_res = res

        T_ann *= 0.95

    return best_res


# Recherche base bras pour TARGET_START ---------

print("=== Recherche base du bras pour pos début ===")
R_target_start = tcp_rotation_tangent(0.0)

start_base_1 = np.array([
    CYLINDER_CENTER[0] - 0.10,
    CYLINDER_CENTER[1],
    CYLINDER_CENTER[2] + 0.10,
    0.0, 0.0, 0.0
], dtype=float)

best_arm_res      = run_sa_for_orientation(TARGET_START, R_target_start, start_base_1)
best_arm_base_pose = best_arm_res["base_pose"]

print("best_arm_base_pose =", np.round(best_arm_base_pose, 6))
print("pos_err =", best_arm_res["pos_err"])
print("ori_err =", best_arm_res["ori_err"])
print("manip   =", best_arm_res["manip"])

x, y, z, roll, pitch, yaw = best_arm_base_pose
T_world_arm_goal = np.eye(4)
T_world_arm_goal[:3, 3]  = [x, y, z]
T_world_arm_goal[:3, :3] = rpy_to_R(roll, pitch, yaw)

T_world_spot_goal        = T_world_arm_goal @ T_arm_mount_inv
spot_goal_xyz            = T_world_spot_goal[:3, 3].copy()
spot_goal_xyz_retreat    = spot_goal_xyz.copy()
spot_goal_xyz_retreat[0] += RETREAT_DX
spot_goal_roll           = roll
spot_goal_pitch          = pitch
spot_goal_yaw            = yaw

print("spot_goal_xyz =", np.round(spot_goal_xyz, 6))
print("spot_goal_rpy =", np.round([spot_goal_roll, spot_goal_pitch, spot_goal_yaw], 6))

print("\n=== Recherche base du bras pour TARGET_BACK (partie 2) ===")
R_target_back = tcp_rotation_tangent(np.pi)


# INITIALISATION + SOLVER spot ----------------------------

spot_posture = {
    "fl.hx": 0.0, "fl.hy": 0.85, "fl.kn": -1.55,
    "fr.hx": 0.0, "fr.hy": 0.85, "fr.kn": -1.55,
    "hl.hx": 0.0, "hl.hy": 0.85, "hl.kn": -1.55,
    "hr.hx": 0.0, "hr.hy": 0.85, "hr.kn": -1.55,
}

robot = placo.RobotWrapper(SPOT_URDF, placo.Flags.ignore_collisions)
robot.update_kinematics()

for joint, value in spot_posture.items():
    robot.set_joint(joint, value)
robot.update_kinematics()

T_world_base  = robot.get_T_world_frame(BASE_FRAME).copy()
T_world_foot  = robot.get_T_world_frame("hl.foot")
T_world_base[2, 3] -= T_world_foot[2, 3] - 0.05
robot.set_T_world_frame(BASE_FRAME, T_world_base)
robot.update_kinematics()

solver = placo.KinematicsSolver(robot)
solver.dt = dt
solver.mask_fbase(False)
solver.enable_joint_limits(True)
solver.enable_velocity_limits(True)
solver.add_regularization_task(1e-4)

T_world_base_0 = robot.get_T_world_frame(BASE_FRAME).copy()
Z_BASE_INIT    = T_world_base_0[2, 3]

foot_targets_world = {foot: robot.get_T_world_frame(foot)[:3, 3].copy() for foot in FEET}

base_ori_task = solver.add_orientation_task(BASE_FRAME, T_world_base_0[:3, :3].copy())
base_ori_task.configure("base_ori_task", "hard", 1.0)

base_pos_task = solver.add_position_task(BASE_FRAME, T_world_base_0[:3, 3].copy())
base_pos_task.configure("base_pos_task", "hard", 1.0)
base_pos_task.mask.set_axises("yz")

foot_tasks = {}
for foot in FEET:
    task = solver.add_position_task(foot, foot_targets_world[foot].copy())
    task.configure(f"{foot}_task", "hard", 1.0)
    foot_tasks[foot] = task

joints_task = solver.add_joints_task()
joints_task.configure("spot_posture", "soft", 0.5)
joints_task.set_joints(spot_posture)



# INITIALISATION + SOLVER bras ----------------------------


arm = placo.RobotWrapper(ARM_MJCF, placo.Flags.mjcf | placo.Flags.ignore_collisions)
arm.update_kinematics()

T_world_arm_init = robot.get_T_world_frame(BASE_FRAME).copy() @ T_spot_base_arm_mount
arm.set_T_world_frame(ARM_BASE_FRAME, T_world_arm_init)
arm.update_kinematics()

arm_solver = placo.KinematicsSolver(arm)
arm_solver.dt = dt
arm_solver.mask_fbase(False)
arm_solver.enable_joint_limits(True)
arm_solver.enable_velocity_limits(True)
arm_solver.add_regularization_task(1e-4)

arm_joint_names_ik = arm.joint_names(False)
arm_q_rest         = arm.state.q.copy()
arm_q_rest[7:]     = 0.0

arm_base_pos_task = arm_solver.add_position_task(ARM_BASE_FRAME, T_world_arm_init[:3, 3].copy())
arm_base_pos_task.configure("arm_base_pos", "hard", 1.0)

arm_base_ori_task = arm_solver.add_orientation_task(ARM_BASE_FRAME, T_world_arm_init[:3, :3].copy())
arm_base_ori_task.configure("arm_base_ori", "hard", 1.0)

arm_rest_task = arm_solver.add_joints_task()
arm_rest_task.configure("arm_rest", "soft", 5.0)
arm_rest_joints = {name: arm_q_rest[7 + i] for i, name in enumerate(arm_joint_names_ik)}
arm_rest_task.set_joints(arm_rest_joints)

arm_tcp_pos_task = arm_solver.add_position_task(ARM_TCP, arm.get_T_world_frame(ARM_TCP)[:3, 3].copy())
arm_tcp_pos_task.configure("arm_tcp_pos", "soft", 0.0)

arm_tcp_ori_task = arm_solver.add_orientation_task(ARM_TCP, arm.get_T_world_frame(ARM_TCP)[:3, :3].copy())
arm_tcp_ori_task.configure("arm_tcp_ori", "soft", 0.0)



# # FONCTIONS DU BRAS -------


tcp_locked_pos = None   # position monde à maintenir pendant mvm du spot
tcp_locked_R   = None   # matrice de rotation monde à maintenir pendant mvm du spot

#fonction pour prendre les valeurs actuelles du tcp pour le bloquer lors du mvmt du spot
def _save_tcp_lock():

    global tcp_locked_pos, tcp_locked_R
    T_tcp          = arm.get_T_world_frame(ARM_TCP).copy()
    tcp_locked_pos = T_tcp[:3, 3].copy()
    tcp_locked_R   = T_tcp[:3, :3].copy()


# Fonction qui permet au bras de suivre le spot constamment
def update_arm_base_task():

    T_world_spot = robot.get_T_world_frame(BASE_FRAME).copy()
    T_world_arm  = T_world_spot @ T_spot_base_arm_mount

    arm.set_T_world_frame(ARM_BASE_FRAME, T_world_arm)
    arm.update_kinematics()

    arm_base_pos_task.target_world  = T_world_arm[:3, 3].copy()
    arm_base_ori_task.R_world_frame = T_world_arm[:3, :3].copy()

# Fonction qui permet de positionner le bras en pos repos
def update_arm_rest():
    #l'idée c'est de changer les poids des tasks

    update_arm_base_task()

    arm_rest_task.configure("arm_rest", "soft", 20.0)

    arm_tcp_pos_task.configure("arm_tcp_pos", "soft", 0.0)

    arm_tcp_ori_task.configure("arm_tcp_ori", "soft", 0.0)

    arm_solver.solve(True)
    arm.update_kinematics()
    viz_arm.display(arm.state.q)

# Fonction qui permet de positionner le tcp à un target donné avec R_target donné(utiliser lors du mvmt du spot pour fixer le tcp du bras)
def update_arm_hold():
    #l'idée c'est de changer les poids des tasks

    update_arm_base_task()

    arm_rest_task.configure("arm_rest", "soft", 0.0)


    arm_tcp_pos_task.configure("arm_tcp_pos", "soft", 150.0)
    arm_tcp_pos_task.target_world = tcp_locked_pos


    arm_tcp_ori_task.configure("arm_tcp_ori", "soft", 5.0)
    arm_tcp_ori_task.mask.set_axises("xyz", "local") 
    arm_tcp_ori_task.R_world_frame = tcp_locked_R

    arm_solver.solve(True)
    arm.update_kinematics()
    viz_arm.display(arm.state.q)

# Fonction qui permet de positionner le tcp à un target donné avec R_target donné
def update_arm_reach(target, R_target):
    #l'idée c'est de changer les poids des tasks

    update_arm_base_task()

    arm_rest_task.configure("arm_rest", "soft", 0.0)

    arm_tcp_pos_task.configure("arm_tcp_pos", "soft", 150.0)
    arm_tcp_pos_task.target_world = target

    arm_tcp_ori_task.configure("arm_tcp_ori", "soft", 5.0)
    arm_tcp_ori_task.mask.set_axises("xyz", "local")
    arm_tcp_ori_task.R_world_frame = R_target

    arm_solver.solve(True)
    arm.update_kinematics()
    viz_arm.display(arm.state.q)


def update_arm_reach_position_only(target):
    """
    Déplacement du TCP en position uniquement.
    Utile pour le recul droit selon Z : on évite que l'orientation bloque l'IK.
    """

    update_arm_base_task()

    arm_rest_task.configure("arm_rest", "soft", 0.0)

    arm_tcp_pos_task.configure("arm_tcp_pos", "soft", 150.0)
    arm_tcp_pos_task.target_world = target

    # Orientation désactivée pour garantir une trajectoire droite en position
    arm_tcp_ori_task.configure("arm_tcp_ori", "soft", 0.0)

    arm_solver.solve(True)
    arm.update_kinematics()
    viz_arm.display(arm.state.q)


viz     = robot_viz(robot)
viz_arm = robot_viz(arm, name="arm")
webbrowser.open(viz.viewer.url())



# Machine à états --------------


MODE_X              = 0
MODE_Y              = 1
MODE_Z              = 2
MODE_ROLL           = 3
MODE_PITCH          = 4
MODE_YAW            = 5
MODE_ARM_OPEN       = 6
MODE_ARM_TRAJ       = 7
MODE_ARM_BACK_Z = 8

mode             = MODE_X
phase_time       = 0.0
phase_index      = 0
done             = False
transition_time  = 0.0
traj_time        = 0.0
traj_time_2      = 0.0
traj_time_3      = 0.0
arm_open_time = 0.0

BACK_Z_DISTANCE = 0.05
BACK_Z_DURATION = 4.0
back_z_time = 0.0
back_z_start = None
back_z_target = None
back_z_R = None

retreat_roll_start  = 0.0
retreat_pitch_start = 0.0
retreat_yaw_start   = 0.0

trot_order   = [["fl.foot", "hr.foot"], ["fr.foot", "hl.foot"]]
swing_feet   = []
swing_start  = {}
swing_target = {}


#cette fonction me permet 1) de sélectionner les pieds à utiliser pour la phase concerner, 2) donner la position x ou y a atteindre (pour les deux pieds uniquement) et 3) envoyer un done si la position est atteinte 
def prepare_next_phase(axis, goal_xyz):
    global phase_time, phase_index, done
    global swing_feet, swing_start, swing_target

    phase_time  = 0.0
    swing_feet  = trot_order[phase_index % len(trot_order)]
    swing_start = {}
    swing_target= {}

    base_pos = robot.get_T_world_frame(BASE_FRAME)[:3, 3]

    for foot in swing_feet:
        swing_start[foot]  = foot_targets_world[foot].copy()
        swing_target[foot] = foot_targets_world[foot].copy()

        if axis == "x":
            remaining = goal_xyz[0] - base_pos[0]
            step = np.sign(remaining) * min(X_SWING, abs(remaining))
            if abs(remaining) <= 0.01:
                done       = True
                swing_feet = []
                return
            swing_target[foot][0] += step

        elif axis == "y":
            remaining = goal_xyz[1] - base_pos[1]
            step = np.sign(remaining) * min(X_SWING, abs(remaining))
            if abs(remaining) <= 0.01:
                done       = True
                swing_feet = []
                return
            swing_target[foot][1] += step

    phase_index += 1


prepare_next_phase("x", spot_goal_xyz)


# ============================================================
# BOUCLE PRINCIPALE
# ============================================================

@schedule(interval=dt)
def loop():
    global phase_time, done, mode, transition_time
    global traj_time, traj_time_2, traj_time_3, arm_open_time
    global phase_index, swing_feet
    global retreat_roll_start, retreat_pitch_start, retreat_yaw_start
    global tcp_locked_pos, tcp_locked_R, arm_open_time
    global back_z_time, back_z_start, back_z_target, back_z_R

    # ----------------------------------------------------------
    # MODE_X
    # ----------------------------------------------------------
    if mode == MODE_X:
        if not done:
            phase_time += dt
            tau = min(phase_time, T_TROT)

            for foot in FEET:
                if foot in swing_feet:
                    x0, xf = swing_start[foot][0], swing_target[foot][0]
                    xz = eqt_trot(x0, xf, tau, T_TROT, H)
                    p  = swing_start[foot].copy()
                    p[0] = xz[0]
                    p[2] = swing_start[foot][2] + xz[1]
                    foot_tasks[foot].target_world = p
                else:
                    foot_tasks[foot].target_world = foot_targets_world[foot]

            if phase_time >= T_TROT:
                for foot in swing_feet:
                    foot_targets_world[foot] = swing_target[foot].copy()
                prepare_next_phase("x", spot_goal_xyz)

            solver.solve(True)
            robot.update_kinematics()

        else:
            for foot in FEET:
                foot_tasks[foot].target_world = foot_targets_world[foot]

            cur    = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()
            cur[0] = spot_goal_xyz[0]
            base_pos_task.target_world = cur
            base_pos_task.mask.set_axises("xyz")
            solver.solve(True)
            robot.update_kinematics()

            print(f"[X done] X = {robot.get_T_world_frame(BASE_FRAME)[0,3]:.4f}")

            base_pos_task.mask.set_axises("xz")
            done        = False
            phase_time  = 0.0
            phase_index = 0
            mode        = MODE_Y
            prepare_next_phase("y", spot_goal_xyz)

    # ----------------------------------------------------------
    # MODE_Y
    # ----------------------------------------------------------
    elif mode == MODE_Y:
        if not done:
            phase_time += dt
            tau = min(phase_time, T_TROT)

            for foot in FEET:
                if foot in swing_feet:
                    y0, yf = swing_start[foot][1], swing_target[foot][1]
                    yz = eqt_trot(y0, yf, tau, T_TROT, H)
                    p  = swing_start[foot].copy()
                    p[1] = yz[0]
                    p[2] = swing_start[foot][2] + yz[1]
                    foot_tasks[foot].target_world = p
                else:
                    foot_tasks[foot].target_world = foot_targets_world[foot]

            if phase_time >= T_TROT:
                for foot in swing_feet:
                    foot_targets_world[foot] = swing_target[foot].copy()
                prepare_next_phase("y", spot_goal_xyz)

            solver.solve(True)
            robot.update_kinematics()

        else:
            for foot in FEET:
                foot_tasks[foot].target_world = foot_targets_world[foot]

            cur    = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()
            cur[1] = spot_goal_xyz[1]
            base_pos_task.target_world = cur
            base_pos_task.mask.set_axises("xyz")
            solver.solve(True)
            robot.update_kinematics()

            print(f"[Y done] Y = {robot.get_T_world_frame(BASE_FRAME)[1,3]:.4f}")

            transition_time = 0.0
            done = False
            mode = MODE_Z

    # ----------------------------------------------------------
    # MODE_Z
    # ----------------------------------------------------------
    elif mode == MODE_Z:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

        transition_time += dt
        s        = smooth(transition_time / Z_TRANSITION_DURATION)
        cur      = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()
        target_z = Z_BASE_INIT + (spot_goal_xyz[2] - Z_BASE_INIT) * s

        base_pos_task.mask.set_axises("xyz")
        base_pos_task.target_world = np.array([cur[0], cur[1], target_z])

        solver.solve(True)
        robot.update_kinematics()

        if transition_time >= Z_TRANSITION_DURATION:
            print(f"[Z done] Z = {robot.get_T_world_frame(BASE_FRAME)[2,3]:.4f}")
            transition_time = 0.0
            mode = MODE_ROLL

    # ----------------------------------------------------------
    # MODE_ROLL
    # ----------------------------------------------------------
    elif mode == MODE_ROLL:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

        transition_time += dt
        s = smooth(transition_time / ROTATION_DURATION_RPY)

        base_ori_task.R_world_frame = rot_z(0.0) @ rot_y(0.0) @ rot_x(spot_goal_roll * s)
        base_pos_task.mask.set_axises("xyz")
        base_pos_task.target_world = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()

        solver.solve(True)
        robot.update_kinematics()

        if transition_time >= ROTATION_DURATION_RPY:
            print("[ROLL done]")
            transition_time = 0.0
            mode = MODE_PITCH

    # ----------------------------------------------------------
    # MODE_PITCH
    # ----------------------------------------------------------
    elif mode == MODE_PITCH:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

        transition_time += dt
        s = smooth(transition_time / ROTATION_DURATION_RPY)

        base_ori_task.R_world_frame = (
            rot_z(0.0) @
            rot_y(spot_goal_pitch * s) @
            rot_x(spot_goal_roll)
        )
        base_pos_task.mask.set_axises("xyz")
        base_pos_task.target_world = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()

        solver.solve(True)
        robot.update_kinematics()

        if transition_time >= ROTATION_DURATION_RPY:
            print("[PITCH done]")
            transition_time = 0.0
            mode = MODE_YAW

    # ----------------------------------------------------------
    # MODE_YAW
    # ----------------------------------------------------------
    elif mode == MODE_YAW:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

        transition_time += dt
        s = smooth(transition_time / ROTATION_DURATION_RPY)

        base_ori_task.R_world_frame = (rot_z(spot_goal_yaw * s) @rot_y(spot_goal_pitch) @rot_x(spot_goal_roll))
        base_pos_task.mask.set_axises("xyz")
        base_pos_task.target_world = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()

        solver.solve(True)
        robot.update_kinematics()

        if transition_time >= ROTATION_DURATION_RPY:
            print("[YAW done]")
            arm_open_time = 0.0
            _save_tcp_lock()
            mode = MODE_ARM_OPEN

    # ----------------------------------------------------------
    # MODE_ARM_TRAJ — partie 1 : theta 0 → pi/3
    # ----------------------------------------------------------
    elif mode == MODE_ARM_OPEN:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

        solver.solve(True)
        robot.update_kinematics()

        arm_open_time += dt

        # Garder le TCP exactement fixe
        update_arm_hold()

        # Ouvrir seulement la pince
        open_gripper()

        if arm_open_time >= ARM_OPEN_DURATION:
            print("[ARM_OPEN done]")
            tcp_locked_pos = None
            tcp_locked_R = None
            traj_time = 0.0
            mode = MODE_ARM_TRAJ

    elif mode == MODE_ARM_TRAJ:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

        solver.solve(True)
        robot.update_kinematics()

        traj_time += dt
        s     = smooth(traj_time / TRAJ_DURATION)
        theta = (np.pi / 3.0) * s

        target_now   = circle_target(theta, CYLINDER_CENTER, CYLINDER_RADIUS)
        R_target_now = tcp_rotation_tangent(theta, direction=-1.0)
        update_arm_reach(target_now, R_target_now)

        if traj_time >= TRAJ_DURATION:
            print("[ARM_TRAJ_1 done]")

            T_tcp = arm.get_T_world_frame(ARM_TCP).copy()
            back_z_start = T_tcp[:3, 3].copy()

            z_tcp_world = T_tcp[:3, 2].copy()
            z_tcp_world /= np.linalg.norm(z_tcp_world)

            back_z_target = back_z_start - BACK_Z_DISTANCE * z_tcp_world

            back_z_time = 0.0
            mode = MODE_ARM_BACK_Z

    elif mode == MODE_ARM_BACK_Z:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

        solver.solve(True)
        robot.update_kinematics()

        back_z_time += dt
        s = smooth(back_z_time / BACK_Z_DURATION)

        # Trajectoire droite entre deux points
        target_now = back_z_start + s * (back_z_target - back_z_start)

        # Position seule : l'orientation est désactivée pour éviter de bloquer l'IK
        update_arm_reach_position_only(target_now)

        if back_z_time >= BACK_Z_DURATION:
            print("[ARM_BACK_Z done]")
            # On bloque la pose finale pour éviter de relancer le mouvement en boucle
            _save_tcp_lock()


    # Gestion des bras en dehors des modes de mouvements -------
    
    if mode in (MODE_ARM_OPEN, MODE_ARM_TRAJ, MODE_ARM_BACK_Z):
        pass  
    else:
        if tcp_locked_pos is not None and tcp_locked_R is not None:
            update_arm_hold()    
        else:
            update_arm_rest()


    # Visualisation ---------------------


    viz.viewer["my_part"].set_object(
        g.StlMeshGeometry.from_file("/home/mtouri/Downloads/vanne.stl")
    )
    viz.viewer["my_part"].set_transform(
        tfm.concatenate_matrices(
            tfm.translation_matrix(CYLINDER_CENTER+ np.array([-0.01, 0.0, 0.0])),
            tfm.rotation_matrix(np.pi / 2, [0, 1, 0])
        )
    )

    viz.display(robot.state.q)
    robot_frame_viz(arm, ARM_TCP)

    frame_viz("spot_goal", tf.translation_matrix(spot_goal_xyz.tolist()))
    point_viz("arm_base_goal", best_arm_base_pose[:3], radius=0.01, color=0xFF00FF)
    point_viz("spot_goal_xyz", spot_goal_xyz,          radius=0.01, color=0x00FFFF)


run_loop()
print("fin")