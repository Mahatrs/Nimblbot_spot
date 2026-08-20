#!/usr/bin/env python3

import argparse
import subprocess
from pathlib import Path


SPOT_SIM_PATH = "/home/mtouri/Desktop/Maha_folder/trouver_pos_simu_vanne.py"
JSON_OUT = "/tmp/spot_best_pose.json"

HOSTNAME = "192.168.80.3"
SPOT_WS = "/home/mtouri/spot_ws"

ESTOP_DIR = "/home/mtouri/Desktop/Maha_folder/spot-sdk/python/examples/estop"

ARM_SCRIPT_DIR = "/home/mtouri/ros2_ws_humble/src/whole-body-spot-nb/whole_body_scripts/whole_body_scripts"
ARM_WS_SETUP = "/home/mtouri/ros2_ws/install/setup.zsh"

TMP_DIR = Path("/tmp/robot_terminator")
TERMINATOR_CONFIG = TMP_DIR / "terminator_config"


def make_script(name, command, cwd=None):
    TMP_DIR.mkdir(parents=True, exist_ok=True)

    script_path = TMP_DIR / f"{name}.sh"
    cd_line = f"cd {cwd}" if cwd else ""

    content = f"""#!/usr/bin/env zsh
set +e

trap 'echo ""; echo "Ctrl+C détecté - terminal conservé"; exec zsh' INT

echo "=== {name.upper()} ==="

{cd_line}

{command}

echo
echo "=== Commande terminée : {name} ==="
echo "Appuie sur Entrée pour garder ce terminal ouvert."
read
exec zsh
"""

    script_path.write_text(content)
    script_path.chmod(0o755)

    return str(script_path)


def create_terminator_layout(scripts):
    config = """
[global_config]
  suppress_multiple_term_dialog = True

[keybindings]
  hide_window =

[profiles]
  [[default]]
    use_system_font = False
    font = Monospace 11
"""

    for profile_name, script_path in scripts.items():
        config += f"""
  [[{profile_name}]]
    use_system_font = False
    font = Monospace 11
    use_custom_command = True
    custom_command = /usr/bin/zsh "{script_path}"
"""

    config += """
[layouts]
  [[robot]]

    [[[window0]]]
      type = Window
      parent = ""
      title = Spot Mission
      size = 1600, 900

    [[[main_split]]]
      type = HPaned
      parent = window0
      order = 0
      position = 530

    [[[right_split]]]
      type = VPaned
      parent = main_split
      order = 1
      position = 450

    [[[term_estop]]]
      type = Terminal
      parent = main_split
      order = 0
      profile = estop
      title = E-STOP

    [[[term_spot_driver]]]
      type = Terminal
      parent = right_split
      order = 0
      profile = spot_driver
      title = SPOT DRIVER

    [[[term_spot_move]]]
      type = Terminal
      parent = right_split
      order = 1
      profile = spot_move
      title = SPOT MOVE + ARM

[plugins]
"""

    TERMINATOR_CONFIG.write_text(config)


def launch_terminator():
    subprocess.Popen([
        "terminator",
        "--no-dbus",
        "-g", str(TERMINATOR_CONFIG),
        "-l", "robot"
    ])

