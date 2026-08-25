import argparse
import logging
import math
import os
import time
import json
from typing import Optional, Tuple

import bosdyn.geometry
import cv2
import numpy as np
from bosdyn.client.frame_helpers import (
    BODY_FRAME_NAME,
    ODOM_FRAME_NAME,
    VISION_FRAME_NAME,
)
from bosdyn.client.math_helpers import Quat, SE2Pose, SE3Pose
from bosdyn.client.robot_command import RobotCommandBuilder
from bosdyn_msgs.conversions import convert
from cv_bridge import CvBridge
from rclpy.node import Node
from sensor_msgs.msg import CameraInfo, Image
import torch
import rclpy
from rclpy.qos import qos_profile_sensor_data

import synchros2.process as ros_process
import synchros2.scope as ros_scope
from spot_msgs.action import RobotCommand
from synchros2.action_client import ActionClientWrapper
from synchros2.tf_listener_wrapper import TFListenerWrapper
from synchros2.utilities import fqn, namespace_with

from .simple_spot_commander import SimpleSpotCommander


class RelativeMoveThenDetectValve:
    def __init__(
    self,
    robot_name: Optional[str],
    node: Optional[Node],
    model_path: str,
    left_rgb_topic: str,
    left_depth_topic: str,
    left_camera_info_topic: str,
    right_rgb_topic: str,
    right_depth_topic: str,
    right_camera_info_topic: str,
    confidence: float,
    valve_class_name: str,
    output_path: str,
    depth_scale: float,
) -> None:
        self._logger = logging.getLogger(fqn(self.__class__))

        node = node or ros_scope.node()
        if node is None:
            raise ValueError("No ROS 2 node available")

        self._node = node
        self._robot_name = robot_name
        self._bridge = CvBridge()

        self._robot = SimpleSpotCommander(robot_name, node)

        self._robot_command_client = ActionClientWrapper(RobotCommand,namespace_with(robot_name, "robot_command"),node)

        self._tf_listener = TFListenerWrapper(node)

        # ------------------------------------------------------------------
        # YOLO
        # ------------------------------------------------------------------
        self._logger.info("Loading YOLO model: %s", model_path)
        self._model = torch.hub.load("ultralytics/yolov5","custom",path=model_path,force_reload=False)
        print("YOLO classes:", self._model.names)

        self._confidence = confidence
        self._valve_class_name = valve_class_name.lower()
        self._output_path = output_path
        self._depth_scale = depth_scale


        # ------------------------------------------------------------------
        # Topics ROS 2
        # ------------------------------------------------------------------
        left_rgb_topic = namespace_with(
            robot_name,
            left_rgb_topic,
        )
        left_depth_topic = namespace_with(
            robot_name,
            left_depth_topic,
        )
        left_camera_info_topic = namespace_with(
            robot_name,
            left_camera_info_topic,
        )

        right_rgb_topic = namespace_with(
            robot_name,
            right_rgb_topic,
        )
        right_depth_topic = namespace_with(
            robot_name,
            right_depth_topic,
        )
        right_camera_info_topic = namespace_with(
            robot_name,
            right_camera_info_topic,
        )

        self._camera_data = {
            "left": {
                "rgb": None,
                "depth": None,
                "camera_info": None,
            },
            "right": {
                "rgb": None,
                "depth": None,
                "camera_info": None,
            },
        }
        self._subscriptions = []

        # Caméra gauche
        self._subscriptions.append(
            node.create_subscription(
                Image,
                left_rgb_topic,
                lambda message: self._rgb_callback(message, "left"),
                qos_profile_sensor_data,
            )
        )

        self._subscriptions.append(
            node.create_subscription(
                Image,
                left_depth_topic,
                lambda message: self._depth_callback(message, "left"),
                qos_profile_sensor_data,
            )
        )

        self._subscriptions.append(
            node.create_subscription(
                CameraInfo,
                left_camera_info_topic,
                lambda message: self._camera_info_callback(message, "left"),
                qos_profile_sensor_data,
            )
        )

        # Caméra droite
        self._subscriptions.append(
            node.create_subscription(
                Image,
                right_rgb_topic,
                lambda message: self._rgb_callback(message, "right"),
                qos_profile_sensor_data,
            )
        )

        self._subscriptions.append(
            node.create_subscription(
                Image,
                right_depth_topic,
                lambda message: self._depth_callback(message, "right"),
                qos_profile_sensor_data,
            )
        )

        self._subscriptions.append(
            node.create_subscription(
                CameraInfo,
                right_camera_info_topic,
                lambda message: self._camera_info_callback(message, "right"),
                qos_profile_sensor_data,
            )
        )


    # ======================================================================
    # Callbacks caméra
    # ======================================================================
    def _rgb_callback(self, message: Image, camera_name: str) -> None:
        self._camera_data[camera_name]["rgb"] = message

        self._logger.debug(
            "%s RGB received: %dx%d, encoding=%s",
            camera_name,
            message.width,
            message.height,
            message.encoding,
        )


    def _depth_callback(self, message: Image, camera_name: str) -> None:
        self._camera_data[camera_name]["depth"] = message

        self._logger.debug(
            "%s depth received: %dx%d, encoding=%s",
            camera_name,
            message.width,
            message.height,
            message.encoding,
        )


    def _camera_info_callback(self, message: CameraInfo, camera_name: str) -> None:
        self._camera_data[camera_name]["camera_info"] = message

        self._logger.debug(
            "%s CameraInfo received: %dx%d, frame=%s",
            camera_name,
            message.width,
            message.height,
            message.header.frame_id,
        )

    # ======================================================================
    # Initialisation de Spot
    # ======================================================================

    def initialize_robot(self) -> bool:
        self._logger.info("Claiming robot")
        result = self._robot.command("claim")

        if not result.success:
            self._logger.error("Unable to claim robot: %s", result.message)
            return False

        self._logger.info("Powering robot on")
        result = self._robot.command("power_on")

        if not result.success:
            self._logger.error("Unable to power on robot: %s", result.message)
            return False

        self._logger.info("Standing robot")
        result = self._robot.command("stand")

        if not result.success:
            self._logger.error("Unable to stand robot: %s", result.message)
            return False

        self._logger.info("Successfully stood up")
        return True

    # ======================================================================
    # Déplacement
    # ======================================================================

    def relative_move_then_pose(self, dx: float, dy: float, dyaw: float, dz: float, roll: float, pitch: float, yaw: float, frame_name: str, stairs: bool) -> None:
        body_frame = namespace_with(self._robot_name, BODY_FRAME_NAME)
        target_frame = namespace_with(self._robot_name, frame_name)

        self._logger.info("Waiting for TF: %s -> %s",target_frame, body_frame)

        self._tf_listener.wait_for_a_tform_b(target_frame, body_frame)

        frame_t_body = self._tf_listener.lookup_a_tform_b(target_frame, body_frame)

        x = frame_t_body.transform.translation.x
        y = frame_t_body.transform.translation.y
        z = frame_t_body.transform.translation.z

        self._logger.info("Position Spot initiale: x=%.3f, y=%.3f, z=%.3f",x,y,z)

        frame_t_body_se2 = SE3Pose(
            frame_t_body.transform.translation.x,
            frame_t_body.transform.translation.y,
            frame_t_body.transform.translation.z,
            Quat(
                frame_t_body.transform.rotation.w,
                frame_t_body.transform.rotation.x,
                frame_t_body.transform.rotation.y,
                frame_t_body.transform.rotation.z,
            ),
        ).get_closest_se2_transform()

        body_t_goal = SE2Pose(dx, dy, dyaw)
        frame_t_goal = frame_t_body_se2 * body_t_goal

        self._logger.info("Walking to relative goal: dx=%.3f, dy=%.3f, dyaw=%.2f deg",dx,dy,math.degrees(dyaw))

        walk_command = (
            RobotCommandBuilder.synchro_se2_trajectory_point_command(
                goal_x=frame_t_goal.x,
                goal_y=frame_t_goal.y,
                goal_heading=frame_t_goal.angle,
                frame_name=frame_name,
                params=RobotCommandBuilder.mobility_params(
                    stair_hint=stairs
                ),
            )
        )

        walk_goal = RobotCommand.Goal()
        convert(walk_command, walk_goal.command)

        self._robot_command_client.send_goal_and_wait("relative_move",walk_goal)

        self._logger.info("Movement action finished")

        self._logger.info("Applying body pose: dz=%.3f, roll=%.2f deg,pitch=%.2f deg, yaw=%.2f deg",dz,math.degrees(roll), math.degrees(pitch), math.degrees(yaw))

        footprint_R_body = bosdyn.geometry.EulerZXY(yaw=yaw,roll=roll,pitch=pitch,)

        stand_command = RobotCommandBuilder.synchro_stand_command(body_height=dz,footprint_R_body=footprint_R_body)

        stand_goal = RobotCommand.Goal()
        convert(stand_command, stand_goal.command)

        self._robot_command_client.send_goal_and_wait("body_pose", stand_goal)

        # Petit délai pour laisser le corps se stabiliser.
        time.sleep(2.0)

        frame_t_body = self._tf_listener.lookup_a_tform_b(target_frame,body_frame)

        x = frame_t_body.transform.translation.x
        y = frame_t_body.transform.translation.y
        z = frame_t_body.transform.translation.z

        self._logger.info("Position Spot finale: x=%.3f, y=%.3f, z=%.3f",x,y,z)

    # ======================================================================
    # Attente des données caméra
    # ======================================================================

    def wait_for_camera_data(
        self,
        camera_name: str,
        timeout: float = 5.0,
    ) -> bool:
        self._logger.info(
            "Waiting for camera data: %s",
            camera_name,
        )

        start_time = time.monotonic()

        while time.monotonic() - start_time < timeout:
            rclpy.spin_once(
                self._node,
                timeout_sec=0.1,
            )

            camera_data = self._camera_data[camera_name]

            if (
                camera_data["rgb"] is not None
                and camera_data["depth"] is not None
                and camera_data["camera_info"] is not None
            ):
                self._logger.info(
                    "All data received from camera: %s",
                    camera_name,
                )
                return True

        camera_data = self._camera_data[camera_name]

        if camera_data["rgb"] is None:
            self._logger.warning(
                "%s camera: no RGB image received",
                camera_name,
            )

        if camera_data["depth"] is None:
            self._logger.warning(
                "%s camera: no depth image received",
                camera_name,
            )

        if camera_data["camera_info"] is None:
            self._logger.warning(
                "%s camera: no CameraInfo received",
                camera_name,
            )

        return False

    # ======================================================================
    # Profondeur
    # ======================================================================

    def _depth_image_to_meters(self,depth_message: Image,) -> np.ndarray:
        depth_image = self._bridge.imgmsg_to_cv2(depth_message,desired_encoding="passthrough")

        depth_image = np.asarray(depth_image)

        # Images de profondeur uint16 :
        # typiquement millimètres ou unités dépendant du driver.
        if depth_image.dtype == np.uint16:
            depth_meters = (depth_image.astype(np.float32) * self._depth_scale)

        # Images 32FC1 :
        # généralement déjà exprimées en mètres.
        elif depth_image.dtype in (np.float32, np.float64):
            depth_meters = depth_image.astype(np.float32)

        else:
            raise ValueError(
                f"Unsupported depth format: {depth_image.dtype}"
            )

        return depth_meters

    @staticmethod
    def _get_median_depth(depth_image: np.ndarray,center_u: int,center_v: int,radius: int = 5,) -> Optional[float]:
        """
        Calcule la profondeur médiane autour du centre de la détection.

        Utiliser une petite région est plus robuste que lire un seul pixel,
        qui peut être invalide ou bruité.
        """
        height, width = depth_image.shape[:2]

        u_min = max(0, center_u - radius)
        u_max = min(width, center_u + radius + 1)

        v_min = max(0, center_v - radius)
        v_max = min(height, center_v + radius + 1)

        depth_region = depth_image[v_min:v_max, u_min:u_max]

        valid_depths = depth_region[np.isfinite(depth_region)& (depth_region > 0.05)& (depth_region < 5.0)]

        if valid_depths.size == 0:
            return None

        return float(np.median(valid_depths))

    # ======================================================================
    # Projection pixel → caméra 3D
    # ======================================================================

    @staticmethod
    def _pixel_to_camera_point(u: int,v: int,depth: float,camera_info: CameraInfo) -> np.ndarray:
        """
        Projection avec le modèle pinhole :

            X = (u - cx) * Z / fx
            Y = (v - cy) * Z / fy
            Z = profondeur
        """
        fx = camera_info.k[0]
        fy = camera_info.k[4]
        cx = camera_info.k[2]
        cy = camera_info.k[5]

        if fx == 0.0 or fy == 0.0:
            raise ValueError("Invalid CameraInfo: fx or fy is zero")

        x_camera = (u - cx) * depth / fx
        y_camera = (v - cy) * depth / fy
        z_camera = depth

        return np.array([x_camera, y_camera, z_camera],dtype=np.float64)

    # ======================================================================
    # Transformation TF d’un point
    # ======================================================================

    @staticmethod
    def _quaternion_to_rotation_matrix(w: float, x: float,y: float,z: float) -> np.ndarray:
        quaternion_norm = math.sqrt(w * w + x * x + y * y + z * z)

        if quaternion_norm == 0.0:
            raise ValueError("Invalid zero quaternion")

        w /= quaternion_norm
        x /= quaternion_norm
        y /= quaternion_norm
        z /= quaternion_norm

        return np.array(
            [
                [
                    1.0 - 2.0 * (y * y + z * z),
                    2.0 * (x * y - z * w),
                    2.0 * (x * z + y * w),
                ],
                [
                    2.0 * (x * y + z * w),
                    1.0 - 2.0 * (x * x + z * z),
                    2.0 * (y * z - x * w),
                ],
                [
                    2.0 * (x * z - y * w),
                    2.0 * (y * z + x * w),
                    1.0 - 2.0 * (x * x + y * y),
                ],
            ],
            dtype=np.float64,
        )

    def _transform_camera_point(self,point_camera: np.ndarray,camera_frame: str,output_frame: str) -> np.ndarray:
        """
        Transforme un point exprimé dans camera_frame vers output_frame.
        """
        self._logger.info("Waiting for TF: %s <- %s",output_frame,camera_frame,)

        self._tf_listener.wait_for_a_tform_b(output_frame,camera_frame,)

        output_t_camera = self._tf_listener.lookup_a_tform_b(output_frame,camera_frame,)

        translation = np.array(
            [
                output_t_camera.transform.translation.x,
                output_t_camera.transform.translation.y,
                output_t_camera.transform.translation.z,
            ],
            dtype=np.float64,
        )

        quaternion = output_t_camera.transform.rotation

        rotation = self._quaternion_to_rotation_matrix(
            quaternion.w,
            quaternion.x,
            quaternion.y,
            quaternion.z,
        )

        return rotation @ point_camera + translation

    # ======================================================================
    # YOLO
    # ======================================================================
    def _select_valve_detection(self,results) -> Optional[Tuple[int, int, int, int, float, int, str]]:

        detections = results.xyxy[0]

        if detections is None or len(detections) == 0:
            return None

        names = self._model.names
        candidates = []

        for detection in detections:
            x1, y1, x2, y2, confidence, class_id = detection.tolist()

            class_id = int(class_id)
            confidence = float(confidence)
            class_name = str(names[class_id])

            if (self._valve_class_name and class_name.lower() != self._valve_class_name):
                continue

            candidates.append(
                (
                    int(x1),
                    int(y1),
                    int(x2),
                    int(y2),
                    confidence,
                    class_id,
                    class_name,
                )
            )

        if not candidates:
            return None

        return max(candidates, key=lambda detection: detection[4])    
    # ======================================================================
    # Capture + détection + pose 3D
    # ======================================================================

    def _detect_with_camera(
        self,
        camera_name: str,
        output_frame_name: str,
        show_image: bool,
    ) -> Optional[np.ndarray]:

        if not self.wait_for_camera_data(camera_name):
            return None

        camera_data = self._camera_data[camera_name]

        # Copie locale des messages utilisés pour cette tentative.
        rgb_message = camera_data["rgb"]
        depth_message = camera_data["depth"]
        camera_info = camera_data["camera_info"]

        if (
            rgb_message is None
            or depth_message is None
            or camera_info is None
        ):
            self._logger.warning(
                "Incomplete data for camera '%s'",
                camera_name,
            )
            return None

        try:
            rgb_image = self._bridge.imgmsg_to_cv2(
                rgb_message,
                desired_encoding="bgr8",
            )

            depth_image = self._depth_image_to_meters(
                depth_message,
            )

            if rgb_image.shape[:2] != depth_image.shape[:2]:
                self._logger.warning(
                    "%s camera: RGB and depth dimensions differ: "
                    "RGB=%s, depth=%s",
                    camera_name,
                    rgb_image.shape[:2],
                    depth_image.shape[:2],
                )
                return None

            raw_filename = self._build_output_filename(
                self._output_path,
                f"valve_raw_{camera_name}.jpg",
            )

            cv2.imwrite(raw_filename, rgb_image)

            # YOLO
            self._model.conf = self._confidence
            results = self._model(rgb_image)

            if not results:
                self._logger.warning(
                    "%s camera: YOLO returned no result",
                    camera_name,
                )
                return None

            detection = self._select_valve_detection(results)

            if detection is None:
                self._logger.warning(
                    "%s camera: valve not detected",
                    camera_name,
                )

                cv2.putText(
                    rgb_image,
                    f"Vanne non detectee - camera {camera_name}",
                    (20, 40),
                    cv2.FONT_HERSHEY_SIMPLEX,
                    0.8,
                    (0, 0, 255),
                    2,
                    cv2.LINE_AA,
                )

                filename = self._build_output_filename(
                    self._output_path,
                    f"valve_detection_{camera_name}.jpg",
                )

                cv2.imwrite(filename, rgb_image)
                return None

            (
                x1,
                y1,
                x2,
                y2,
                confidence,
                _class_id,
                class_name,
            ) = detection

            center_u = int(round((x1 + x2) / 2.0))
            center_v = int(round((y1 + y2) / 2.0))

            depth = self._get_median_depth(
                depth_image,
                center_u,
                center_v,
                radius=5,
            )

            if depth is None:
                self._logger.warning(
                    "%s camera: no valid depth around (%d, %d)",
                    camera_name,
                    center_u,
                    center_v,
                )

                self._draw_detection(
                    rgb_image,
                    x1,
                    y1,
                    x2,
                    y2,
                    center_u,
                    center_v,
                    class_name,
                    confidence,
                    None,
                    None,
                    output_frame_name,
                )

                filename = self._build_output_filename(
                    self._output_path,
                    f"valve_detection_{camera_name}.jpg",
                )

                cv2.imwrite(filename, rgb_image)
                return None

            point_camera = self._pixel_to_camera_point(
                center_u,
                center_v,
                depth,
                camera_info,
            )

            camera_frame = camera_info.header.frame_id

            if not camera_frame:
                camera_frame = rgb_message.header.frame_id

            if not camera_frame:
                self._logger.warning(
                    "%s camera: empty frame_id",
                    camera_name,
                )
                return None

            # Repère de sortie demandé : body.
            body_frame = namespace_with(
                self._robot_name,
                BODY_FRAME_NAME,
            )

            # Transformation du point :
            # frontleft_fisheye / frontright_fisheye -> body
            point_body = self._transform_camera_point(
                point_camera=point_camera,
                camera_frame=camera_frame,
                output_frame=body_frame,
            )

            self._camera_frame = camera_frame
            self._successful_camera = camera_name
            self._result_frame = body_frame

            
            self._logger.info(
                "Valve in camera frame %s: x=%.3f, y=%.3f, z=%.3f",
                camera_frame,
                point_camera[0],
                point_camera[1],
                point_camera[2],
            )

            self._logger.info(
                "Valve transformed to %s: x=%.3f, y=%.3f, z=%.3f",
                body_frame,
                point_body[0],
                point_body[1],
                point_body[2],
            )

            self._draw_detection(
                rgb_image,
                x1,
                y1,
                x2,
                y2,
                center_u,
                center_v,
                class_name,
                confidence,
                depth,
                point_body,
                body_frame,
            )

            filename = self._build_output_filename(
                self._output_path,
                f"valve_detection_{camera_name}.jpg",
            )

            cv2.imwrite(filename, rgb_image)

            if show_image:
                self._show_image(rgb_image)

            # Important : on retourne le point dans output_frame.
            return point_body

        except Exception:
            self._logger.exception(
                "Detection failed with camera '%s'",
                camera_name,
            )
            return None

    

    def capture_detect_and_localize(
        self,
        output_frame_name: str,
        show_image: bool = True,
    ) -> Optional[np.ndarray]:
        """
        Essaie d'abord la caméra gauche, puis la caméra droite en cas d'échec.
        """
        camera_order = ["left", "right"]

        for camera_name in camera_order:
            self._logger.info(
                "Trying valve detection with camera: %s",
                camera_name,
            )

            point_output = self._detect_with_camera(
                camera_name=camera_name,
                output_frame_name=output_frame_name,
                show_image=show_image,
            )

            if point_output is not None:
                self._logger.info(
                    "Detection succeeded with camera: %s",
                    camera_name,
                )
                return point_output

            self._logger.warning(
                "Detection failed with camera '%s'. "
                "Trying the next camera.",
                camera_name,
            )

        self._logger.error(
            "Unable to detect and localize the valve "
            "with either camera"
        )

        return None
    # ======================================================================
    # Annotation OpenCV
    # ======================================================================

    @staticmethod
    def _draw_detection(image: np.ndarray,x1: int,y1: int,x2: int,y2: int,center_u: int,center_v: int,class_name: str,confidence: float, depth, point_camera, camera_frame) -> None:
        # Bounding box
        cv2.rectangle(image,(x1, y1),(x2, y2),(0, 255, 0),3,)

        # Centre
        cv2.circle(image,(center_u, center_v),7,(0, 0, 255),-1)

        cv2.drawMarker(image,(center_u, center_v),(255, 0, 0),markerType=cv2.MARKER_CROSS,markerSize=25,thickness=2)

        label = f"{class_name}: {confidence:.2f}"

        cv2.putText(image,label,(x1, max(25, y1 - 10)),cv2.FONT_HERSHEY_SIMPLEX,0.7,(0, 255, 0),2,cv2.LINE_AA,)

        center_text = f"centre=({center_u}, {center_v})"

        cv2.putText(
            image,
            center_text,
            (x1, min(image.shape[0] - 45, y2 + 25)),
            cv2.FONT_HERSHEY_SIMPLEX,
            0.6,
            (0, 0, 255),
            2,
            cv2.LINE_AA,
        )

        if depth is not None:
            depth_text = f"depth={depth:.3f} m"

            cv2.putText(
                image,
                depth_text,
                (x1, min(image.shape[0] - 20, y2 + 50)),
                cv2.FONT_HERSHEY_SIMPLEX,
                0.6,
                (255, 255, 0),
                2,
                cv2.LINE_AA,
            )

        if point_camera is not None:
            pose_text = (
                f"{camera_frame}: "
                f"x={point_camera[0]:.2f}, "
                f"y={point_camera[1]:.2f}, "
                f"z={point_camera[2]:.2f} m"
            )

            cv2.putText(
                image,
                pose_text,
                (20, 35),
                cv2.FONT_HERSHEY_SIMPLEX,
                0.65,
                (255, 255, 0),
                2,
                cv2.LINE_AA,
            )

    @staticmethod
    def _show_image(image: np.ndarray) -> None:
        cv2.imshow("Valve detection", image)

        # Attend une touche avant de fermer.
        cv2.waitKey(0)
        cv2.destroyAllWindows()

    @staticmethod
    def _build_output_filename(
        output_directory: str,
        filename: str,
    ) -> str:
        output_directory = os.path.abspath(
            os.path.expanduser(output_directory)
        )

        os.makedirs(output_directory, exist_ok=True)

        return os.path.join(output_directory, filename)


