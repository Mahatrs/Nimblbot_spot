# Coordinated Door Opening

This guide describes the steps required to run the demonstration sequence where Spot and the Nimbl'Bot arm move together to open a kitchen door and continue toward the coffee machine.

The sequence combines teleoperation with coordinated commands between Spot and the arm.

## 1. Start the Robot

1. Set up the robot.
2. Power on Spot.
3. Power on the IOTA.
4. On the computer, connect to Spot's Wi-Fi network.

## 2. Check the IOTA Connection

Open a terminal and verify that the IOTA is reachable:

```bash id="c8myql"
ping iota1.local
```

## 3. Launch RViz Manager

In a terminal, run:

```bash id="5smts2"
zsh -c 'source ~/ros2_ws/install/setup.zsh; rviz2 & RVIZ_PID=$!; trap "kill $RVIZ_PID" EXIT; ros2 run rviz_manager manager'
```

Once the RViz window is open:

1. Press and hold the emergency stop button until it is disabled.
2. Click Launch Robot.
3. Wait for the robot to appear in RViz.
4. Open the Operation window.
5. Select the Modular function for the robot.

## 4. Test the Arm

Before starting the demonstration, verify that the arm is working correctly.

Navigate to:

```bash id="1dbzpy"
cd ~/Nimblbot_spot/whole-body-spot-nb/whole_body_scripts/whole_body_scripts/utilities
```

Then run:

```bash id="b2trxf"
python3 send_angles.py
```

Check that the robot arm responds correctly.

## 5. Demonstration Sequence

The complete demonstration is divided into several phases.

### Approach the Door

Spot starts near the main entrance and moves toward the kitchen door.

This phase can be performed using teleoperation to correctly position Spot in front of the door.

### Deploy the Arm

Once Spot is correctly positioned, the Nimbl'Bot arm moves from its rest configuration toward the door.

Make sure the arm has enough free space to deploy.

### Coordinated Door Opening

The door must already be slightly open before starting this phase.

Spot begins a rotation while the arm follows the movement of the door.

During this phase, Spot and the Nimbl'Bot arm move simultaneously. Unlike the other sequences where the movements are mainly executed one after another, the mobile base and the manipulator must remain coordinated during the door opening.

### Enter the Kitchen

Once the door is sufficiently open, Spot moves forward into the kitchen.

At the same time, the arm progressively returns toward its rest configuration to reduce its footprint while Spot passes through the doorway.

### Approach the Coffee Machine

After passing through the door, Spot performs a final rotation and moves toward the coffee machine.

The final positioning can then be completed using teleoperation if necessary.

---

# ⚠️ Important Safety Notes

Before starting the sequence:

* Make sure the door is slightly open.
* Keep enough free space around Spot and the arm.
* Make sure no person or obstacle is in the robot's path.
* Be careful during the door opening phase, as Spot and the arm move simultaneously.
* The arm does not handle collisions with the environment.
* Make sure the door is sufficiently open before Spot enters the kitchen.
* Be ready to stop the robot if the movement becomes unsafe.

