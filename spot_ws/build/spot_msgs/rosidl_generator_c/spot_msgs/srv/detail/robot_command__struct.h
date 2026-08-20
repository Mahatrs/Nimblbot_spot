// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:srv/RobotCommand.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__STRUCT_H_
#define SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'command'
#include "bosdyn_api_msgs/msg/detail/robot_command__struct.h"
// Member 'duration'
#include "builtin_interfaces/msg/detail/duration__struct.h"

/// Struct defined in srv/RobotCommand in the package spot_msgs.
typedef struct spot_msgs__srv__RobotCommand_Request
{
  bosdyn_api_msgs__msg__RobotCommand command;
  builtin_interfaces__msg__Duration duration;
} spot_msgs__srv__RobotCommand_Request;

// Struct for a sequence of spot_msgs__srv__RobotCommand_Request.
typedef struct spot_msgs__srv__RobotCommand_Request__Sequence
{
  spot_msgs__srv__RobotCommand_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__RobotCommand_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/RobotCommand in the package spot_msgs.
typedef struct spot_msgs__srv__RobotCommand_Response
{
  bool success;
  rosidl_runtime_c__String message;
  uint32_t robot_command_id;
} spot_msgs__srv__RobotCommand_Response;

// Struct for a sequence of spot_msgs__srv__RobotCommand_Response.
typedef struct spot_msgs__srv__RobotCommand_Response__Sequence
{
  spot_msgs__srv__RobotCommand_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__RobotCommand_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__STRUCT_H_
