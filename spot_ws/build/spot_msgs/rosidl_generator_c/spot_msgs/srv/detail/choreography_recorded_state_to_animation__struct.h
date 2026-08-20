// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:srv/ChoreographyRecordedStateToAnimation.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_RECORDED_STATE_TO_ANIMATION__STRUCT_H_
#define SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_RECORDED_STATE_TO_ANIMATION__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/ChoreographyRecordedStateToAnimation in the package spot_msgs.
typedef struct spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request
{
  bool has_arm;
} spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request;

// Struct for a sequence of spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request.
typedef struct spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__Sequence
{
  spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
// Member 'animation_file_contents'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/ChoreographyRecordedStateToAnimation in the package spot_msgs.
typedef struct spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response
{
  bool success;
  rosidl_runtime_c__String message;
  rosidl_runtime_c__String animation_file_contents;
} spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response;

// Struct for a sequence of spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response.
typedef struct spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__Sequence
{
  spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_RECORDED_STATE_TO_ANIMATION__STRUCT_H_
