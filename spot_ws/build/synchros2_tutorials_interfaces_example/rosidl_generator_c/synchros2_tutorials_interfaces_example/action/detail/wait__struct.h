// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from synchros2_tutorials_interfaces_example:action/Wait.idl
// generated code does not contain a copyright notice

#ifndef SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__ACTION__DETAIL__WAIT__STRUCT_H_
#define SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__ACTION__DETAIL__WAIT__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in action/Wait in the package synchros2_tutorials_interfaces_example.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_Goal
{
  float n_seconds_to_wait;
} synchros2_tutorials_interfaces_example__action__Wait_Goal;

// Struct for a sequence of synchros2_tutorials_interfaces_example__action__Wait_Goal.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_Goal__Sequence
{
  synchros2_tutorials_interfaces_example__action__Wait_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} synchros2_tutorials_interfaces_example__action__Wait_Goal__Sequence;


// Constants defined in the message

/// Struct defined in action/Wait in the package synchros2_tutorials_interfaces_example.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_Result
{
  float n_seconds_waited;
} synchros2_tutorials_interfaces_example__action__Wait_Result;

// Struct for a sequence of synchros2_tutorials_interfaces_example__action__Wait_Result.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_Result__Sequence
{
  synchros2_tutorials_interfaces_example__action__Wait_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} synchros2_tutorials_interfaces_example__action__Wait_Result__Sequence;


// Constants defined in the message

/// Struct defined in action/Wait in the package synchros2_tutorials_interfaces_example.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_Feedback
{
  float n_seconds_remaining;
} synchros2_tutorials_interfaces_example__action__Wait_Feedback;

// Struct for a sequence of synchros2_tutorials_interfaces_example__action__Wait_Feedback.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_Feedback__Sequence
{
  synchros2_tutorials_interfaces_example__action__Wait_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} synchros2_tutorials_interfaces_example__action__Wait_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"

/// Struct defined in action/Wait in the package synchros2_tutorials_interfaces_example.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  synchros2_tutorials_interfaces_example__action__Wait_Goal goal;
} synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request;

// Struct for a sequence of synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__Sequence
{
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/Wait in the package synchros2_tutorials_interfaces_example.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response;

// Struct for a sequence of synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__Sequence
{
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/Wait in the package synchros2_tutorials_interfaces_example.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request;

// Struct for a sequence of synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__Sequence
{
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"

/// Struct defined in action/Wait in the package synchros2_tutorials_interfaces_example.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response
{
  int8_t status;
  synchros2_tutorials_interfaces_example__action__Wait_Result result;
} synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response;

// Struct for a sequence of synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__Sequence
{
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"

/// Struct defined in action/Wait in the package synchros2_tutorials_interfaces_example.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  synchros2_tutorials_interfaces_example__action__Wait_Feedback feedback;
} synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage;

// Struct for a sequence of synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage.
typedef struct synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__Sequence
{
  synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__ACTION__DETAIL__WAIT__STRUCT_H_
