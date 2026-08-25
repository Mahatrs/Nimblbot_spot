import placo
import numpy as np
from ischedule import schedule, run_loop
from placo_utils.visualization import robot_viz, frame_viz, point_viz
import webbrowser
from placo_utils.tf import tf
import argparse
import json
import meshcat.transformations as tfm
import meshcat.geometry as g


#args

parser = argparse.ArgumentParser()
   
parser.add_argument('--Xtarget', default=1.0, type=float,
                        help='Position X du TCP')
parser.add_argument('--Ytarget', default=0.0, type=float,
                        help='Position Y du TCP')
parser.add_argument('--Ztarget', default=0.6, type=float,
                        help='Position Z du TCP')
parser.add_argument('--output_json', default='/tmp/spot_best_pose.json', type=str,
                    help='Chemin du fichier JSON de sortie')
XYZ = parser.parse_args()
#fcts utiles

def eqt_trot(x0, xf, t, T, H):
    s = t / T
    x = (1 - s) ** 2 * x0 + 2 * (1 - s) * s * ((x0 + xf) / 2.0) + s ** 2 * xf
    z = 4.0 * H * (1 - s) * s
    return np.array([x, z])

def rot_x(angle):
    c, s = np.cos(angle), np.sin(angle)
    return np.array([[1, 0, 0], [0, c, -s], [0, s, c]])

def rot_y(angle):
    c, s = np.cos(angle), np.sin(angle)
    return np.array([[c, 0, s], [0, 1, 0], [-s, 0, c]])

def rot_z(angle):
    c, s = np.cos(angle), np.sin(angle)
    return np.array([[c, -s, 0], [s, c, 0], [0, 0, 1]])

def smooth(s):
    s = np.clip(s, 0, 1)
    return 3 * s**2 - 2 * s**3

def rpy_to_quat(roll, pitch, yaw):
    cr, sr = np.cos(roll/2), np.sin(roll/2)
    cp, sp = np.cos(pitch/2), np.sin(pitch/2)
    cy, sy = np.cos(yaw/2), np.sin(yaw/2)
    qw = cr*cp*cy + sr*sp*sy
    qx = sr*cp*cy - cr*sp*sy
    qy = cr*sp*cy + sr*cp*sy
    qz = cr*cp*sy - sr*sp*cy
    return np.array([qx, qy, qz, qw])

def rot_to_rpy(R):
    pitch = np.arctan2(-R[2, 0], np.sqrt(R[0, 0]**2 + R[1, 0]**2))
    roll = np.arctan2(R[2, 1], R[2, 2])
    yaw = np.arctan2(R[1, 0], R[0, 0])
    return roll, pitch, yaw


# paramètres

URDF_PATH   = "/home/mtouri/Desktop/Maha_folder/spot_descritpion/model.urdf"
BASE_FRAME  = "base"
FEET        = ["fl.foot", "fr.foot", "hl.foot", "hr.foot"]

MJCF_PATH_ARM  ="/home/mtouri/Desktop/Maha_folder/robot_descriptions_maha/mjcf_description/nb-55-v7_motor_gripper.xml"
ARM_BASE_FRAME = "lower_ring_0"
ARM_TCP         = "gripper_tcp"




target = np.array([XYZ.Xtarget, XYZ.Ytarget, XYZ.Ztarget])


robot_algo_frame_viz = placo.RobotWrapper(MJCF_PATH_ARM,placo.Flags.mjcf | placo.Flags.ignore_collisions)
solver_algo = placo.KinematicsSolver(robot_algo_frame_viz)
solver_algo.mask_fbase(False)
solver_algo.dt = 0.01
solver_algo.enable_joint_limits(True)
solver_algo.enable_velocity_limits(True)
solver_algo.add_regularization_task(1e-4)

T0 = robot_algo_frame_viz.get_T_world_frame(ARM_TCP).copy()
tcp_task = solver_algo.add_position_task(ARM_TCP, T0[:3, 3].copy())
tcp_task.configure(ARM_TCP, "soft", 10.0)

import numpy as np

