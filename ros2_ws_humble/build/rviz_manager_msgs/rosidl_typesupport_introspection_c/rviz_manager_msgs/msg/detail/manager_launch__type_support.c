// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from rviz_manager_msgs:msg/ManagerLaunch.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "rviz_manager_msgs/msg/detail/manager_launch__rosidl_typesupport_introspection_c.h"
#include "rviz_manager_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "rviz_manager_msgs/msg/detail/manager_launch__functions.h"
#include "rviz_manager_msgs/msg/detail/manager_launch__struct.h"


// Include directives for member types
// Member `header`
#include "std_msgs/msg/header.h"
// Member `header`
#include "std_msgs/msg/detail/header__rosidl_typesupport_introspection_c.h"
// Member `action`
// Member `ns`
// Member `package`
// Member `executable`
// Member `arguments`
// Member `ros_arguments`
// Member `working_dir`
// Member `session_name`
#include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  rviz_manager_msgs__msg__ManagerLaunch__init(message_memory);
}

void rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_fini_function(void * message_memory)
{
  rviz_manager_msgs__msg__ManagerLaunch__fini(message_memory);
}

size_t rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__size_function__ManagerLaunch__arguments(
  const void * untyped_member)
{
  const rosidl_runtime_c__String__Sequence * member =
    (const rosidl_runtime_c__String__Sequence *)(untyped_member);
  return member->size;
}

const void * rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_const_function__ManagerLaunch__arguments(
  const void * untyped_member, size_t index)
{
  const rosidl_runtime_c__String__Sequence * member =
    (const rosidl_runtime_c__String__Sequence *)(untyped_member);
  return &member->data[index];
}

void * rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_function__ManagerLaunch__arguments(
  void * untyped_member, size_t index)
{
  rosidl_runtime_c__String__Sequence * member =
    (rosidl_runtime_c__String__Sequence *)(untyped_member);
  return &member->data[index];
}

void rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__fetch_function__ManagerLaunch__arguments(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const rosidl_runtime_c__String * item =
    ((const rosidl_runtime_c__String *)
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_const_function__ManagerLaunch__arguments(untyped_member, index));
  rosidl_runtime_c__String * value =
    (rosidl_runtime_c__String *)(untyped_value);
  *value = *item;
}

void rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__assign_function__ManagerLaunch__arguments(
  void * untyped_member, size_t index, const void * untyped_value)
{
  rosidl_runtime_c__String * item =
    ((rosidl_runtime_c__String *)
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_function__ManagerLaunch__arguments(untyped_member, index));
  const rosidl_runtime_c__String * value =
    (const rosidl_runtime_c__String *)(untyped_value);
  *item = *value;
}

bool rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__resize_function__ManagerLaunch__arguments(
  void * untyped_member, size_t size)
{
  rosidl_runtime_c__String__Sequence * member =
    (rosidl_runtime_c__String__Sequence *)(untyped_member);
  rosidl_runtime_c__String__Sequence__fini(member);
  return rosidl_runtime_c__String__Sequence__init(member, size);
}

size_t rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__size_function__ManagerLaunch__ros_arguments(
  const void * untyped_member)
{
  const rosidl_runtime_c__String__Sequence * member =
    (const rosidl_runtime_c__String__Sequence *)(untyped_member);
  return member->size;
}

const void * rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_const_function__ManagerLaunch__ros_arguments(
  const void * untyped_member, size_t index)
{
  const rosidl_runtime_c__String__Sequence * member =
    (const rosidl_runtime_c__String__Sequence *)(untyped_member);
  return &member->data[index];
}

void * rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_function__ManagerLaunch__ros_arguments(
  void * untyped_member, size_t index)
{
  rosidl_runtime_c__String__Sequence * member =
    (rosidl_runtime_c__String__Sequence *)(untyped_member);
  return &member->data[index];
}

void rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__fetch_function__ManagerLaunch__ros_arguments(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const rosidl_runtime_c__String * item =
    ((const rosidl_runtime_c__String *)
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_const_function__ManagerLaunch__ros_arguments(untyped_member, index));
  rosidl_runtime_c__String * value =
    (rosidl_runtime_c__String *)(untyped_value);
  *value = *item;
}

void rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__assign_function__ManagerLaunch__ros_arguments(
  void * untyped_member, size_t index, const void * untyped_value)
{
  rosidl_runtime_c__String * item =
    ((rosidl_runtime_c__String *)
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_function__ManagerLaunch__ros_arguments(untyped_member, index));
  const rosidl_runtime_c__String * value =
    (const rosidl_runtime_c__String *)(untyped_value);
  *item = *value;
}

bool rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__resize_function__ManagerLaunch__ros_arguments(
  void * untyped_member, size_t size)
{
  rosidl_runtime_c__String__Sequence * member =
    (rosidl_runtime_c__String__Sequence *)(untyped_member);
  rosidl_runtime_c__String__Sequence__fini(member);
  return rosidl_runtime_c__String__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_message_member_array[14] = {
  {
    "header",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, header),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_INT32,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "action",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, action),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "ns",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, ns),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "bash_session",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, bash_session),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "is_launch_file",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, is_launch_file),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "package",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, package),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "executable",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, executable),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "arguments",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, arguments),  // bytes offset in struct
    NULL,  // default value
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__size_function__ManagerLaunch__arguments,  // size() function pointer
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_const_function__ManagerLaunch__arguments,  // get_const(index) function pointer
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_function__ManagerLaunch__arguments,  // get(index) function pointer
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__fetch_function__ManagerLaunch__arguments,  // fetch(index, &value) function pointer
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__assign_function__ManagerLaunch__arguments,  // assign(index, value) function pointer
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__resize_function__ManagerLaunch__arguments  // resize(index) function pointer
  },
  {
    "ros_arguments",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, ros_arguments),  // bytes offset in struct
    NULL,  // default value
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__size_function__ManagerLaunch__ros_arguments,  // size() function pointer
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_const_function__ManagerLaunch__ros_arguments,  // get_const(index) function pointer
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__get_function__ManagerLaunch__ros_arguments,  // get(index) function pointer
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__fetch_function__ManagerLaunch__ros_arguments,  // fetch(index, &value) function pointer
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__assign_function__ManagerLaunch__ros_arguments,  // assign(index, value) function pointer
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__resize_function__ManagerLaunch__ros_arguments  // resize(index) function pointer
  },
  {
    "working_dir",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, working_dir),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "session_name",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, session_name),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "use_sim_time",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, use_sim_time),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "timeout",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_INT32,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(rviz_manager_msgs__msg__ManagerLaunch, timeout),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_message_members = {
  "rviz_manager_msgs__msg",  // message namespace
  "ManagerLaunch",  // message name
  14,  // number of fields
  sizeof(rviz_manager_msgs__msg__ManagerLaunch),
  rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_message_member_array,  // message members
  rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_init_function,  // function to initialize message memory (memory has to be allocated)
  rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_message_type_support_handle = {
  0,
  &rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_rviz_manager_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, rviz_manager_msgs, msg, ManagerLaunch)() {
  rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, std_msgs, msg, Header)();
  if (!rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_message_type_support_handle.typesupport_identifier) {
    rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &rviz_manager_msgs__msg__ManagerLaunch__rosidl_typesupport_introspection_c__ManagerLaunch_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
