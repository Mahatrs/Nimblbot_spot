// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:srv/GetInverseKinematicSolutions.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__GET_INVERSE_KINEMATIC_SOLUTIONS__STRUCT_H_
#define SPOT_MSGS__SRV__DETAIL__GET_INVERSE_KINEMATIC_SOLUTIONS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'request'
#include "bosdyn_spot_api_msgs/msg/detail/inverse_kinematics_request__struct.h"

/// Struct defined in srv/GetInverseKinematicSolutions in the package spot_msgs.
typedef struct spot_msgs__srv__GetInverseKinematicSolutions_Request
{
  bosdyn_spot_api_msgs__msg__InverseKinematicsRequest request;
} spot_msgs__srv__GetInverseKinematicSolutions_Request;

// Struct for a sequence of spot_msgs__srv__GetInverseKinematicSolutions_Request.
typedef struct spot_msgs__srv__GetInverseKinematicSolutions_Request__Sequence
{
  spot_msgs__srv__GetInverseKinematicSolutions_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__GetInverseKinematicSolutions_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'response'
#include "bosdyn_spot_api_msgs/msg/detail/inverse_kinematics_response__struct.h"

/// Struct defined in srv/GetInverseKinematicSolutions in the package spot_msgs.
typedef struct spot_msgs__srv__GetInverseKinematicSolutions_Response
{
  bosdyn_spot_api_msgs__msg__InverseKinematicsResponse response;
} spot_msgs__srv__GetInverseKinematicSolutions_Response;

// Struct for a sequence of spot_msgs__srv__GetInverseKinematicSolutions_Response.
typedef struct spot_msgs__srv__GetInverseKinematicSolutions_Response__Sequence
{
  spot_msgs__srv__GetInverseKinematicSolutions_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__GetInverseKinematicSolutions_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__SRV__DETAIL__GET_INVERSE_KINEMATIC_SOLUTIONS__STRUCT_H_