def spot_z_pitch_from_target_z(z_target):
    """
    ztarget = 0.0 -> zspot = 0.4 , pitch = 30°
    ztarget = 0.5 -> zspot = 0.6 , pitch = 0°

    interpolation linéaire entre les deux
    """

    z_min = 0.0
    z_max = 1.0

    alpha = np.clip(
        (z_target - z_min) / (z_max - z_min),
        0.0,
        1.0
    )

    z_spot = 0.4 + alpha * (0.6 - 0.4)

    pitch_spot = 0 #(1.0 - alpha) * 30.0

    return z_spot, pitch_spot

zf,pf = spot_z_pitch_from_target_z(target[2])


X_GOAL     = target[0]-0.85
Y_GOAL     = target[1]-0.05
Z_GOAL     = zf
ROLL_GOAL  = 0
PITCH_GOAL = pf
YAW_GOAL   = 0

best_base_spot = [X_GOAL, Y_GOAL, Z_GOAL, ROLL_GOAL, PITCH_GOAL, YAW_GOAL]
best_base_bras = [X_GOAL+0.3, Y_GOAL, Z_GOAL+0.08, ROLL_GOAL, PITCH_GOAL, YAW_GOAL]

print('-----------------------------------')
print("Meilleur base pour spot =", best_base_spot)
print("Meilleur base pour bras =", best_base_bras)
print('-----------------------------------')


result = {
    "real_spot_cmd": {
        "dx": float(best_base_spot[0]),
        "dy": float(best_base_spot[1]),
        "dyaw": 0.0,
        "dz": float(best_base_spot[2] - 0.6),
        "roll": float(best_base_spot[3]),
        "pitch": float(best_base_spot[4]),
        "yaw": float(best_base_spot[5]),
     },
    "real_arm_cmd": {}
}
with open(XYZ.output_json, "w") as f:
     json.dump(result, f, indent=2)


PITCH_GOAL = np.deg2rad(PITCH_GOAL)


# Déplacement
dt = 0.01
H  = 0.08
T  = 0.4
X_SWING        = 0.2
ROTATION_DURATION     = 1.5
Z_TRANSITION_DURATION = 1.5

MODE_X = 0 
MODE_Y = 1 
MODE_Z = 2
MODE_ROLL = 3 
MODE_PITCH = 4
MODE_YAW = 5
MODE_ARM_p1 = 6
MODE_ARM_p2 = 7
MODE_ARM_p3 = 8
MODE_ARM_p4 = 9

trot_order = [["fl.foot", "hr.foot"], ["fr.foot", "hl.foot"]]

spot_posture = {
    "fl.hx": 0.0, "fl.hy": 0.85, "fl.kn": -1.55,
    "fr.hx": 0.0, "fr.hy": 0.85, "fr.kn": -1.55,
    "hl.hx": 0.0, "hl.hy": 0.85, "hl.kn": -1.55,
    "hr.hx": 0.0, "hr.hy": 0.85, "hr.kn": -1.55,
}

# Init Spot
robot = placo.RobotWrapper(URDF_PATH, placo.Flags.ignore_collisions)
robot.update_kinematics()
for joint, value in spot_posture.items():
    robot.set_joint(joint, value)
robot.update_kinematics()

T_world_base = robot.get_T_world_frame(BASE_FRAME).copy()
T_world_foot = robot.get_T_world_frame('hl.foot')
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

foot_targets_world = {}
for foot in FEET:
    foot_targets_world[foot] = robot.get_T_world_frame(foot)[:3, 3].copy()

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

# Init Bras
T_spot_base_arm_mount       = np.eye(4)
T_spot_base_arm_mount[0, 3] = 0.3
T_spot_base_arm_mount[1, 3] = 0.0
T_spot_base_arm_mount[2, 3] = 0.08


arm = placo.RobotWrapper(
   MJCF_PATH_ARM,
   placo.Flags.mjcf | placo.Flags.ignore_collisions
)
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
arm_q_rest = arm.state.q.copy()
arm_q_rest[7:] = 0.0

arm_base_pos_task = arm_solver.add_position_task(ARM_BASE_FRAME,T_world_arm_init[:3, 3].copy())
arm_base_pos_task.configure("arm_base_pos", "hard", 1.0)

