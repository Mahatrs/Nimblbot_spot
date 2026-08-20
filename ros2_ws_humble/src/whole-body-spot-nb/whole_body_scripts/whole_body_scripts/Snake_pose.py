#!/usr/bin/env python3

import math
import time

import rclpy
from rclpy.node import Node

from trajectory_msgs.msg import JointTrajectory, JointTrajectoryPoint


class NIMBLROSNode(Node):

    def __init__(self):
        super().__init__("nimbl_angles_publisher")

        self.namespace = "nb"

        alias = ["nb55"]
        self.nb_moteurs = 12 * 2 + 1

        self.joint_names = (
            alias
            + [
                alias[0] + str(i)
                for i in range(1, self.nb_moteurs)
            ]
        )

        # 20 Hz
        self.dt_robot = 1.0 / 20.0

        # ============================================================
        # ANGLES EN DEGRES
        # ============================================================

        self.angles_deg = [
            -179.999999,  # nb55
            -0.000001,    # nb551
            -179.999999,  # nb552
            -0.000001,    # nb553
            -179.999999,  # nb554
            -0.000001,    # nb555
            0.000001,     # nb556
            179.999999,   # nb557
            0.000001,     # nb558
            179.999999,   # nb559
            0.000001,     # nb5510
            179.999999,   # nb5511
            0.000001,     # nb5512
            179.999999,   # nb5513
            0.000001,     # nb5514
            179.999999,   # nb5515
            0.000001,     # nb5516
            179.999999,   # nb5517
            0.000001,     # nb5518
            179.999999,   # nb5519
            180.0,        # nb5520
            180.0,        # nb5521
            -103.19477,   # nb5522
            -76.80523,    # nb5523
            30.0,         # nb5524
        ]


        # Vérification
        if len(self.angles_deg) != self.nb_moteurs:
            raise ValueError(
                f"Il faut {self.nb_moteurs} angles, "
                f"mais tu en as {len(self.angles_deg)}"
            )

        # Conversion degrés -> radians
        self.positions = [
            math.radians(angle)
            for angle in self.angles_deg
        ]

        # ============================================================
        # DUREE DE PUBLICATION
        # ============================================================

        self.publish_duration = 15.0  # secondes
        self.start_time = time.time()

        # ============================================================
        # PUBLISHER
        # ============================================================

        self.publisher = self.create_publisher(
            JointTrajectory,
            f"/{self.namespace}/desired_trajectory_modular",
            10
        )

        # ============================================================
        # TIMER
        # ============================================================

        self.timer = self.create_timer(
            self.dt_robot,
            self.publish_positions
        )

        self.get_logger().info(
            f"Publisher lancé pendant {self.publish_duration:.1f} secondes"
        )

    def publish_positions(self):

        # Temps écoulé
        elapsed_time = time.time() - self.start_time

        # ============================================================
        # ARRET APRES LA DUREE DEMANDEE
        # ============================================================

        if elapsed_time >= self.publish_duration:

            self.get_logger().info(
                f"Durée de {self.publish_duration:.1f}s atteinte. "
                "Arrêt du node."
            )

            self.timer.cancel()
            rclpy.shutdown()
            return

        # ============================================================
        # CREATION DU MESSAGE
        # ============================================================

        msg = JointTrajectory()
        msg.joint_names = self.joint_names

        point = JointTrajectoryPoint()

        point.positions = [
            float(x)
            for x in self.positions
        ]

        point.velocities = [
            0.45
        ] * len(point.positions)

        point.effort = [
            0.0
        ] * len(point.positions)

        msg.points.append(point)

        # ============================================================
        # PUBLICATION
        # ============================================================

        self.publisher.publish(msg)

        self.get_logger().info(
            f"Trajectory sent - "
            f"{elapsed_time:.1f}/{self.publish_duration:.1f}s"
        )


def main():

    rclpy.init()

    node = NIMBLROSNode()

    try:
        rclpy.spin(node)

    except KeyboardInterrupt:
        pass

    finally:

        node.destroy_node()

        if rclpy.ok():
            rclpy.try_shutdown()


if __name__ == "__main__":
    main()