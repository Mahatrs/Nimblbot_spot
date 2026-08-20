// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:srv/GetPtzPosition.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__GET_PTZ_POSITION__STRUCT_H_
#define SPOT_MSGS__SRV__DETAIL__GET_PTZ_POSITION__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'name'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/GetPtzPosition in the package spot_msgs.
typedef struct spot_msgs__srv__GetPtzPosition_Request
{
  rosidl_runtime_c__String name;
} spot_msgs__srv__GetPtzPosition_Request;

// Struct for a sequence of spot_msgs__srv__GetPtzPosition_Request.
typedef struct spot_msgs__srv__GetPtzPosition_Request__Sequence
{
  spot_msgs__srv__GetPtzPosition_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__GetPtzPosition_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
// already included above
// #include "rosidl_runtime_c/string.h"
// Member 'position'
#include "bosdyn_spot_cam_api_msgs/msg/detail/ptz_position__struct.h"

/// Struct defined in srv/GetPtzPosition in the package spot_msgs.
typedef struct spot_msgs__srv__GetPtzPosition_Response
{
  bool success;
  rosidl_runtime_c__String message;
  bosdyn_spot_cam_api_msgs__msg__PtzPosition position;
} spot_msgs__srv__GetPtzPosition_Response;

// Struct for a sequence of spot_msgs__srv__GetPtzPosition_Response.
typedef struct spot_msgs__srv__GetPtzPosition_Response__Sequence
{
  spot_msgs__srv__GetPtzPosition_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__GetPtzPosition_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__SRV__DETAIL__GET_PTZ_POSITION__STRUCT_H_
