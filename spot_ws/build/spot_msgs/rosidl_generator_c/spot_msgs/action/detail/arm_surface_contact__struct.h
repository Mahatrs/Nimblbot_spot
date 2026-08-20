// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:action/ArmSurfaceContact.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__ACTION__DETAIL__ARM_SURFACE_CONTACT__STRUCT_H_
#define SPOT_MSGS__ACTION__DETAIL__ARM_SURFACE_CONTACT__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'command'
#include "bosdyn_api_msgs/msg/detail/arm_surface_contact_request__struct.h"

/// Struct defined in action/ArmSurfaceContact in the package spot_msgs.
typedef struct spot_msgs__action__ArmSurfaceContact_Goal
{
  bosdyn_api_msgs__msg__ArmSurfaceContactRequest command;
} spot_msgs__action__ArmSurfaceContact_Goal;

// Struct for a sequence of spot_msgs__action__ArmSurfaceContact_Goal.
typedef struct spot_msgs__action__ArmSurfaceContact_Goal__Sequence
{
  spot_msgs__action__ArmSurfaceContact_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ArmSurfaceContact_Goal__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
#include "rosidl_runtime_c/string.h"

/// Struct defined in action/ArmSurfaceContact in the package spot_msgs.
typedef struct spot_msgs__action__ArmSurfaceContact_Result
{
  bool success;
  rosidl_runtime_c__String message;
} spot_msgs__action__ArmSurfaceContact_Result;

// Struct for a sequence of spot_msgs__action__ArmSurfaceContact_Result.
typedef struct spot_msgs__action__ArmSurfaceContact_Result__Sequence
{
  spot_msgs__action__ArmSurfaceContact_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ArmSurfaceContact_Result__Sequence;


// Constants defined in the message

/// Struct defined in action/ArmSurfaceContact in the package spot_msgs.
typedef struct spot_msgs__action__ArmSurfaceContact_Feedback
{
  uint8_t structure_needs_at_least_one_member;
} spot_msgs__action__ArmSurfaceContact_Feedback;

// Struct for a sequence of spot_msgs__action__ArmSurfaceContact_Feedback.
typedef struct spot_msgs__action__ArmSurfaceContact_Feedback__Sequence
{
  spot_msgs__action__ArmSurfaceContact_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ArmSurfaceContact_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "spot_msgs/action/detail/arm_surface_contact__struct.h"

/// Struct defined in action/ArmSurfaceContact in the package spot_msgs.
typedef struct spot_msgs__action__ArmSurfaceContact_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  spot_msgs__action__ArmSurfaceContact_Goal goal;
} spot_msgs__action__ArmSurfaceContact_SendGoal_Request;

// Struct for a sequence of spot_msgs__action__ArmSurfaceContact_SendGoal_Request.
typedef struct spot_msgs__action__ArmSurfaceContact_SendGoal_Request__Sequence
{
  spot_msgs__action__ArmSurfaceContact_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ArmSurfaceContact_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/ArmSurfaceContact in the package spot_msgs.
typedef struct spot_msgs__action__ArmSurfaceContact_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} spot_msgs__action__ArmSurfaceContact_SendGoal_Response;

// Struct for a sequence of spot_msgs__action__ArmSurfaceContact_SendGoal_Response.
typedef struct spot_msgs__action__ArmSurfaceContact_SendGoal_Response__Sequence
{
  spot_msgs__action__ArmSurfaceContact_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ArmSurfaceContact_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/ArmSurfaceContact in the package spot_msgs.
typedef struct spot_msgs__action__ArmSurfaceContact_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} spot_msgs__action__ArmSurfaceContact_GetResult_Request;

// Struct for a sequence of spot_msgs__action__ArmSurfaceContact_GetResult_Request.
typedef struct spot_msgs__action__ArmSurfaceContact_GetResult_Request__Sequence
{
  spot_msgs__action__ArmSurfaceContact_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ArmSurfaceContact_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "spot_msgs/action/detail/arm_surface_contact__struct.h"

/// Struct defined in action/ArmSurfaceContact in the package spot_msgs.
typedef struct spot_msgs__action__ArmSurfaceContact_GetResult_Response
{
  int8_t status;
  spot_msgs__action__ArmSurfaceContact_Result result;
} spot_msgs__action__ArmSurfaceContact_GetResult_Response;

// Struct for a sequence of spot_msgs__action__ArmSurfaceContact_GetResult_Response.
typedef struct spot_msgs__action__ArmSurfaceContact_GetResult_Response__Sequence
{
  spot_msgs__action__ArmSurfaceContact_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ArmSurfaceContact_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "spot_msgs/action/detail/arm_surface_contact__struct.h"

/// Struct defined in action/ArmSurfaceContact in the package spot_msgs.
typedef struct spot_msgs__action__ArmSurfaceContact_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  spot_msgs__action__ArmSurfaceContact_Feedback feedback;
} spot_msgs__action__ArmSurfaceContact_FeedbackMessage;

// Struct for a sequence of spot_msgs__action__ArmSurfaceContact_FeedbackMessage.
typedef struct spot_msgs__action__ArmSurfaceContact_FeedbackMessage__Sequence
{
  spot_msgs__action__ArmSurfaceContact_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__action__ArmSurfaceContact_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__ACTION__DETAIL__ARM_SURFACE_CONTACT__STRUCT_H_
