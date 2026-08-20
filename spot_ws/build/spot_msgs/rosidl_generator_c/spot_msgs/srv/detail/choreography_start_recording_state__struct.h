// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:srv/ChoreographyStartRecordingState.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__STRUCT_H_
#define SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'STATUS_UNKNOWN'.
/**
  * Start Replicated Enum
 */
enum
{
  spot_msgs__srv__ChoreographyStartRecordingState_Request__STATUS_UNKNOWN = 0
};

/// Constant 'STATUS_OK'.
enum
{
  spot_msgs__srv__ChoreographyStartRecordingState_Request__STATUS_OK = 1
};

/// Constant 'STATUS_UNKNOWN_RECORDING_SESSION_ID'.
enum
{
  spot_msgs__srv__ChoreographyStartRecordingState_Request__STATUS_UNKNOWN_RECORDING_SESSION_ID = 2
};

/// Constant 'STATUS_RECORDING_BUFFER_FULL'.
enum
{
  spot_msgs__srv__ChoreographyStartRecordingState_Request__STATUS_RECORDING_BUFFER_FULL = 3
};

/// Struct defined in srv/ChoreographyStartRecordingState in the package spot_msgs.
typedef struct spot_msgs__srv__ChoreographyStartRecordingState_Request
{
  /// Start Message
  float duration_seconds;
} spot_msgs__srv__ChoreographyStartRecordingState_Request;

// Struct for a sequence of spot_msgs__srv__ChoreographyStartRecordingState_Request.
typedef struct spot_msgs__srv__ChoreographyStartRecordingState_Request__Sequence
{
  spot_msgs__srv__ChoreographyStartRecordingState_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__ChoreographyStartRecordingState_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/ChoreographyStartRecordingState in the package spot_msgs.
typedef struct spot_msgs__srv__ChoreographyStartRecordingState_Response
{
  bool success;
  rosidl_runtime_c__String message;
  uint8_t status;
  uint64_t recording_session_id;
} spot_msgs__srv__ChoreographyStartRecordingState_Response;

// Struct for a sequence of spot_msgs__srv__ChoreographyStartRecordingState_Response.
typedef struct spot_msgs__srv__ChoreographyStartRecordingState_Response__Sequence
{
  spot_msgs__srv__ChoreographyStartRecordingState_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__ChoreographyStartRecordingState_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__STRUCT_H_
