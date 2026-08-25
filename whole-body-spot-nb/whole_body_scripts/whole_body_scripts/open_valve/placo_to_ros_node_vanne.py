#!/usr/bin/env python3

import argparse
import webbrowser
import time
import numpy as np

import rclpy
from rclpy.node import Node
from nimblpy.generators import mjcf_maker
from nimblpy.common.robot_loader import load_robot_config

from trajectory_msgs.msg import JointTrajectory, JointTrajectoryPoint

import placo
from placo_utils.tf import tf
from placo_utils.visualization import robot_viz, robot_frame_viz, frame_viz
import importlib
from slugify import slugify
from nimblbot_utils.mimic_kinematics import mimic_to_q, q_to_mimic
from sensor_msgs.msg import JointState


def rot_x(angle):
    c, s = np.cos(angle), np.sin(angle)
    return np.array([[1, 0, 0], [0, c, -s], [0, s, c]])


def rot_y(angle):
    c, s = np.cos(angle), np.sin(angle)
    return np.array([[c, 0, s], [0, 1, 0], [-s, 0, c]])


def rot_z(angle):
    c, s = np.cos(angle), np.sin(angle)
    return np.array([[c, -s, 0], [s, c, 0], [0, 0, 1]])


def smooth(s):
    s = np.clip(s, 0.0, 1.0)
    return 3 * s**2 - 2 * s**3


MODE_REACH = 0
MODE_WAIT = 1
MODE_PUSH_FORWARD = 2
MODE_CIRCLE = 3
MODE_PUSH_BACKWARD = 4
MODE_DONE = 5