def make_spot_and_arm_step_1(step_name, xtarget, ytarget, ztarget, frame):

    return f"""
echo
echo "=================================================="
echo "=== ÉTAPE {step_name} : SIMULATION TARGET ==="
echo "=================================================="

source ~/miniconda3/etc/profile.d/conda.sh
conda activate placo_env

python3 {SPOT_SIM_PATH} \\
    --Xtarget {xtarget} \\
    --Ytarget {ytarget} \\
    --Ztarget {ztarget} \\
    --output_json {JSON_OUT}

if [ ! -f "{JSON_OUT}" ]; then
    echo "ERREUR : fichier JSON introuvable : {JSON_OUT}"
    exit 1
fi

DX=$(python3 -c 'import json; print(json.load(open("{JSON_OUT}"))["real_spot_cmd"]["dx"])')
DY=$(python3 -c 'import json; print(json.load(open("{JSON_OUT}"))["real_spot_cmd"]["dy"])')
DYAW=$(python3 -c 'import json; print(json.load(open("{JSON_OUT}"))["real_spot_cmd"]["dyaw"])')
DZ=$(python3 -c 'import json; print(json.load(open("{JSON_OUT}"))["real_spot_cmd"]["dz"])')
ROLL=$(python3 -c 'import json; print(json.load(open("{JSON_OUT}"))["real_spot_cmd"]["roll"])')
PITCH=$(python3 -c 'import json; print(json.load(open("{JSON_OUT}"))["real_spot_cmd"]["pitch"])')
YAW=$(python3 -c 'import json; print(json.load(open("{JSON_OUT}"))["real_spot_cmd"]["yaw"])')
ARM_X=$(python3 -c 'import json; d=json.load(open("{JSON_OUT}")); print(d.get("real_arm_cmd",{{}}).get("x",""))')
ARM_Y=$(python3 -c 'import json; d=json.load(open("{JSON_OUT}")); print(d.get("real_arm_cmd",{{}}).get("y",""))')
ARM_Z=$(python3 -c 'import json; d=json.load(open("{JSON_OUT}")); print(d.get("real_arm_cmd",{{}}).get("z",""))')
ARM_ROLL=$(python3 -c 'import json; d=json.load(open("{JSON_OUT}")); print(d.get("real_arm_cmd",{{}}).get("roll",""))')
ARM_PITCH=$(python3 -c 'import json; d=json.load(open("{JSON_OUT}")); print(d.get("real_arm_cmd",{{}}).get("pitch",""))')
ARM_YAW=$(python3 -c 'import json; d=json.load(open("{JSON_OUT}")); print(d.get("real_arm_cmd",{{}}).get("yaw",""))')

echo "Commande Spot calculée :"
echo "dx=$DX dy=$DY dyaw=$DYAW dz=$DZ roll=$ROLL pitch=$PITCH yaw=$YAW"

echo "ARM_X=$ARM_X"
echo "ARM_Y=$ARM_Y"
echo "ARM_Z=$ARM_Z"
echo "ARM_ROLL=$ARM_ROLL"
echo "ARM_PITCH=$ARM_PITCH"
echo "ARM_YAW=$ARM_YAW"

if [ -z "$ARM_X" ] || [ -z "$ARM_Y" ] || [ -z "$ARM_Z" ] || [ -z "$ARM_ROLL" ] || [ -z "$ARM_PITCH" ] || [ -z "$ARM_YAW" ]; then
    echo "ERREUR : real_arm_cmd incomplet dans le JSON"
    cat "{JSON_OUT}"
    exit 1
fi


echo
echo "=================================================="
echo "=== ÉTAPE {step_name} : DÉPLACEMENT SPOT ==="
echo "=================================================="

conda deactivate 2>/dev/null || true
clean_ros_env

source /opt/ros/humble/setup.zsh
source {SPOT_WS}/install/setup.zsh


ros2 run spot_examples relative_move_then_pose \
    --dx $DX \
    --dy $DY \
    --dyaw $DYAW \
    --dz $DZ \
    --roll $ROLL \
    --pitch $PITCH \
    --yaw $YAW \
    --frame {frame}

echo "Spot déjà debout après étape {step_name}."
sleep 1

echo
echo "=================================================="
echo "=== ÉTAPE {step_name} : LANCEMENT DU BRAS ==="
echo "=================================================="


clean_ros_env
source ~/miniconda3/etc/profile.d/conda.sh
conda activate placo_env

source /opt/ros/humble/setup.zsh
source {ARM_WS_SETUP}

cd {ARM_SCRIPT_DIR}
python3 placo_to_ros_node_vanne.py --Xbase "$ARM_X" --Ybase "$ARM_Y" --Zbase "$ARM_Z" --Xtarget "{xtarget}" --Ytarget "{ytarget}" --Ztarget "{ztarget}" --roll 0.0 --pitch 0.0 --yaw 0.0

sleep 1

"""

def main():
    parser = argparse.ArgumentParser()

    parser.add_argument("--xtarget", default=2.5, type=float)
    parser.add_argument("--ytarget", default=0.0, type=float)
    parser.add_argument("--ztarget", default=0.6, type=float)


    parser.add_argument("--frame", default="odom")

    args = parser.parse_args()

    print("=== Création des scripts Spot ===")
    print("Target 1:", args.xtarget, args.ytarget, args.ztarget)


    estop_script = make_script(
        "estop",
        f"""
python3 estop_gui.py {HOSTNAME}
""",
        cwd=ESTOP_DIR
    )

    spot_driver_script = make_script(
        "spot_driver",
        f"""
export PATH=/usr/bin:/bin:/usr/sbin:/sbin
export PYTHONEXECUTABLE=/usr/bin/python3

unset PYTHONPATH
unset LD_LIBRARY_PATH

source /opt/ros/humble/setup.zsh
source {SPOT_WS}/install/setup.zsh

which python3
python3 --version

ros2 launch spot_driver spot_driver.launch.py \\
    launch_image_publishers:=False \\
    publish_point_clouds:=False \\
    launch_rviz:=True
"""
    )

    step_1_script = make_spot_and_arm_step_1(
        "1",
        args.xtarget,
        args.ytarget,
        args.ztarget,
        args.frame,
)


    spot_move_script = make_script(
        "spot_move",
        f"""
export PATH=/usr/bin:/bin:/usr/sbin:/sbin
export PYTHONEXECUTABLE=/usr/bin/python3

unset PYTHONPATH
unset LD_LIBRARY_PATH

clean_ros_env() {{
    unset ROS_DISTRO
    unset AMENT_PREFIX_PATH
    unset COLCON_PREFIX_PATH
    unset CMAKE_PREFIX_PATH
    unset PYTHONPATH
    unset LD_LIBRARY_PATH
}}

source /opt/ros/humble/setup.zsh
source {SPOT_WS}/install/setup.zsh

which python3
python3 --version

echo "Attente du service /spot/claim..."

until ros2 service list | grep -q "^/spot/claim$"; do
    echo "Service /spot/claim pas encore disponible..."
    sleep 2
done

echo "Service /spot/claim disponible."

{step_1_script}


"""
    )

    scripts = {
        "estop": estop_script,
        "spot_driver": spot_driver_script,
        "spot_move": spot_move_script,
    }

    create_terminator_layout(scripts)
    launch_terminator()

    print("Mission Spot + bras lancée.")


if __name__ == "__main__":
    main()
