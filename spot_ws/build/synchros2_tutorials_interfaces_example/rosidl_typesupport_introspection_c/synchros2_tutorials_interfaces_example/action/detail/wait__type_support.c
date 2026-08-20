// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from synchros2_tutorials_interfaces_example:action/Wait.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"
#include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "synchros2_tutorials_interfaces_example/action/detail/wait__functions.h"
#include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  synchros2_tutorials_interfaces_example__action__Wait_Goal__init(message_memory);
}

void synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_fini_function(void * message_memory)
{
  synchros2_tutorials_interfaces_example__action__Wait_Goal__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_message_member_array[1] = {
  {
    "n_seconds_to_wait",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_Goal, n_seconds_to_wait),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_message_members = {
  "synchros2_tutorials_interfaces_example__action",  // message namespace
  "Wait_Goal",  // message name
  1,  // number of fields
  sizeof(synchros2_tutorials_interfaces_example__action__Wait_Goal),
  synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_message_member_array,  // message members
  synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_init_function,  // function to initialize message memory (memory has to be allocated)
  synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_message_type_support_handle = {
  0,
  &synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_Goal)() {
  if (!synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_message_type_support_handle.typesupport_identifier) {
    synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &synchros2_tutorials_interfaces_example__action__Wait_Goal__rosidl_typesupport_introspection_c__Wait_Goal_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__functions.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  synchros2_tutorials_interfaces_example__action__Wait_Result__init(message_memory);
}

void synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_fini_function(void * message_memory)
{
  synchros2_tutorials_interfaces_example__action__Wait_Result__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_message_member_array[1] = {
  {
    "n_seconds_waited",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_Result, n_seconds_waited),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_message_members = {
  "synchros2_tutorials_interfaces_example__action",  // message namespace
  "Wait_Result",  // message name
  1,  // number of fields
  sizeof(synchros2_tutorials_interfaces_example__action__Wait_Result),
  synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_message_member_array,  // message members
  synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_init_function,  // function to initialize message memory (memory has to be allocated)
  synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_message_type_support_handle = {
  0,
  &synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_Result)() {
  if (!synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_message_type_support_handle.typesupport_identifier) {
    synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &synchros2_tutorials_interfaces_example__action__Wait_Result__rosidl_typesupport_introspection_c__Wait_Result_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__functions.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  synchros2_tutorials_interfaces_example__action__Wait_Feedback__init(message_memory);
}

void synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_fini_function(void * message_memory)
{
  synchros2_tutorials_interfaces_example__action__Wait_Feedback__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_message_member_array[1] = {
  {
    "n_seconds_remaining",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_Feedback, n_seconds_remaining),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_message_members = {
  "synchros2_tutorials_interfaces_example__action",  // message namespace
  "Wait_Feedback",  // message name
  1,  // number of fields
  sizeof(synchros2_tutorials_interfaces_example__action__Wait_Feedback),
  synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_message_member_array,  // message members
  synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_init_function,  // function to initialize message memory (memory has to be allocated)
  synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_message_type_support_handle = {
  0,
  &synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_Feedback)() {
  if (!synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_message_type_support_handle.typesupport_identifier) {
    synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &synchros2_tutorials_interfaces_example__action__Wait_Feedback__rosidl_typesupport_introspection_c__Wait_Feedback_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__functions.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"


// Include directives for member types
// Member `goal_id`
#include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
#include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"
// Member `goal`
#include "synchros2_tutorials_interfaces_example/action/wait.h"
// Member `goal`
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__init(message_memory);
}

void synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_fini_function(void * message_memory)
{
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_member_array[2] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request, goal_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "goal",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request, goal),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_members = {
  "synchros2_tutorials_interfaces_example__action",  // message namespace
  "Wait_SendGoal_Request",  // message name
  2,  // number of fields
  sizeof(synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request),
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_member_array,  // message members
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_type_support_handle = {
  0,
  &synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_SendGoal_Request)() {
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_Goal)();
  if (!synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_type_support_handle.typesupport_identifier) {
    synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__functions.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"


// Include directives for member types
// Member `stamp`
#include "builtin_interfaces/msg/time.h"
// Member `stamp`
#include "builtin_interfaces/msg/detail/time__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__init(message_memory);
}

void synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_fini_function(void * message_memory)
{
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_message_member_array[2] = {
  {
    "accepted",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response, accepted),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "stamp",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response, stamp),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_message_members = {
  "synchros2_tutorials_interfaces_example__action",  // message namespace
  "Wait_SendGoal_Response",  // message name
  2,  // number of fields
  sizeof(synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response),
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_message_member_array,  // message members
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_message_type_support_handle = {
  0,
  &synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_SendGoal_Response)() {
  synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, builtin_interfaces, msg, Time)();
  if (!synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_message_type_support_handle.typesupport_identifier) {
    synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_SendGoal_service_members = {
  "synchros2_tutorials_interfaces_example__action",  // service namespace
  "Wait_SendGoal",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_SendGoal_Request_message_type_support_handle,
  NULL  // response message
  // synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_SendGoal_Response_message_type_support_handle
};

static rosidl_service_type_support_t synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_SendGoal_service_type_support_handle = {
  0,
  &synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_SendGoal_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_SendGoal_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_SendGoal_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_SendGoal)() {
  if (!synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_SendGoal_service_type_support_handle.typesupport_identifier) {
    synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_SendGoal_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_SendGoal_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_SendGoal_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_SendGoal_Response)()->data;
  }

  return &synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_SendGoal_service_type_support_handle;
}

// already included above
// #include <stddef.h>
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__functions.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__init(message_memory);
}

void synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_fini_function(void * message_memory)
{
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_message_member_array[1] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request, goal_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_message_members = {
  "synchros2_tutorials_interfaces_example__action",  // message namespace
  "Wait_GetResult_Request",  // message name
  1,  // number of fields
  sizeof(synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request),
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_message_member_array,  // message members
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_message_type_support_handle = {
  0,
  &synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_GetResult_Request)() {
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  if (!synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_message_type_support_handle.typesupport_identifier) {
    synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__rosidl_typesupport_introspection_c__Wait_GetResult_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__functions.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"


// Include directives for member types
// Member `result`
// already included above
// #include "synchros2_tutorials_interfaces_example/action/wait.h"
// Member `result`
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__init(message_memory);
}

void synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_fini_function(void * message_memory)
{
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_message_member_array[2] = {
  {
    "status",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_INT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response, status),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "result",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response, result),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_message_members = {
  "synchros2_tutorials_interfaces_example__action",  // message namespace
  "Wait_GetResult_Response",  // message name
  2,  // number of fields
  sizeof(synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response),
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_message_member_array,  // message members
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_message_type_support_handle = {
  0,
  &synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_GetResult_Response)() {
  synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_Result)();
  if (!synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_message_type_support_handle.typesupport_identifier) {
    synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__rosidl_typesupport_introspection_c__Wait_GetResult_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_GetResult_service_members = {
  "synchros2_tutorials_interfaces_example__action",  // service namespace
  "Wait_GetResult",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_GetResult_Request_message_type_support_handle,
  NULL  // response message
  // synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_GetResult_Response_message_type_support_handle
};

static rosidl_service_type_support_t synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_GetResult_service_type_support_handle = {
  0,
  &synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_GetResult_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_GetResult_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_GetResult_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_GetResult)() {
  if (!synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_GetResult_service_type_support_handle.typesupport_identifier) {
    synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_GetResult_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_GetResult_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_GetResult_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_GetResult_Response)()->data;
  }

  return &synchros2_tutorials_interfaces_example__action__detail__wait__rosidl_typesupport_introspection_c__Wait_GetResult_service_type_support_handle;
}

// already included above
// #include <stddef.h>
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__functions.h"
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.h"


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"
// Member `feedback`
// already included above
// #include "synchros2_tutorials_interfaces_example/action/wait.h"
// Member `feedback`
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__init(message_memory);
}

void synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_fini_function(void * message_memory)
{
  synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_message_member_array[2] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage, goal_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "feedback",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage, feedback),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_message_members = {
  "synchros2_tutorials_interfaces_example__action",  // message namespace
  "Wait_FeedbackMessage",  // message name
  2,  // number of fields
  sizeof(synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage),
  synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_message_member_array,  // message members
  synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_init_function,  // function to initialize message memory (memory has to be allocated)
  synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_message_type_support_handle = {
  0,
  &synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_FeedbackMessage)() {
  synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, synchros2_tutorials_interfaces_example, action, Wait_Feedback)();
  if (!synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_message_type_support_handle.typesupport_identifier) {
    synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__rosidl_typesupport_introspection_c__Wait_FeedbackMessage_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