def cli() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()

    parser.add_argument("--robot", type=str, default=None)

    # ----------------------------------------------------------------------
    # Mouvement
    # ----------------------------------------------------------------------
    parser.add_argument("--dx", type=float, default=0.0)
    parser.add_argument("--dy", type=float, default=0.0)
    parser.add_argument("--dyaw", type=float, default=0.0)

    parser.add_argument("--dz", type=float, default=0.0)

    parser.add_argument("--roll", type=float, default=0.0)
    parser.add_argument("--pitch", type=float, default=0.0)
    parser.add_argument("--yaw", type=float, default=0.0)

    parser.add_argument(
        "--frame",
        choices=[VISION_FRAME_NAME, ODOM_FRAME_NAME],
        default=ODOM_FRAME_NAME,
    )

    parser.add_argument("--stairs", action="store_true")

    # ----------------------------------------------------------------------
    # YOLO
    # ----------------------------------------------------------------------
    parser.add_argument(
        "--model",
        type=str,
        required=True,
        help="Chemin du modèle YOLO entraîné, par exemple best.pt",
    )

    parser.add_argument(
        "--confidence",
        type=float,
        default=0.20,
        help="Seuil de confiance YOLO",
    )

    parser.add_argument(
        "--valve-class",
        type=str,
        default="valve",
        help="Nom exact de la classe de vanne dans le modèle YOLO",
    )

    # ----------------------------------------------------------------------
    # Topics caméra
    # À vérifier avec: ros2 topic list
    # ----------------------------------------------------------------------
    parser.add_argument(
        "--left-rgb-topic",
        type=str,
        default="/camera/frontleft/image",
    )

    parser.add_argument(
        "--left-depth-topic",
        type=str,
        default="/depth_registered/frontleft/image",
    )

    parser.add_argument(
        "--left-camera-info-topic",
        type=str,
        default="/camera/frontleft/camera_info",
    )

    parser.add_argument(
        "--right-rgb-topic",
        type=str,
        default="/camera/frontright/image",
    )

    parser.add_argument(
        "--right-depth-topic",
        type=str,
        default="/depth_registered/frontright/image",
    )

    parser.add_argument(
        "--right-camera-info-topic",
        type=str,
        default="/camera/frontright/camera_info",
    )

    parser.add_argument(
        "--depth-scale",
        type=float,
        default=0.001,
        help=(
            "Conversion des valeurs uint16 vers mètres. "
            "0.001 signifie que les valeurs sont en millimètres."
        ),
    )

    parser.add_argument(
        "--output",
        type=str,
        default="./valve_results",
        help="Dossier de sauvegarde des images",
    )

    parser.add_argument(
        "--no-display",
        action="store_true",
        help="Sauvegarder sans ouvrir de fenêtre OpenCV",
    )

    #---------- JSON------------
    parser.add_argument(
    "--output-json",
    type=str,
    default="/tmp/valve_pose_3d.json",
    help="Fichier JSON dans lequel enregistrer la position 3D de la vanne",
)

    return parser


