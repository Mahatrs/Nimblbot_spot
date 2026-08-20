// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:srv/MutateWorldObject.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__MUTATE_WORLD_OBJECT__STRUCT_H_
#define SPOT_MSGS__SRV__DETAIL__MUTATE_WORLD_OBJECT__STRUCT_H_

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
#include "bosdyn_api_msgs/msg/detail/mutate_world_object_request__struct.h"

/// Struct defined in srv/MutateWorldObject in the package spot_msgs.
typedef struct spot_msgs__srv__MutateWorldObject_Request
{
  bosdyn_api_msgs__msg__MutateWorldObjectRequest request;
} spot_msgs__srv__MutateWorldObject_Request;

// Struct for a sequence of spot_msgs__srv__MutateWorldObject_Request.
typedef struct spot_msgs__srv__MutateWorldObject_Request__Sequence
{
  spot_msgs__srv__MutateWorldObject_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__MutateWorldObject_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'response'
#include "bosdyn_api_msgs/msg/detail/mutate_world_object_response__struct.h"

/// Struct defined in srv/MutateWorldObject in the package spot_msgs.
typedef struct spot_msgs__srv__MutateWorldObject_Response
{
  bosdyn_api_msgs__msg__MutateWorldObjectResponse response;
} spot_msgs__srv__MutateWorldObject_Response;

// Struct for a sequence of spot_msgs__srv__MutateWorldObject_Response.
typedef struct spot_msgs__srv__MutateWorldObject_Response__Sequence
{
  spot_msgs__srv__MutateWorldObject_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__MutateWorldObject_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__SRV__DETAIL__MUTATE_WORLD_OBJECT__STRUCT_H_
