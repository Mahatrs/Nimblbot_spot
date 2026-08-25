# Valve Opening

This guide describes the steps required to start Spot with Nimbl'Bot arm and launch the valve opening sequence using vision.

## 1. Start the Robot

1. Set up the robot.
2. Power on Spot.
3. Power on the IOTA.
4. On the computer, connect to Spot's Wi-Fi network.

## 2. Check the IOTA Connection

Open a terminal and verify that the IOTA is reachable:

```bash
ping iota1.local
```

## 3. Launch RViz Manager

In a terminal, run:

```bash
zsh -c 'source ~/ros2_ws/install/setup.zsh; rviz2 & RVIZ_PID=$!; trap "kill $RVIZ_PID" EXIT; ros2 run rviz_manager manager'
```

Once the RViz window is open:

1. Press and hold the emergency stop button until it is disabled.
2. Click Launch Robot.
3. Wait for the robot to appear in RViz.
4. Open the Operation window.
5. Select the Modular function for the robot.

## 4. Test the Arm

Before starting the valve opening sequence, verify that the arm is working correctly.

Navigate to:

```bash
cd ~/Nimblbot_spot/whole-body-spot-nb/whole_body_scripts/whole_body_scripts/utilities
```

Then run:

```bash
python3 send_angles.py
```

Check that the robot arm responds correctly.

## 5. Launch the Valve Opening Program

Once the robot and arm have been tested successfully, navigate to:

```bash
cd ~/Nimblbot_spot/whole-body-spot-nb/whole_body_bringup/whole_body_bringup
```

Then launch:

```bash
python3 Launch_spot_vanne_visio.py
```

The valve opening sequence can now begin.

---

# ⚠️ Important Safety Notes

Before launching the valve opening sequence, check the following points.

### Arm Collision Warning

The arm does not handle collisions with the surrounding environment.

Be careful when commanding the arm to move to a new position.

Before executing an arm movement:

* Make sure the target position is reachable.
* Check that there are no obstacles along the expected trajectory.
* Make sure there is enough free space around the arm and the valve.
* Be ready to stop the robot if the trajectory is unsafe.

### Spot Movement

Before starting the script, make sure Spot has enough free space around it.

The valve opening sequence may move Spot to position the robot in front of the valve.

Do not start the sequence if there are obstacles, people, equipment, walls, or other objects in Spot's path.

Make sure the area around the valve is also clear before launching the sequence.
