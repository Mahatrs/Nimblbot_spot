// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:action/ExecuteDance.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__ACTION__DETAIL__EXECUTE_DANCE__STRUCT_H_
#define SPOT_MSGS__ACTION__DETAIL__EXECUTE_DANCE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'choreo_name'
// Member 'choreo_file_content'
#include "rosidl_runtime_c/string.h"
// Member 'choreo_sequence_serialized'
#include "rosidl_runtime_c/primitives_sequence.h"

/// Struct defined in action/ExecuteDance in the package spot_msgs.
typedef struct spot_msgs__action__ExecuteDance_Goal
{
  rosidl_runtime_c__String choreo_name;
  rosidl_runtime_c__String choreo_file_content;
  rosidl_runtime_c__uint8__Sequence choreo_sequence_serialized;
  uint32_t start_slice;
} spot_msgs__action__ExecuteDance_Goal;

// Struct for a sequence of spot_msgs__action__ExecuteDance_Goal.
typedef struct spot_msgs__action__ExecuteDance_Goal__Sequence
{
  spot_msgs__action__ExecuteDance_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ExecuteDance_Goal__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in action/ExecuteDance in the package spot_msgs.
typedef struct spot_msgs__action__ExecuteDance_Result
{
  bool success;
  rosidl_runtime_c__String message;
} spot_msgs__action__ExecuteDance_Result;

// Struct for a sequence of spot_msgs__action__ExecuteDance_Result.
typedef struct spot_msgs__action__ExecuteDance_Result__Sequence
{
  spot_msgs__action__ExecuteDance_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ExecuteDance_Result__Sequence;


// Constants defined in the message

/// Struct defined in action/ExecuteDance in the package spot_msgs.
typedef struct spot_msgs__action__ExecuteDance_Feedback
{
  bool is_dancing;
} spot_msgs__action__ExecuteDance_Feedback;

// Struct for a sequence of spot_msgs__action__ExecuteDance_Feedback.
typedef struct spot_msgs__action__ExecuteDance_Feedback__Sequence
{
  spot_msgs__action__ExecuteDance_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ExecuteDance_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "spot_msgs/action/detail/execute_dance__struct.h"

/// Struct defined in action/ExecuteDance in the package spot_msgs.
typedef struct spot_msgs__action__ExecuteDance_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  spot_msgs__action__ExecuteDance_Goal goal;
} spot_msgs__action__ExecuteDance_SendGoal_Request;

// Struct for a sequence of spot_msgs__action__ExecuteDance_SendGoal_Request.
typedef struct spot_msgs__action__ExecuteDance_SendGoal_Request__Sequence
{
  spot_msgs__action__ExecuteDance_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ExecuteDance_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/ExecuteDance in the package spot_msgs.
typedef struct spot_msgs__action__ExecuteDance_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} spot_msgs__action__ExecuteDance_SendGoal_Response;

// Struct for a sequence of spot_msgs__action__ExecuteDance_SendGoal_Response.
typedef struct spot_msgs__action__ExecuteDance_SendGoal_Response__Sequence
{
  spot_msgs__action__ExecuteDance_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ExecuteDance_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/ExecuteDance in the package spot_msgs.
typedef struct spot_msgs__action__ExecuteDance_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} spot_msgs__action__ExecuteDance_GetResult_Request;

// Struct for a sequence of spot_msgs__action__ExecuteDance_GetResult_Request.
typedef struct spot_msgs__action__ExecuteDance_GetResult_Request__Sequence
{
  spot_msgs__action__ExecuteDance_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ExecuteDance_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "spot_msgs/action/detail/execute_dance__struct.h"

/// Struct defined in action/ExecuteDance in the package spot_msgs.
typedef struct spot_msgs__action__ExecuteDance_GetResult_Response
{
  int8_t status;
  spot_msgs__action__ExecuteDance_Result result;
} spot_msgs__action__ExecuteDance_GetResult_Response;

// Struct for a sequence of spot_msgs__action__ExecuteDance_GetResult_Response.
typedef struct spot_msgs__action__ExecuteDance_GetResult_Response__Sequence
{
  spot_msgs__action__ExecuteDance_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ExecuteDance_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "spot_msgs/action/detail/execute_dance__struct.h"

/// Struct defined in action/ExecuteDance in the package spot_msgs.
typedef struct spot_msgs__action__ExecuteDance_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  spot_msgs__action__ExecuteDance_Feedback feedback;
} spot_msgs__action__ExecuteDance_FeedbackMessage;

// Struct for a sequence of spot_msgs__action__ExecuteDance_FeedbackMessage.
typedef struct spot_msgs__action__ExecuteDance_FeedbackMessage__Sequence
{
  spot_msgs__action__ExecuteDance_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ExecuteDance_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__ACTION__DETAIL__EXECUTE_DANCE__STRUCT_H_
