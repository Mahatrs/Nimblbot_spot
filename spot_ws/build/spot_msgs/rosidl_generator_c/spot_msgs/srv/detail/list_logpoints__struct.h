// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:srv/ListLogpoints.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__LIST_LOGPOINTS__STRUCT_H_
#define SPOT_MSGS__SRV__DETAIL__LIST_LOGPOINTS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/ListLogpoints in the package spot_msgs.
typedef struct spot_msgs__srv__ListLogpoints_Request
{
  uint8_t structure_needs_at_least_one_member;
} spot_msgs__srv__ListLogpoints_Request;

// Struct for a sequence of spot_msgs__srv__ListLogpoints_Request.
typedef struct spot_msgs__srv__ListLogpoints_Request__Sequence
{
  spot_msgs__srv__ListLogpoints_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__ListLogpoints_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
#include "rosidl_runtime_c/string.h"
// Member 'logpoints'
#include "bosdyn_spot_cam_api_msgs/msg/detail/logpoint__struct.h"

/// Struct defined in srv/ListLogpoints in the package spot_msgs.
typedef struct spot_msgs__srv__ListLogpoints_Response
{
  bool success;
  rosidl_runtime_c__String message;
  bosdyn_spot_cam_api_msgs__msg__Logpoint__Sequence logpoints;
} spot_msgs__srv__ListLogpoints_Response;

// Struct for a sequence of spot_msgs__srv__ListLogpoints_Response.
typedef struct spot_msgs__srv__ListLogpoints_Response__Sequence
{
  spot_msgs__srv__ListLogpoints_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__ListLogpoints_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__SRV__DETAIL__LIST_LOGPOINTS__STRUCT_H_
