#!/usr/bin/env python3

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path


SPOT_SIM_PATH = "/home/mtouri/Desktop/Maha_folder/spot_twist_bras_irl.py"
JSON_OUT = "/tmp/spot_best_pose.json"

HOSTNAME = "192.168.80.3"
SPOT_WS = "/home/mtouri/spot_ws"

ESTOP_DIR = "/home/mtouri/Desktop/Maha_folder/spot-sdk/python/examples/estop"

TMP_DIR = Path("/tmp/robot_terminator")
TERMINATOR_CONFIG = TMP_DIR / "terminator_config"


def run_simulation(args):
    subprocess.run([
        sys.executable,
        SPOT_SIM_PATH,
        "--Xtarget", str(args.xtarget),
        "--Ytarget", str(args.ytarget),
        "--Ztarget", str(args.ztarget),
        "--output_json", JSON_OUT,
    ], check=True)

    if not os.path.exists(JSON_OUT):
        raise FileNotFoundError(f"JSON not found: {JSON_OUT}")

    with open(JSON_OUT, "r") as f:
        return json.load(f)


def make_script(name, command, cwd=None):
    TMP_DIR.mkdir(parents=True, exist_ok=True)

    script_path = TMP_DIR / f"{name}.sh"
    cd_line = f"cd {cwd}" if cwd else ""

    content = f"""#!/usr/bin/env zsh
set +e

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


def main():
    parser = argparse.ArgumentParser()

    parser.add_argument("--xtarget", type=float, required=True)
    parser.add_argument("--ytarget", type=float, required=True)
    parser.add_argument("--ztarget", type=float, required=True)

    parser.add_argument("--xplace", type=float, required=True)
    parser.add_argument("--yplace", type=float, required=True)
    parser.add_argument("--zplace", type=float, required=True)

    parser.add_argument("--frame", default="odom")

    args = parser.parse_args()

    print("=== 1. Simulation ===")

    data = run_simulation(args)
    spot_cmd = data["real_spot_cmd"]

    print("Spot command:", spot_cmd)
    print("Pick target:", args.xtarget, args.ytarget, args.ztarget)
    print("Place target:", args.xplace, args.yplace, args.zplace)

    print("=== 2. Création des scripts Spot ===")

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

    spot_move_script = make_script(
        "spot_move",
        f"""
export PATH=/usr/bin:/bin:/usr/sbin:/sbin
export PYTHONEXECUTABLE=/usr/bin/python3

unset PYTHONPATH
unset LD_LIBRARY_PATH

source /opt/ros/humble/setup.zsh
source {SPOT_WS}/install/setup.zsh

which python3
python3 --version

echo "Attente du service /spot/claim..."

until ros2 service list | grep -q "^/spot/claim$"; do
    echo "Service /spot/claim pas encore disponible..."
    sleep 2
done

echo "Service /spot/claim disponible. Déplacement Spot..."

ros2 run spot_examples relative_move_then_pose \\
    --dx {spot_cmd['dx']} \\
    --dy {spot_cmd['dy']} \\
    --dyaw {spot_cmd['dyaw']} \\
    --dz {spot_cmd['dz']} \\
    --roll {spot_cmd['roll']} \\
    --pitch {spot_cmd['pitch']} \\
    --yaw {spot_cmd['yaw']} \\
    --frame {args.frame}

echo "Maintien Spot debout..."

for i in $(seq 1 5); do
    ros2 service call /spot/stand std_srvs/srv/Trigger
    echo "Spot maintenu debout"
    sleep 1
done

echo "Lancement du contrôle du bras..."

cd /home/mtouri/ros2_ws_humble/src/whole-body-spot-nb/whole_body_scripts/whole_body_scripts

source /home/mtouri/ros2_ws/install/setup.zsh

python3 send_angles.py \\
    --Xpick {args.xtarget} \\
    --Ypick {args.ytarget} \\
    --Zpick {args.ztarget} \\
    --Xplace {args.xplace} \\
    --Yplace {args.yplace} \\
    --Zplace {args.zplace}

echo "Script send_angles terminé."
"""
    )

    scripts = {
        "estop": estop_script,
        "spot_driver": spot_driver_script,
        "spot_move": spot_move_script,
    }

    print("=== 3. Création layout Terminator ===")
    create_terminator_layout(scripts)

    print("=== 4. Lancement Terminator ===")
    launch_terminator()

    print("Mission Spot + bras lancée.")


if __name__ == "__main__":
    main()