@ros_process.main(cli())
def main(args: argparse.Namespace) -> int:
    commander = RelativeMoveThenDetectValve(
        robot_name=args.robot,
        node=main.node,
        model_path=args.model,
        left_rgb_topic=args.left_rgb_topic,
        left_depth_topic=args.left_depth_topic,
        left_camera_info_topic=args.left_camera_info_topic,
        right_rgb_topic=args.right_rgb_topic,
        right_depth_topic=args.right_depth_topic,
        right_camera_info_topic=args.right_camera_info_topic,
        confidence=args.confidence,
        valve_class_name=args.valve_class,
        output_path=args.output,
        depth_scale=args.depth_scale,
    )

    if not commander.initialize_robot():
        return 1

    commander.relative_move_then_pose(
        dx=args.dx,
        dy=args.dy,
        dyaw=math.radians(args.dyaw),
        dz=args.dz,
        roll=math.radians(args.roll),
        pitch=math.radians(args.pitch),
        yaw=math.radians(args.yaw),
        frame_name=args.frame,
        stairs=args.stairs,
    )

    valve_position = commander.capture_detect_and_localize(
        output_frame_name=BODY_FRAME_NAME,
        show_image=not args.no_display,
    )

    if valve_position is None:
        commander._logger.error(
            "Unable to determine the valve 3D position"
        )
        return 2

    output_json_path = os.path.abspath(
        os.path.expanduser(args.output_json)
    )

    output_directory = os.path.dirname(output_json_path)

    if output_directory:
        os.makedirs(output_directory, exist_ok=True)

    valve_data = {
        "frame": commander._result_frame,
        "camera_used": commander._successful_camera,
        "camera_frame": commander._camera_frame,
        "origin": "body_frame_origin",
        "x": float(valve_position[0]),
        "y": float(valve_position[1]),
        "z": float(valve_position[2]),
    }

    with open(output_json_path, "w", encoding="utf-8") as json_file:
        json.dump(
            valve_data,
            json_file,
            indent=4,
        )

    commander._logger.info(
        "Valve 3D position saved to %s",
        output_json_path,
    )

    print()
    print("Position 3D enregistrée :")
    print(json.dumps(valve_data, indent=4))
    print()

    return 0


if __name__ == "__main__":
    raise SystemExit(main())