// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:srv/AcquireLease.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__STRUCT_H_
#define SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'client_name'
// Member 'resource_name'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/AcquireLease in the package spot_msgs.
typedef struct spot_msgs__srv__AcquireLease_Request
{
  rosidl_runtime_c__String client_name;
  rosidl_runtime_c__String resource_name;
  bool force;
} spot_msgs__srv__AcquireLease_Request;

// Struct for a sequence of spot_msgs__srv__AcquireLease_Request.
typedef struct spot_msgs__srv__AcquireLease_Request__Sequence
{
  spot_msgs__srv__AcquireLease_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__AcquireLease_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
// already included above
// #include "rosidl_runtime_c/string.h"
// Member 'lease'
#include "bosdyn_api_msgs/msg/detail/lease__struct.h"

/// Struct defined in srv/AcquireLease in the package spot_msgs.
typedef struct spot_msgs__srv__AcquireLease_Response
{
  bool success;
  rosidl_runtime_c__String message;
  bosdyn_api_msgs__msg__Lease lease;
} spot_msgs__srv__AcquireLease_Response;

// Struct for a sequence of spot_msgs__srv__AcquireLease_Response.
typedef struct spot_msgs__srv__AcquireLease_Response__Sequence
{
  spot_msgs__srv__AcquireLease_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__AcquireLease_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__STRUCT_H_