arm_base_ori_task = arm_solver.add_orientation_task(ARM_BASE_FRAME,T_world_arm_init[:3, :3].copy())
arm_base_ori_task.configure("arm_base_ori", "hard", 1.0)

# maintenir les joints en position initiale
arm_rest_task = arm_solver.add_joints_task()
arm_rest_task.configure("arm_rest", "soft", 5.0)
arm_rest_joints = {name: arm_q_rest[7 + i] for i, name in enumerate(arm_joint_names_ik)}
arm_rest_task.set_joints(arm_rest_joints)


arm_tcp_task = arm_solver.add_position_task(ARM_TCP,arm.get_T_world_frame(ARM_TCP)[:3, 3].copy())
arm_tcp_task.configure("arm_tcp_task", "soft", 0.0)
arm_tcp_task.target_world = target

R_tcp_target = rot_y(np.pi / 2)

arm_ori_task = arm_solver.add_orientation_task(ARM_TCP,R_tcp_target)
arm_ori_task.configure("arm_tcp_ori", "soft", 0.0)

# axisalign_task = arm_solver.add_axisalign_task(
#     "gripper_tcp",
#     np.array([0., 0., 1.]),   # TCP z-axis
#     np.array([1., 0., 0.])    # world x-axis
# )
# axisalign_task.configure("look_at", "soft", 0.0)

#pour que la base suivent le mvm du spot
def update_arm_base_task():
    T_world_spot = robot.get_T_world_frame(BASE_FRAME).copy()
    T_world_arm  = T_world_spot @ T_spot_base_arm_mount

    arm.set_T_world_frame(ARM_BASE_FRAME, T_world_arm)
    arm.update_kinematics()

    arm_base_pos_task.target_world = T_world_arm[:3, 3].copy()
    arm_base_ori_task.R_world_frame = T_world_arm[:3, :3].copy()

#pour maintenir le bras en pos de repos tout en suivant le spot
def update_arm_rest():
    update_arm_base_task()
    arm_rest_task.configure("arm_rest", "soft", 20.0)
    arm_tcp_task.configure("arm_tcp_task", "soft", 0.0)
    arm_ori_task.configure("arm_tcp_ori", "soft", 0.0)
    arm_solver.solve(True)
    arm.update_kinematics()
    viz_arm.display(arm.state.q)

#pour atteindre la cible tout en suivant le spot
def update_arm_reach():
    update_arm_base_task()

    arm_rest_task.configure("arm_rest", "soft", 0.0)

    arm_tcp_task.configure("arm_tcp_task", "soft", 20.0)
    arm_tcp_task.target_world = target

    arm_ori_task.configure("arm_tcp_ori", "soft", 0.01)
    arm_ori_task.R_world_frame = R_tcp_target

    for _ in range(5):
        arm_solver.solve(True)
        arm.update_kinematics()

    viz_arm.display(arm.state.q)

T_tcp = arm.get_T_world_frame(ARM_TCP)
R_tcp = T_tcp[:3, :3]

print("axe X TCP dans world =", R_tcp[:, 0])
print("axe Y TCP dans world =", R_tcp[:, 1])
print("axe Z TCP dans world =", R_tcp[:, 2])
# Visualisation
viz     = robot_viz(robot)
viz_arm = robot_viz(arm, name="arm")
webbrowser.open(viz.viewer.url())

phase_time      = 0.0
phase_index     = 0
done            = False
mode            = MODE_X
transition_time = 0.0

swing_feet   = []
swing_start  = {}
swing_target = {}


