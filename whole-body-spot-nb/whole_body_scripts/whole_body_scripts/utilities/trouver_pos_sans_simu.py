import placo
import numpy as np
from ischedule import schedule, run_loop
from placo_utils.visualization import robot_viz, frame_viz, point_viz
import webbrowser
from placo_utils.tf import tf
import argparse
import json

#args

parser = argparse.ArgumentParser()
   
parser.add_argument('--Xtarget', default=1.0, type=float,
                        help='Position X du TCP')
parser.add_argument('--Ytarget', default=0.0, type=float,
                        help='Position Y du TCP')
parser.add_argument('--Ztarget', default=0.5, type=float,
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

MJCF_PATH_ARM   = "/home/mtouri/Desktop/Maha_folder/robot_descriptions_maha/mjcf_description/nb-55-v7_motor.xml"
ARM_BASE_FRAME = "lower_ring_0"
ARM_TCP         = "tcp"




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

def spot_z_pitch_from_target_z(z_target):
    """
    ztarget = 0.0 -> zspot=0.4, pitch=0.5
    ztarget = 0.1 -> zspot=0.4, pitch=0.4
    ztarget = 0.2 -> zspot=0.4, pitch=0.3
    ztarget = 0.3 -> zspot=0.4, pitch=0.2
    ztarget = 0.4 -> zspot=0.4, pitch=0.1
    ztarget = 0.5 -> zspot=0.4, pitch=0.0
    ztarget > 0.5 -> zspot=0.55, pitch=0.0
    """
    if z_target < 0.5:
        z_clamped = np.clip(z_target, 0.0, 0.4)
        z_spot = 0.4
        pitch_spot = 30.0 * (1.0 - z_clamped / 0.4)
    else:
        z_spot = 0.6
        pitch_spot = 0.0

    return z_spot, pitch_spot

zf,pf = spot_z_pitch_from_target_z(target[2])


X_GOAL     = target[0]-0.6
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
    "real_arm_cmd": {
        "x": float(best_base_bras[0]),
        "y": float(best_base_bras[1]),
        "z": float(best_base_bras[2]),
        "roll": float(best_base_bras[3]),
        "pitch": float(best_base_bras[4]),
        "yaw": float(best_base_bras[5]),
    }
}
with open(XYZ.output_json, "w") as f:
     json.dump(result, f, indent=2)




print("fin")