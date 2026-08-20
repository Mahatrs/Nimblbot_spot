#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from sensor_msgs.msg import JointState

DEFAULT_GRIPPER_POSITION_LIMITS = [0.0, 0.0185]

def main():
    rclpy.init()

    node = Node("control_gripper")

    pub = node.create_publisher(
        JointState,
        "/gripper/desired_position",
        10,
    )

    node.create_rate(1)

    msg = JointState()
    msg.position = [DEFAULT_GRIPPER_POSITION_LIMITS[1]]  
    msg.velocity = [100.0]
    msg.effort = [100.0]

    for _ in range(10):
        pub.publish(msg)
        rclpy.spin_once(node, timeout_sec=0.1)

    node.get_logger().info("Commande envoyée")

    node.destroy_node()
    rclpy.shutdown()

if __name__ == "__main__":
    main()