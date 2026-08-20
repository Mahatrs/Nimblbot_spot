#!/usr/bin/env python

import rclpy
from rclpy.lifecycle import Node

from sensor_msgs.msg import JointState


DEFAULT_GRIPPER_POSITION_LIMITS = [0.0, 0.0185]  # m
DEFAULT_GRIPPER_VELOCITY = 100.0  # %
DEFAULT_GRIPPER_EFFORT = 100.0  # %


rclpy.init()
node = Node("control_gripper")

pub = node.create_publisher(
    JointState,
    "/gripper/desired_position",
    10,
)

gripper_close = DEFAULT_GRIPPER_POSITION_LIMITS[0]
gripper_open = DEFAULT_GRIPPER_POSITION_LIMITS[1]

current = gripper_close


def desired_position():
    global current
    msg = JointState()

    if current == gripper_close:
        current = gripper_open
    else:
        current = gripper_close

    msg.position = [current]
    msg.velocity = [DEFAULT_GRIPPER_VELOCITY]
    msg.effort = [DEFAULT_GRIPPER_EFFORT]

    pub.publish(msg)


desired_position()
timer = node.create_timer(1.0, desired_position)

try:
    rclpy.spin(node)
except Exception:
    pass
finally:
    node.destroy_node()
    rclpy.try_shutdown()
