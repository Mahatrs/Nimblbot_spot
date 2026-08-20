#!/usr/bin/env python3

import webbrowser
import numpy as np

import rclpy
from rclpy.node import Node

from trajectory_msgs.msg import (
    JointTrajectory,
    JointTrajectoryPoint,
)

import placo

from placo_utils.visualization import (
    robot_viz,
    robot_frame_viz,
)

from nimblbot_utils.mimic_kinematics import mimic_to_q


class PlacoSnakeNode(Node):

    def __init__(self):

        super().__init__("placo_snake_node")

        # ============================================================
        # CONFIG
        # ============================================================

        self.namespace = "nb"

        self.MJCF_PATH = (
            "/home/mtouri/Desktop/Maha_folder/"
            "robot_descriptions_maha/"
            "mjcf_description/"
            "nb-55-v7_mimic.xml"
        )

        self.nb_moteurs = 25

        self.dt_robot = 1.0 / 20.0

        # ============================================================
        # NOMS DES MOTEURS ROS
        # ============================================================

        alias = "nb55"

        self.joint_names = [
            alias if i == 0 else f"{alias}{i}"
            for i in range(self.nb_moteurs)
        ]

        # ============================================================
        # ROS PUBLISHER
        # ============================================================

        self.publisher = self.create_publisher(
            JointTrajectory,
            f"/{self.namespace}/desired_trajectory_modular",
            10,
        )

        # ============================================================
        # PLACO
        # ============================================================

        self.init_placo()

        # ============================================================
        # TIMER
        # ============================================================

        self.timer = self.create_timer(
            self.dt_robot,
            self.control_loop,
        )

        self.get_logger().info(
            "Snake node prêt."
        )

    # ================================================================
    # INITIALISATION
    # ================================================================

    def init_placo(self):

        print()
        print("Loading model:")
        print(self.MJCF_PATH)
        print()

        self.robot = placo.RobotWrapper(
            self.MJCF_PATH,
            placo.Flags.mjcf
            | placo.Flags.ignore_collisions,
        )

        self.robot.update_kinematics()

        # ============================================================
        # VERIFICATION JOINTS
        # ============================================================

        self.placo_joint_names = list(
            self.robot.joint_names()
        )

        print(
            "Nombre joints PlaCo =",
            len(self.placo_joint_names)
        )

        for i, name in enumerate(
            self.placo_joint_names
        ):
            print(
                f"{i:2d} : {name}"
            )

        if len(self.placo_joint_names) != 49:

            raise RuntimeError(
                f"49 joints attendus, "
                f"{len(self.placo_joint_names)} trouvés."
            )

        # ============================================================
        # POSTURE SERPENT
        # ============================================================
        #
        # 24 valeurs :
        #
        # lower_ring_0
        # upper_ring_0
        # lower_ring_1
        # upper_ring_1
        # ...
        #
        # puis wrist séparément.
        #
        # ============================================================

        snake_degrees = [

             0.0,   8.0,
            15.0,  20.0,

            15.0,   8.0,
             0.0,  -8.0,

           -15.0, -20.0,
           -15.0,  -8.0,

             0.0,   8.0,
            15.0,  20.0,

            15.0,   8.0,
             0.0,  -8.0,

           -15.0, -20.0,
           -15.0,  -8.0,
        ]

        if len(snake_degrees) != 24:
            raise RuntimeError(
                "snake_degrees doit contenir 24 valeurs."
            )

        # ============================================================
        # ECRITURE DIRECTE DANS q
        # ============================================================

        q = self.robot.state.q.copy()

        q_joints = q[7:]

        print()
        print("===== CREATION POSTURE SERPENT =====")

        for i in range(12):

            # Deux articulations actives par module
            lower_angle = np.deg2rad(
                snake_degrees[2 * i]
            )

            upper_angle = np.deg2rad(
                snake_degrees[2 * i + 1]
            )

            # --------------------------------------------------------
            # INDICES DANS q_joints
            # --------------------------------------------------------
            #
            # module i :
            #
            # 4*i     lower_ring
            # 4*i+1   lower_middle
            # 4*i+2   upper_ring
            # 4*i+3   upper_middle
            #
            # --------------------------------------------------------

            lower_ring_idx = 4 * i
            lower_middle_idx = 4 * i + 1

            upper_ring_idx = 4 * i + 2
            upper_middle_idx = 4 * i + 3

            # Ring
            q_joints[
                lower_ring_idx
            ] = lower_angle

            # Mimic opposé
            q_joints[
                lower_middle_idx
            ] = -lower_angle

            # Ring supérieur
            q_joints[
                upper_ring_idx
            ] = upper_angle

            # Mimic opposé
            q_joints[
                upper_middle_idx
            ] = -upper_angle

            print(
                f"Module {i:2d} : "
                f"lower={snake_degrees[2*i]:+6.1f}°, "
                f"upper={snake_degrees[2*i+1]:+6.1f}°"
            )

        # ============================================================
        # WRIST
        # ============================================================

        wrist_angle = np.deg2rad(
            0.0
        )

        q_joints[48] = (
            wrist_angle
        )

        # ============================================================
        # REMETTRE DANS q
        # ============================================================

        q[7:] = q_joints

        self.robot.state.q = q

        self.robot.update_kinematics()

        print()
        print(
            "Posture écrite directement dans PlaCo."
        )

        # ============================================================
        # CONVERSION VERS 25 MOTEURS
        # ============================================================

        active_mask = np.array(
            [True, False, True, False] * 12
            + [True],
            dtype=bool,
        )

        q_active = (
            q_joints[
                active_mask
            ]
        )

        print(
            "Nombre joints actifs =",
            len(q_active)
        )

        result = mimic_to_q(
            q_active
        )

        # Même comportement que ton ancien script
        self.positions = [
            float(x)
            for x in result[0]
        ]

        print(
            "Nombre positions ROS =",
            len(self.positions)
        )

        if len(self.positions) != 25:

            raise RuntimeError(
                f"25 positions attendues, "
                f"{len(self.positions)} obtenues."
            )

        # ============================================================
        # AFFICHAGE DES COMMANDES
        # ============================================================

        print()
        print(
            "========== COMMANDES ROS =========="
        )

        for name, value in zip(
            self.joint_names,
            self.positions,
        ):

            print(
                f"{name:<10} : "
                f"{np.rad2deg(value):+7.2f}°"
            )

        print(
            "==================================="
        )
        print()

        # ============================================================
        # VISUALISATION
        # ============================================================

        self.viz = robot_viz(
            self.robot
        )

        webbrowser.open(
            self.viz.viewer.url()
        )

        self.viz.display(
            self.robot.state.q
        )

        robot_frame_viz(
            self.robot,
            "tcp"
        )

        self.get_logger().info(
            "Posture serpent chargée."
        )

    # ================================================================
    # PUBLICATION
    # ================================================================

    def publish_positions(self):

        msg = JointTrajectory()

        msg.joint_names = (
            self.joint_names
        )

        point = (
            JointTrajectoryPoint()
        )

        point.positions = (
            self.positions
        )

        # Petite vitesse
        point.velocities = [
            0.10
        ] * 25

        point.effort = [
            0.0
        ] * 25

        msg.points.append(
            point
        )

        self.publisher.publish(
            msg
        )

    # ================================================================
    # CONTROL LOOP
    # ================================================================

    def control_loop(self):

        try:

            # Publier continuellement la même posture
            self.publish_positions()

            # Affichage PlaCo
            self.viz.display(
                self.robot.state.q
            )

        except Exception as e:

            self.get_logger().error(
                f"Erreur control_loop : {e}"
            )


# ====================================================================
# MAIN
# ====================================================================

def main():

    rclpy.init()

    node = PlacoSnakeNode()

    try:

        rclpy.spin(
            node
        )

    except KeyboardInterrupt:

        print(
            "\nArrêt demandé."
        )

    finally:

        node.destroy_node()

        rclpy.try_shutdown()


if __name__ == "__main__":
    main()