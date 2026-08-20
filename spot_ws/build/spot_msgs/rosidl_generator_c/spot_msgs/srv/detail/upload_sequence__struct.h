// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:srv/UploadSequence.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__UPLOAD_SEQUENCE__STRUCT_H_
#define SPOT_MSGS__SRV__DETAIL__UPLOAD_SEQUENCE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'sequence_proto_serialized'
#include "rosidl_runtime_c/primitives_sequence.h"

/// Struct defined in srv/UploadSequence in the package spot_msgs.
typedef struct spot_msgs__srv__UploadSequence_Request
{
  rosidl_runtime_c__uint8__Sequence sequence_proto_serialized;
} spot_msgs__srv__UploadSequence_Request;

// Struct for a sequence of spot_msgs__srv__UploadSequence_Request.
typedef struct spot_msgs__srv__UploadSequence_Request__Sequence
{
  spot_msgs__srv__UploadSequence_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__UploadSequence_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/UploadSequence in the package spot_msgs.
typedef struct spot_msgs__srv__UploadSequence_Response
{
  bool success;
  rosidl_runtime_c__String message;
} spot_msgs__srv__UploadSequence_Response;

// Struct for a sequence of spot_msgs__srv__UploadSequence_Response.
typedef struct spot_msgs__srv__UploadSequence_Response__Sequence
{
  spot_msgs__srv__UploadSequence_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__UploadSequence_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__SRV__DETAIL__UPLOAD_SEQUENCE__STRUCT_H_
