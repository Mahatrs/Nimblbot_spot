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
from sensor_msgs.msg import JointState

import placo
from placo_utils.tf import tf
from placo_utils.visualization import robot_viz, robot_frame_viz, frame_viz

import importlib
from slugify import slugify
from nimblbot_utils.mimic_kinematics import mimic_to_q


def rot_x(angle):
    c, s = np.cos(angle), np.sin(angle)
    return np.array([
        [1, 0, 0],
        [0, c, -s],
        [0, s, c]
    ])


def rot_y(angle):
    c, s = np.cos(angle), np.sin(angle)
    return np.array([
        [c, 0, s],
        [0, 1, 0],
        [-s, 0, c]
    ])


def rot_z(angle):
    c, s = np.cos(angle), np.sin(angle)
    return np.array([
        [c, -s, 0],
        [s, c, 0],
        [0, 0, 1]
    ])


class PlacoToROSNode(Node):

    def __init__(self, args):
        super().__init__("placo_to_ros_node")

        self.namespace = "nb"
        self.args = args

        alias = ["nb55"]
        self.nb_moteurs = 12 * 2 + 1
        self.joint_names = alias + [
            alias[0] + str(i) for i in range(1, self.nb_moteurs)
        ]

        self.dt_robot = 1.0 / 20.0
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

        self.stop_delay = 20.0
        self.target_reached_time = None
        self.last_positions = None

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

        # Cible exprimée dans le repère du modèle Placo.
        # Si le modèle Placo a sa base à l'origine, c'est cohérent.
        self.target_center = R_world_base.T @ (target_world - base_world)

        conf = load_robot_config("nb60_v7_4T")
        mjcf_maker.main(conf, quat_base={"w": 1, "x": 0, "y": 0, "z": 0})

        self.MJCF_PATH = (
            importlib.resources.files("nimblpy.generators")
            / "models"
            / f"{slugify(conf['robot_name'])}_mimic.xml"
        )

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

        T0 = self.robot.get_T_world_frame("tcp").copy()
        tcp0 = T0[:3, 3].copy()

        self.effector_task = self.solver.add_position_task("tcp", tcp0)
        self.effector_task.configure("tcp", "soft", 50.0)

        self.viz = robot_viz(self.robot)
        webbrowser.open(self.viz.viewer.url())

        self.get_logger().info(
            f"Target Placo : {self.target_center}"
        )

    def update_placo_from_measurements(self):
        """
        Réinjecte la vraie position robot dans le modèle Placo.
        C'est indispensable pour éviter que Placo résolve depuis une posture fictive.
        """
        if self.q_mesure is None:
            return False

        q = self.robot.state.q.copy()

        q_joints_init = []

        for i in range(self.nb_moteurs):
            if i < len(self.q_mesure):
                value = self.q_mesure[i]
            else:
                value = 0.0

            q_joints_init.append(value)
            q_joints_init.append(-value)

        # Le dernier moteur n'a pas de paire négative dans ton modèle
        q_joints_init.pop(-1)

        n = min(len(q[7:]), len(q_joints_init))
        q[7:7 + n] = q_joints_init[:n]

        self.robot.state.q = q
        self.robot.update_kinematics()

        return True

    def compute_placo_positions(self):
        target = self.target_center.copy()

        self.effector_task.target_world = target

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

        # Pour un point final, les vitesses désirées doivent souvent être nulles.
        point.velocities = [0.0] * len(point.positions)
        point.effort = [0.0] * len(point.positions)

        # Important : certains contrôleurs ont besoin d'un time_from_start.
        point.time_from_start.sec = 1
        point.time_from_start.nanosec = 0

        msg.points.append(point)
        self.publisher.publish(msg)

    def control_loop(self):
        try:
            if self.target_reached:
                self.publish_positions(self.last_positions)

                if time.time() - self.target_reached_time > self.stop_delay:
                    self.get_logger().info("Fin du maintien, arrêt du node")
                    self.timer.cancel()
                    rclpy.shutdown()

                return


            positions = self.compute_placo_positions()

            tcp_pos = self.robot.get_T_world_frame("tcp")[:3, 3]
            error = np.linalg.norm(tcp_pos - self.target_center)

            self.get_logger().info(
                f"Erreur TCP-target : {error:.4f} m"
            )

            if error < self.reached_threshold:
                self.target_reached = True
                self.target_reached_time = time.time()
                self.last_positions = positions

                self.get_logger().info(
                    f"Target atteinte, maintien pendant {self.stop_delay}s"
                )

                self.publish_positions(self.last_positions)
                return

            self.viz.display(self.robot.state.q)
            robot_frame_viz(self.robot, "tcp")
            robot_frame_viz(self.robot, "lower_ring_0")
            frame_viz("target", tf.translation_matrix(self.target_center))

            self.publish_positions(positions)

        except Exception as e:
            self.get_logger().error(f"Erreur dans control_loop: {e}")


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