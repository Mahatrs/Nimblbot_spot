#!/usr/bin/env python3

import argparse
import subprocess
import json

SPOT_SIM_PATH = "/home/mtouri/Desktop/Maha_folder/trouver_pose_simu_q.py"
JSON_OUT = "/tmp/spot_best_pose.json"

ARM_SCRIPT_DIR = "/home/mtouri/ros2_ws_humble/src/whole-body-spot-nb/whole_body_scripts/whole_body_scripts"
ARM_WS_SETUP = "/home/mtouri/ros2_ws/install/setup.zsh"

parser = argparse.ArgumentParser()

parser.add_argument("--xtarget", type=float, required=True)
parser.add_argument("--ytarget", type=float, required=True)
parser.add_argument("--ztarget", type=float, required=True)

args = parser.parse_args()

print("=== ETAPE 1 : Calcul IK ===")

subprocess.run(
    [
        "python3",
        SPOT_SIM_PATH,
        "--Xtarget", str(args.xtarget),
        "--Ytarget", str(args.ytarget),
        "--Ztarget", str(args.ztarget),
        "--output_json", JSON_OUT,
    ],
    check=True,
)

print("=== ETAPE 2 : Lecture q_joints ===")

with open(JSON_OUT) as f:
    data = json.load(f)

q_joints = data["real_arm_cmd"]["q_joints"]

print("Nombre de joints :", len(q_joints))
print("q_joints =", q_joints)

print("=== ETAPE 3 : Envoi bras ===")

q_json = json.dumps(q_joints)

shell_cmd = f"""
source /opt/ros/humble/setup.zsh
source {ARM_WS_SETUP}
cd {ARM_SCRIPT_DIR}

python3 placo_to_ros_q.py --q_json '{q_json}'
"""

subprocess.run(
    ["zsh", "-c", shell_cmd],
    check=True,
)
