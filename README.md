# Spot – Nimbl'Bot

This repository contains the main developments carried out during my internship at Nimbl'Bot, focusing on the use of the Spot quadruped robot with the Nimbl'Bot robotic arm.

The project includes several applications combining Spot navigation, arm control, computer vision, and coordinated movements between Spot and the manipulator.

The main applications developed in this repository include:

* Pick and Place: Spot retrieves a tool and brings it back to a human operator.
* Valve Opening: Spot uses vision to detect and approach a valve, allowing the arm to interact with it.
* Door Opening: Spot and the Nimbl'Bot arm move simultaneously to open a door and allow the robot to enter a room.

More detailed README files describing how to launch and use each application can be found in the following path:

```text id="1nif70"
whole-body-spot-nb/whole_body_bringup/whole_body_bringup/
```

These README files contain the required commands, execution steps, and important safety notes for each application.

## Environment

Main tools and technologies used:

* Python
* ROS 2 Humble
* Placo
* Spot SDK
* MeshCat
* OpenCV
* YOLO