def prepare_next_phase(axis):

    global phase_time, phase_index, done
    global swing_feet, swing_start, swing_target

    phase_time  = 0.0
    swing_feet  = trot_order[phase_index % len(trot_order)]
    swing_start  = {}
    swing_target = {}
    base_pos = robot.get_T_world_frame(BASE_FRAME)[:3, 3]

    for foot in swing_feet:
        swing_start[foot]  = foot_targets_world[foot].copy()
        swing_target[foot] = foot_targets_world[foot].copy()

        if axis == 'x':
            remaining = X_GOAL - base_pos[0]
            step = np.sign(remaining) * min(X_SWING, abs(remaining))
            if abs(remaining) <= 0.01:
                done = True
                swing_feet = []
                return
            swing_target[foot][0] += step

        elif axis == 'y':
            remaining = Y_GOAL - base_pos[1]
            step = np.sign(remaining) * min(X_SWING, abs(remaining))
            if abs(remaining) <= 0.01:
                done = True
                swing_feet = []
                return
            swing_target[foot][1] += step

    phase_index += 1


prepare_next_phase('x')
t = 0.0

finished = False
arm_time = 0.0
ARM_HOLD_DURATION = 2.0
arm_base_saved = False
arm_base_pose_saved = None

PUSH_DISTANCE = 0.07   # avance de 10 cm
PUSH_DURATION = 2.0

push_time = 0.0
push_start = None
push_target = None

ARC_RADIUS = 0.05
ARC_DURATION = 3.0

arc_time = 0.0
arc_center = None
arc_start_angle = 0.0

