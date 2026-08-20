#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from std_msgs.msg import Int8
from std_srvs.srv import Empty
from trajectory_msgs.msg import JointTrajectory, JointTrajectoryPoint

import rclpy.logging
from rclpy.node import Node
from rclpy.qos import QoSProfile, DurabilityPolicy
from rclpy.executors import ShutdownException


class SendAnglesNode(Node):

    def __init__(self):
        super().__init__("send_angles_node")

        self.namespace = "nb"
        self.publisher_modular = self.create_publisher(Int8,f"/{self.namespace}/change_control_mode",10)
        self.publisher_modular.publish(Int8(data=2))
      
        self.publisher = self.create_publisher(JointTrajectory,f"/{self.namespace}/desired_trajectory_modular",10)
        self.timer = self.create_timer(0.1,self.send_angles)


    def send_angles(self):

        msg = JointTrajectory()
    

        alias = ["nb55"]
        msg.joint_names = alias + [alias[0] + str(i) for i in range(1, 25)]


        point = JointTrajectoryPoint()

        point.positions = [0.3] * len(msg.joint_names)

        point.velocities = [0.45] * len(point.positions)
        point.effort = [0.0]  * len(point.positions)

        #self.publisher_modular.publish(Int8(data=2))

        msg.points.append(point)
        self.publisher.publish(msg)

        self.get_logger().info("Trajectory sent")

    def cleanup(self):

        shutdown_context = rclpy.Context()
        rclpy.init(context=shutdown_context)
        shutdown_node = rclpy.create_node(
            f"shutdown_{self.get_name()}", context=shutdown_context
        )
        publisher_modular = self.create_publisher(Int8,f"/{self.namespace}/change_control_mode",10)
        publisher_modular.publish(Int8(data=0))

        shutdown_node.destroy_node()
        rclpy.try_shutdown(context=shutdown_context)

def main():

    rclpy.init()
    node = SendAnglesNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        rclpy.logging.get_logger("rclpy").info("Interrupted (SIGINT)")
    except ShutdownException:
        pass
    except Exception as exc:
        node.get_logger().error(f"Unexpected exception: {exc}")
    finally:
        node.cleanup()
        node.destroy_node()
        rclpy.try_shutdown()


if __name__ == "__main__":
    main()