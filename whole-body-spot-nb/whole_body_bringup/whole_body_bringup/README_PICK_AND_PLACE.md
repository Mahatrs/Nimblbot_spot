# Pick and Place

This guide describes the steps required to start Spot with Nimbl'Bot arm, retrieve a tool, and bring it back to a human operator.

## 1. Start the Robot

1. Set up the robot.
2. Power on **Spot**.
3. Power on the **IOTA**.
4. On the computer, connect to Spot's Wi-Fi network.

## 2. Check the IOTA Connection

Open a terminal and verify that the IOTA is reachable:

```bash id="gnhnxv"
ping iota1.local
```

## 3. Launch RViz Manager

In a terminal, run:

```bash id="x0wx8r"
zsh -c 'source ~/ros2_ws/install/setup.zsh; rviz2 & RVIZ_PID=$!; trap "kill $RVIZ_PID" EXIT; ros2 run rviz_manager manager'
```

Once the **RViz** window is open:

1. Press and hold the **emergency stop button** until it is disabled.
2. Click **Launch Robot**.
3. Launch the **Gripper**.
4. Wait for the robot to appear in RViz.
5. Open the **Operation** window.
6. Select the **Modular** function for the robot.

## 4. Test the Arm

Before starting the Pick and Place operation, verify that the arm is working correctly.

Navigate to:

```bash id="vw7b7c"
cd ~/Nimblbot_spot/whole-body-spot-nb/whole_body_scripts/whole_body_scripts/utilities
```

Then run:

```bash id="pxgt5w"
python3 send_angles.py
```

Check that the robot arm responds correctly.

## 5. Test the Gripper

Run:

```bash id="n72p10"
python3 gripper_control.py
```

Check that the gripper responds correctly to the commands.

### If `gripper_control.py` Does Not Work

Open a **new terminal** and connect to the IOTA:

```bash id="sz9b2h"
ssh nimblbot@iota1.local
```

Then run:

```bash id="muj5jp"
ros2 topic echo gripper/desired_position
```

Keep this terminal **open for the rest of the operation**.

## 6. Launch the Pick and Place Program

Once the robot, arm, and gripper have been tested successfully, navigate to:

```bash id="qjljvl"
cd ~/Nimblbot_spot/whole-body-spot-nb/whole_body_bringup/whole_body_bringup
```

Then launch:

```bash id="rx5s3l"
python3 Launch_pick_and_place.py
```

The **Pick and Place** sequence can now begin. Spot will retrieve the tool and bring it back to the human operator.

---

# ⚠️ Important Safety Notes

Before launching the Pick and Place sequence, carefully check the following points.

### Gripper Cable

Make sure that the gripper cable is properly connected and secured before starting the robot.

The cable must be positioned so that it does not interfere with Spot, its arm, the gripper, or any moving part during operation.


### Arm Collision Warning

> **The arm does not handle collisions with the surrounding environment.**

Be especially careful when commanding the arm to move to a new position.

Before executing an arm movement:

* Make sure the target position is reachable.
* Check that there are no obstacles along the expected trajectory.
* Be ready to stop the robot if the trajectory is unsafe.

### Spot Initial Forward Movement

> **Before starting the script, make sure Spot has enough free space in front of it.**

In the current Pick and Place sequence, Spot first moves forward by approximately 1 meter and then by 3 meters.

Do not start the sequence if there are obstacles, people, equipment, walls, or other objects in Spot's path.

If the available space is insufficient, the forward movement values can be modified in the launch file before running the sequence.
