#!/usr/bin/env python3

import argparse
import subprocess
from pathlib import Path


SPOT_SIM_PATH = "/home/mtouri/Desktop/Maha_folder/trouver_pos_simu.py"
SPOT_SIM_PATH_SS= "/home/mtouri/Desktop/Maha_folder/trouver_pos_sans_simu.py"
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

def make_spot_and_arm_step_1(step_name, frame):

    return f"""

echo
echo "=================================================="
echo "=== ÉTAPE {step_name} : DÉPLACEMENT SPOT ==="
echo "=================================================="

conda deactivate 2>/dev/null || true
clean_ros_env

source /opt/ros/humble/setup.zsh
source {SPOT_WS}/install/setup.zsh


ros2 run spot_examples relative_move_then_pose \
    --dx 6.0\
    --dy 0.0 \
    --dyaw 0.0 \
    --dz 0.0 \
    --roll 0.0 \
    --pitch 0.0 \
    --yaw 0.0 \
    --linear-speed 0.65 \
    --frame {frame}

echo "Spot devant la porte."
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
python3 placo_to_ros_node.py --Xbase 0.0 --Ybase 0.0 --Zbase 0.0 --Xtarget 0.0  --Ytarget -0.5 --Ztarget 0.25  --roll 0.0 --pitch 0.0 --yaw 0.0

echo "Bras déplié pour ouvrir la porte"
sleep 1

echo
echo "=================================================="
echo "=== ÉTAPE {step_name} : ROTATION SPOT + RETOUR BRAS ==="
echo "=================================================="

#
# 1. On lance la rotation du Spot
#

conda deactivate 2>/dev/null || true
clean_ros_env

source /opt/ros/humble/setup.zsh
source {SPOT_WS}/install/setup.zsh

echo "Lancement rotation Spot..."

ros2 run spot_examples relative_move_then_pose \
    --dx 0.0 \
    --dy 0.0 \
    --dyaw -90.0 \
    --dz 0.0 \
    --roll 0.0 \
    --pitch 0.0 \
    --yaw 0.0 \
    --angular-speed 12.0 \
    --frame {frame} &

SPOT_ROT_PID=$!

echo "Rotation Spot lancée PID=$SPOT_ROT_PID"


# ==============================================================
# PREPARATION ENVIRONNEMENT BRAS
# ==============================================================

clean_ros_env

source ~/miniconda3/etc/profile.d/conda.sh
conda activate placo_env

source /opt/ros/humble/setup.zsh
source {ARM_WS_SETUP}

cd {ARM_SCRIPT_DIR}


# ==============================================================
# LANCEMENT RETOUR BRAS
# ==============================================================

echo "Lancement retour bras..."

python3 placo_to_ros_node.py \
    --Xbase 0.0 \
    --Ybase 0.0 \
    --Zbase 0.0 \
    --Xtarget 0.5 \
    --Ytarget 0.001 \
    --Ztarget 0.3 \
    --roll 0.0 \
    --pitch 0.0 \
    --yaw 0.0 &

ARM_PID=$!

echo "Retour bras lancé PID=$ARM_PID"


# ==============================================================
# LES DEUX COMMANDES TOURNENT MAINTENANT EN PARALLELE
# ==============================================================

echo
echo "Spot + bras en mouvement simultanément..."
echo


# ==============================================================
# ATTENDRE LA FIN DES DEUX
# ==============================================================

wait $SPOT_ROT_PID

echo "Rotation Spot terminée."

wait $ARM_PID

echo "Retour bras terminé."

echo
echo "Rotation Spot + retour bras terminés."

echo
echo "====================================================================="
echo "=== ÉTAPE {step_name} : AVANCE SPOT DANS LA CUISINE + BRAS POSE S ==="
echo "====================================================================="

#
# 1. On lance la rotation du Spot
#

conda deactivate 2>/dev/null || true
clean_ros_env

source /opt/ros/humble/setup.zsh
source {SPOT_WS}/install/setup.zsh

echo "Lancement rotation Spot..."

ros2 run spot_examples relative_move_then_pose \
    --dx 4.0 \
    --dy 0.0 \
    --dyaw 0.0 \
    --dz 0.0 \
    --roll 0.0 \
    --pitch 0.0 \
    --yaw 0.0 \
    --linear-speed 0.35 \
    --frame {frame} &

SPOT_ROT_PID=$!

echo "Rotation Spot lancée PID=$SPOT_ROT_PID"


# ==============================================================
# PREPARATION ENVIRONNEMENT BRAS
# ==============================================================

clean_ros_env

source ~/miniconda3/etc/profile.d/conda.sh
conda activate placo_env

source /opt/ros/humble/setup.zsh
source {ARM_WS_SETUP}

cd {ARM_SCRIPT_DIR}


# ==============================================================
# LANCEMENT BRAS POSE S
# ==============================================================

echo "Lancement bras pose S..."

python3 Snake_pose.py &

ARM_PID=$!

echo "Retour bras S PID=$ARM_PID"


# ==============================================================
# LES DEUX COMMANDES TOURNENT MAINTENANT EN PARALLELE
# ==============================================================

echo
echo "Spot + bras en mouvement simultanément..."
echo


# ==============================================================
# ATTENDRE LA FIN DES DEUX
# ==============================================================

wait $SPOT_ROT_PID
wait $ARM_PID

echo
echo "Spot Entre dans la piece+ retour bras terminés."

echo
echo "================================================================="
echo "=== ÉTAPE {step_name} : ROTATION SPOT vers cafe ==="
echo "================================================================"

conda deactivate 2>/dev/null || true
clean_ros_env

source /opt/ros/humble/setup.zsh
source {SPOT_WS}/install/setup.zsh


ros2 run spot_examples relative_move_then_pose \
    --dx 0.0\
    --dy 0.0 \
    --dyaw 90.0 \
    --dz 0.0 \
    --roll 0.0 \
    --pitch 0.0 \
    --yaw 0.0 \
    --angular-speed 12.0 \
    --frame {frame}

echo "Spot devant la porte."

sleep 1

echo
echo "================================================================="
echo "=== ÉTAPE {step_name} : DÉPLACEMENT SPOT vers cafe ==="
echo "================================================================"

conda deactivate 2>/dev/null || true
clean_ros_env

source /opt/ros/humble/setup.zsh
source {SPOT_WS}/install/setup.zsh


ros2 run spot_examples relative_move_then_pose \
    --dx 3.0\
    --dy 0.0 \
    --dyaw 0.0 \
    --dz 0.0 \
    --roll 0.0 \
    --pitch 0.0 \
    --yaw 0.0 \
    --linear-speed 0.5 \
    --frame {frame}

echo "Spot devant la porte."

sleep 1

"""


def main():
    parser = argparse.ArgumentParser()

    parser.add_argument("--frame", default="odom")

    args = parser.parse_args()



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

echo "Attente initialisation complète du driver Spot..."
sleep 10
{step_1_script}


echo
echo "Mission complète terminée"
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