class PlacoToROSNode(Node):

    def __init__(self, args):
        super().__init__("placo_to_ros_node")

        conf = load_robot_config("nb60_v7_4T")
        mjcf_maker.main(conf, quat_base={"w": 1, "x": 0, "y": 0, "z": 0})

        self.MJCF_PATH = (
            importlib.resources.files("nimblpy.generators")
            / "models"
            / f"{slugify(conf['robot_name'])}_mimic.xml"
        )

        self.MJCF_PATH = "/home/mtouri/Desktop/Maha_folder/robot_descriptions_maha/mjcf_description/nb-55-v7_mimic.xml"

        self.namespace = "nb"
        self.args = args

        alias = ["nb55"]
        self.nb_moteurs = 12 * 2 + 1
        self.joint_names = alias + [alias[0] + str(i) for i in range(1, self.nb_moteurs)]

        self.dt_robot = 1 / 20
        self.q_mesure = None

        self.publisher = self.create_publisher(
            JointTrajectory,
            f"/{self.namespace}/desired_trajectory_modular",
            10
        )

        self.subscriber_q = self.create_subscription(
            JointState,
            "/nb/mdh_measurements",
            self.__on_mdh_measurements,
            10
        )

        self.init_placo()
        self.timer = self.create_timer(self.dt_robot, self.control_loop)

    def __on_mdh_measurements(self, msg: JointState) -> None:
        self.q_mesure = list(msg.position)

    def init_placo(self):

        self.target_reached = False
        self.reached_threshold = 0.005

        # Pauses différentes selon les étapes
        self.wait_after_reach = 18.0
        self.wait_after_push = 16.0
        self.wait_after_circle = 16.0

        self.wait_duration = 0.0
        self.next_mode = None

        target_world = np.array([
            self.args.Xtarget,
            self.args.Ytarget,
            self.args.Ztarget,
        ])

        base_world = np.array([
            self.args.Xbase,
            self.args.Ybase,
            self.args.Zbase,
        ])

        R_world_base = (
            rot_z(self.args.yaw)
            @ rot_y(self.args.pitch)
            @ rot_x(self.args.roll)
        )

        self.target_center = R_world_base.T @ (target_world - base_world)

        self.mode = MODE_REACH
        self.motion_time = 0.0

        self.push_distance = 0.12
        self.push_duration = 10.0

        self.circle_radius = 0.04
        self.circle_duration = 15.0
        self.nb_tours = 1

        self.push_start = None
        self.push_target = None
        self.circle_center = None

        self.R_tcp_target = rot_y(np.pi / 2)

        print("MJCF_PATH =", self.MJCF_PATH)

        self.robot = placo.RobotWrapper(
            str(self.MJCF_PATH),
            placo.Flags.mjcf | placo.Flags.ignore_collisions
        )

        self.solver = placo.KinematicsSolver(self.robot)
        self.solver.mask_fbase(True)
        self.solver.dt = self.dt_robot
        self.solver.add_regularization_task(1e-4)

        for i in range(12 * 2):
            self.solver.add_gear_task().set_gear(
                self.robot.joint_names()[2 * i],
                self.robot.joint_names()[2 * i + 1],
                -1.0
            )
            self.robot.update_kinematics()

        print("q_mesure =", self.q_mesure)
        print("Target center =", self.target_center)

        tcp_pos = self.robot.get_T_world_frame("tcp")[:3, 3]
        print("TCP =", tcp_pos)
        print("Erreur =", np.linalg.norm(tcp_pos - self.target_center))

        if self.q_mesure is not None:
            q = self.robot.state.q.copy()
            q_joints_vrai = q[7:].copy()
            q_joints_mimic = q_to_mimic(q_joints_vrai)

            q_joints_init = []

            for i in range(self.nb_moteurs):
                q_joints_init.append(q_joints_mimic[i])
                q_joints_init.append(-q_joints_mimic[i])

            q_joints_init.pop(-1)

            for i in range(min(len(self.q_mesure), len(q_joints_init))):
                q_joints_init[i] = self.q_mesure[i]

            q[7:] = q_joints_init
            self.robot.state.q = q
            self.robot.update_kinematics()

        T0 = self.robot.get_T_world_frame("tcp").copy()
        tcp0 = T0[:3, 3].copy()

        self.effector_task = self.solver.add_position_task("tcp", tcp0)
        self.effector_task.configure("tcp", "soft", 50.0)

        self.orientation_task = self.solver.add_orientation_task("tcp", self.R_tcp_target)
        self.orientation_task.configure("tcp_ori", "soft", 0.01)

        self.viz = robot_viz(self.robot)
        webbrowser.open(self.viz.viewer.url())

        self.get_logger().info(
            f"Target du bras avec base à (0 0 0): {self.target_center}"
        )

    def start_wait(self, next_mode, duration):
        self.mode = MODE_WAIT
        self.next_mode = next_mode
        self.wait_duration = duration
        self.motion_time = 0.0

        self.get_logger().info(
            f"Pause de {self.wait_duration}s avant mode {self.next_mode}"
        )

    def compute_placo_positions(self):
        dt = self.dt_robot
        tcp_pos = self.robot.get_T_world_frame("tcp")[:3, 3].copy()

        if self.mode == MODE_REACH:
            target = self.target_center.copy()
            error = np.linalg.norm(tcp_pos - target)

            if error < self.reached_threshold:
                self.start_wait(MODE_PUSH_FORWARD, self.wait_after_reach)
                target = tcp_pos.copy()
                self.get_logger().info("Point atteint -> grande pause avant avance +X")

        elif self.mode == MODE_WAIT:
            self.motion_time += dt
            target = tcp_pos.copy()

            if self.motion_time >= self.wait_duration:
                self.mode = self.next_mode
                self.motion_time = 0.0

                if self.mode == MODE_PUSH_FORWARD:
                    self.push_start = tcp_pos.copy()
                    self.push_target = self.push_start + np.array([
                        self.push_distance,
                        0.0,
                        0.0
                    ])
                    self.get_logger().info("Fin pause -> avance +X")

                elif self.mode == MODE_CIRCLE:
                    self.circle_center = tcp_pos + np.array([
                        0.0,
                        0.0,
                        -self.circle_radius
                    ])
                    self.get_logger().info("Fin pause -> cercle autour axe X")

                elif self.mode == MODE_PUSH_BACKWARD:
                    self.push_start = tcp_pos.copy()
                    self.push_target = self.push_start + np.array([
                        -self.push_distance,
                        0.0,
                        0.0
                    ])
                    self.get_logger().info("Fin pause -> retour -X")

        elif self.mode == MODE_PUSH_FORWARD:
            self.motion_time += dt
            s = smooth(self.motion_time / self.push_duration)

            target = (1 - s) * self.push_start + s * self.push_target

            if self.motion_time >= self.push_duration:
                self.start_wait(MODE_CIRCLE, self.wait_after_push)
                target = tcp_pos.copy()
                self.get_logger().info("Avance finie -> pause avant cercle")

        elif self.mode == MODE_CIRCLE:
            self.motion_time += dt
            s = smooth(self.motion_time / self.circle_duration)

            #theta = self.nb_tours * 2.0 * np.pi * s
            theta = np.pi * s

            target = self.circle_center + np.array([
                0.0,
                self.circle_radius * np.sin(theta),
                self.circle_radius * np.cos(theta),
            ])

            if self.motion_time >= self.circle_duration:
                self.start_wait(MODE_PUSH_BACKWARD, self.wait_after_circle)
                target = tcp_pos.copy()
                self.get_logger().info("Cercle fini -> pause avant retour -X")

        elif self.mode == MODE_PUSH_BACKWARD:
            self.motion_time += dt
            s = smooth(self.motion_time / self.push_duration)

            target = (1 - s) * self.push_start + s * self.push_target

            if self.motion_time >= self.push_duration:
                self.mode = MODE_DONE
                target = tcp_pos.copy()
                self.get_logger().info("Mouvement complet terminé")

        else:
            target = tcp_pos.copy()

        self.effector_task.target_world = target
        self.orientation_task.R_world_frame = self.R_tcp_target

        for _ in range(5):
            self.solver.solve(True)
            self.robot.update_kinematics()

        q_mjcf_full = self.robot.state.q.copy()
        q_joints = q_mjcf_full[7:]

        active_mask_joints = np.array(
            [True, False, True, False] * 12 + [True],
            dtype=bool
        )

        q_active_values = q_joints[active_mask_joints]
        q_fr_mjcf = mimic_to_q(q_active_values)[0]

        return [float(x) for x in q_fr_mjcf]

    def publish_positions(self, positions):
        msg = JointTrajectory()
        msg.joint_names = self.joint_names

        point = JointTrajectoryPoint()
        point.positions = [float(x) for x in positions]

        point.velocities = [0.45] * len(point.positions)
        point.effort = [0.0] * len(point.positions)

        msg.points.append(point)
        self.publisher.publish(msg)

        self.get_logger().info("Trajectory sent")

    def control_loop(self):
        try:
            positions = self.compute_placo_positions()

            if self.mode == MODE_DONE:
                self.publish_positions(positions)
                self.get_logger().info("Fin du mouvement, arrêt du node")
                self.timer.cancel()
                rclpy.shutdown()
                return

            self.viz.display(self.robot.state.q)
            robot_frame_viz(self.robot, "tcp")
            #robot_frame_viz(self.robot, "lower_ring_0")
            #frame_viz("target", tf.translation_matrix(self.target_center))

            self.publish_positions(positions)

        except Exception as e:
            self.get_logger().error(f"Erreur dans control_loop(): {e}")


def parse_args():
    parser = argparse.ArgumentParser()

    parser.add_argument("--Xbase", default=0.0, type=float)
    parser.add_argument("--Ybase", default=0.0, type=float)
    parser.add_argument("--Zbase", default=0.0, type=float)

    parser.add_argument("--Xtarget", default=0.1, type=float)
    parser.add_argument("--Ytarget", default=0.1, type=float)
    parser.add_argument("--Ztarget", default=0.3, type=float)

    parser.add_argument("--roll", default=0.0, type=float)
    parser.add_argument("--pitch", default=0.0, type=float)
    parser.add_argument("--yaw", default=0.0, type=float)

    return parser.parse_args()


def main():
    args = parse_args()

    rclpy.init()
    node = PlacoToROSNode(args)

    try:
        rclpy.spin(node)

    finally:
        node.destroy_node()
        rclpy.try_shutdown()


if __name__ == "__main__":
    main()