@schedule(interval=dt)
def loop():
    global t, phase_time, done, mode, transition_time
    global finished, arm_time
    global arm_base_saved, arm_base_pose_saved
    global push_time, push_start, push_target
    global arc_time, arc_center, arc_start_angle
    t += dt

    # MODE_X ------------------------------
    if mode == MODE_X:
        if not done:
            phase_time += dt
            tau = min(phase_time, T)
            for foot in FEET:
                if foot in swing_feet:
                    x0, xf = swing_start[foot][0], swing_target[foot][0]
                    xz = eqt_trot(x0, xf, tau, T, H)
                    p = swing_start[foot].copy()
                    p[0] = xz[0]
                    p[2] = swing_start[foot][2] + xz[1]
                    foot_tasks[foot].target_world = p
                else:
                    foot_tasks[foot].target_world = foot_targets_world[foot]
            if phase_time >= T:
                for foot in swing_feet:
                    foot_targets_world[foot] = swing_target[foot].copy()
                prepare_next_phase('x')
            solver.solve(True)
            robot.update_kinematics()
        else:
            for foot in FEET:
                foot_tasks[foot].target_world = foot_targets_world[foot]
            cur = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()
            cur[0] = X_GOAL
            base_pos_task.target_world = cur
            base_pos_task.mask.set_axises("xyz")
            solver.solve(True)
            robot.update_kinematics()
            print(f"[X done] X = {robot.get_T_world_frame(BASE_FRAME)[0,3]:.4f}")
            base_pos_task.mask.set_axises("xz")
            done = False
            phase_time = 0.0
            phase_index = 0
            mode = MODE_Y
            prepare_next_phase('y')

    # MODE_Y ------------------------------
    elif mode == MODE_Y:
        if not done:
            phase_time += dt
            tau = min(phase_time, T)
            for foot in FEET:
                if foot in swing_feet:
                    x0, xf = swing_start[foot][1], swing_target[foot][1]
                    xz = eqt_trot(x0, xf, tau, T, H)
                    p = swing_start[foot].copy()
                    p[1] = xz[0]
                    p[2] = swing_start[foot][2] + xz[1]
                    foot_tasks[foot].target_world = p
                else:
                    foot_tasks[foot].target_world = foot_targets_world[foot]
            if phase_time >= T:
                for foot in swing_feet:
                    foot_targets_world[foot] = swing_target[foot].copy()
                prepare_next_phase('y')
            solver.solve(True)
            robot.update_kinematics()
        else:
            for foot in FEET:
                foot_tasks[foot].target_world = foot_targets_world[foot]
            cur = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()
            cur[1] = Y_GOAL
            base_pos_task.target_world = cur
            base_pos_task.mask.set_axises("xyz")
            solver.solve(True)
            robot.update_kinematics()
            print(f"[Y done] Y = {robot.get_T_world_frame(BASE_FRAME)[1,3]:.4f}")
            transition_time = 0.0
            mode = MODE_Z
            done = False

    # MODE_Z ------------------------------
    elif mode == MODE_Z:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]
        transition_time += dt
        s = smooth(transition_time / Z_TRANSITION_DURATION)
        cur = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()
        target_z = Z_BASE_INIT + (Z_GOAL - Z_BASE_INIT) * s
        base_pos_task.mask.set_axises("xyz")
        base_pos_task.target_world = np.array([cur[0], cur[1], target_z])
        solver.solve(True)
        robot.update_kinematics()
        if transition_time >= Z_TRANSITION_DURATION:
            print(f"[Z done] Z = {robot.get_T_world_frame(BASE_FRAME)[2,3]:.4f}")
            transition_time = 0.0
            mode = MODE_ROLL

    # MODE_ROLL ------------------------------
    elif mode == MODE_ROLL:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

        transition_time += dt
        s = smooth(transition_time / ROTATION_DURATION)

        base_ori_task.R_world_frame = (
            rot_z(0.0)
            @ rot_y(0.0)
            @ rot_x(ROLL_GOAL * s)
        )

        base_pos_task.mask.set_axises("xyz")
        base_pos_task.target_world = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()

        solver.solve(True)
        robot.update_kinematics()

        if transition_time >= ROTATION_DURATION:
            print("[ROLL done]")
            transition_time = 0.0
            mode = MODE_PITCH


    # MODE_PITCH ------------------------------
    elif mode == MODE_PITCH:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

        transition_time += dt
        s = smooth(transition_time / ROTATION_DURATION)

        base_ori_task.R_world_frame = (
            rot_z(0.0)
            @ rot_y(PITCH_GOAL * s)
            @ rot_x(ROLL_GOAL)
        )

        base_pos_task.mask.set_axises("xyz")
        base_pos_task.target_world = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()

        solver.solve(True)
        robot.update_kinematics()

        if transition_time >= ROTATION_DURATION:
            print("[PITCH done]")
            transition_time = 0.0
            mode = MODE_YAW


    # MODE_YAW ------------------------------
    elif mode == MODE_YAW:
        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]

        transition_time += dt
        s = smooth(transition_time / ROTATION_DURATION)

        base_ori_task.R_world_frame = (
            rot_z(YAW_GOAL * s)
            @ rot_y(PITCH_GOAL)
            @ rot_x(ROLL_GOAL)
        )

        base_pos_task.mask.set_axises("xyz")
        base_pos_task.target_world = robot.get_T_world_frame(BASE_FRAME)[:3, 3].copy()

        solver.solve(True)
        robot.update_kinematics()

        if transition_time >= ROTATION_DURATION:
            print("[YAW done]")
            mode = MODE_ARM_p1

    # MODE_ARM ------------------------------
    elif mode == MODE_ARM_p1:
        if not arm_base_saved:
            T_arm = arm.get_T_world_frame(ARM_BASE_FRAME).copy()

            x_arm, y_arm, z_arm = T_arm[:3, 3]
            roll_arm, pitch_arm, yaw_arm = rot_to_rpy(T_arm[:3, :3])

            arm_base_pose_saved = {
                "x": float(x_arm),
                "y": float(y_arm),
                "z": float(z_arm),
                "roll": float(roll_arm),
                "pitch": float(pitch_arm),
                "yaw": float(yaw_arm),
            }

            result["real_arm_cmd"] = arm_base_pose_saved

            with open(XYZ.output_json, "w") as f:
                json.dump(result, f, indent=2)

            arm_base_saved = True

            print("Pose base bras enregistrée :", arm_base_pose_saved)

        for foot in FEET:
            foot_tasks[foot].target_world = foot_targets_world[foot]
        

        solver.solve(True)
        robot.update_kinematics()
        update_arm_reach()

        arm_time += dt
        if arm_time >= ARM_HOLD_DURATION:
            print("[ARM done]")
            push_start = arm.get_T_world_frame(ARM_TCP)[:3, 3].copy()
            push_target = push_start + np.array([PUSH_DISTANCE, 0.0, 0.0])  # avance selon X monde
            push_time = 0.0
            mode = MODE_ARM_p2
    elif mode == MODE_ARM_p2:
        update_arm_base_task()

        push_time += dt
        s = smooth(push_time / PUSH_DURATION)

        p = (1 - s) * push_start + s * push_target

        arm_rest_task.configure("arm_rest", "soft", 0.0)
        arm_tcp_task.configure("arm_tcp_task", "soft", 20.0)
        arm_tcp_task.target_world = p

        arm_ori_task.configure("arm_tcp_ori", "soft", 0.01)
        arm_ori_task.R_world_frame = R_tcp_target

        for _ in range(5):
            arm_solver.solve(True)
            arm.update_kinematics()

        viz_arm.display(arm.state.q)

        if push_time >= PUSH_DURATION:
            print("[PUSH done]")
            arc_time = 0.0
            p0 = arm.get_T_world_frame(ARM_TCP)[:3, 3].copy()

            # centre du demi-cercle dans le plan XZ
            arc_center = p0 + np.array([0.0, 0.0, -ARC_RADIUS])

            mode = MODE_ARM_p3
    elif mode == MODE_ARM_p3:
        update_arm_base_task()

        arc_time += dt
        s = smooth(arc_time / ARC_DURATION)

        theta = 2* np.pi * s   # demi-cercle

        p = arc_center + np.array([
            0.0,                         # X constant
            ARC_RADIUS * np.sin(theta),   # rotation autour axe X
            ARC_RADIUS * np.cos(theta)
        ])

        arm_rest_task.configure("arm_rest", "soft", 0.0)
        arm_tcp_task.configure("arm_tcp_task", "soft", 20.0)
        arm_tcp_task.target_world = p

        # garde la même orientation
        arm_ori_task.configure("arm_tcp_ori", "soft", 0.01)
        arm_ori_task.R_world_frame = R_tcp_target

        for _ in range(5):
            arm_solver.solve(True)
            arm.update_kinematics()

        viz_arm.display(arm.state.q)

        if arc_time >= ARC_DURATION:
            print("[ARC done]")
            push_start = arm.get_T_world_frame(ARM_TCP)[:3, 3].copy()
            push_target = push_start + np.array([-PUSH_DISTANCE, 0.0, 0.0])  # avance selon X monde
            push_time = 0.0
            mode = MODE_ARM_p4
    
    elif mode == MODE_ARM_p4:
        update_arm_base_task()

        push_time += dt
        s = smooth(push_time / PUSH_DURATION)

        p = (1 - s) * push_start + s * push_target

        arm_rest_task.configure("arm_rest", "soft", 0.0)
        arm_tcp_task.configure("arm_tcp_task", "soft", 20.0)
        arm_tcp_task.target_world = p

        arm_ori_task.configure("arm_tcp_ori", "soft", 0.01)
        arm_ori_task.R_world_frame = R_tcp_target

        for _ in range(5):
            arm_solver.solve(True)
            arm.update_kinematics()

        viz_arm.display(arm.state.q)

        if push_time >= PUSH_DURATION:
            print("[OUT done]")
            finished = True


    viz.viewer["my_part"].set_object(
        g.StlMeshGeometry.from_file("/home/mtouri/Downloads/vanne.stl")
    )
    viz.viewer["my_part"].set_transform(
        tfm.concatenate_matrices(
            tfm.translation_matrix(target+np.array([0.07, 0, -0.06])),
            tfm.rotation_matrix(np.pi / 2, [0, 1, 0])
        )
    )

    viz.display(robot.state.q)
    frame_viz("target", tf.translation_matrix([X_GOAL, Y_GOAL, Z_GOAL]))
    frame_viz("gripper_tcp",arm.get_T_world_frame(ARM_TCP))

    if mode not in [MODE_ARM_p1, MODE_ARM_p2, MODE_ARM_p3, MODE_ARM_p4]:
        update_arm_rest()

    point_viz("target_spot", best_base_spot, radius=0.03, color=0x0000FF)
    #point_viz("target_base", best_base_bras, radius=0.03, color=0xFF0000)
    point_viz("target", target, radius=0.01, color=0x0000FF)

    if finished:
        raise SystemExit
    
run_loop()
print("fin")