// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:srv/GetChoreographyStatus.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__GET_CHOREOGRAPHY_STATUS__STRUCT_H_
#define SPOT_MSGS__SRV__DETAIL__GET_CHOREOGRAPHY_STATUS__STRUCT_H_

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
  spot_msgs__srv__GetChoreographyStatus_Request__STATUS_UNKNOWN = 0
};

/// Constant 'STATUS_DANCING'.
enum
{
  spot_msgs__srv__GetChoreographyStatus_Request__STATUS_DANCING = 1
};

/// Constant 'STATUS_COMPLETED_SEQUENCE'.
enum
{
  spot_msgs__srv__GetChoreographyStatus_Request__STATUS_COMPLETED_SEQUENCE = 2
};

/// Constant 'STATUS_PREPPING'.
enum
{
  spot_msgs__srv__GetChoreographyStatus_Request__STATUS_PREPPING = 3
};

/// Constant 'STATUS_WAITING_FOR_START_TIME'.
enum
{
  spot_msgs__srv__GetChoreographyStatus_Request__STATUS_WAITING_FOR_START_TIME = 4
};

/// Constant 'STATUS_VALIDATING'.
enum
{
  spot_msgs__srv__GetChoreographyStatus_Request__STATUS_VALIDATING = 5
};

/// Constant 'STATUS_INTERRUPTED'.
enum
{
  spot_msgs__srv__GetChoreographyStatus_Request__STATUS_INTERRUPTED = 6
};

/// Constant 'STATUS_FALLEN'.
enum
{
  spot_msgs__srv__GetChoreographyStatus_Request__STATUS_FALLEN = 7
};

/// Constant 'STATUS_POWERED_OFF'.
enum
{
  spot_msgs__srv__GetChoreographyStatus_Request__STATUS_POWERED_OFF = 8
};

/// Constant 'STATUS_OTHER'.
enum
{
  spot_msgs__srv__GetChoreographyStatus_Request__STATUS_OTHER = 9
};

/// Struct defined in srv/GetChoreographyStatus in the package spot_msgs.
typedef struct spot_msgs__srv__GetChoreographyStatus_Request
{
  uint8_t structure_needs_at_least_one_member;
} spot_msgs__srv__GetChoreographyStatus_Request;

// Struct for a sequence of spot_msgs__srv__GetChoreographyStatus_Request.
typedef struct spot_msgs__srv__GetChoreographyStatus_Request__Sequence
{
  spot_msgs__srv__GetChoreographyStatus_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__GetChoreographyStatus_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/GetChoreographyStatus in the package spot_msgs.
typedef struct spot_msgs__srv__GetChoreographyStatus_Response
{
  bool success;
  rosidl_runtime_c__String message;
  uint8_t status;
  int32_t execution_id;
} spot_msgs__srv__GetChoreographyStatus_Response;

// Struct for a sequence of spot_msgs__srv__GetChoreographyStatus_Response.
typedef struct spot_msgs__srv__GetChoreographyStatus_Response__Sequence
{
  spot_msgs__srv__GetChoreographyStatus_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__srv__GetChoreographyStatus_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__SRV__DETAIL__GET_CHOREOGRAPHY_STATUS__STRUCT_H_
