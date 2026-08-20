// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from spot_msgs:msg/JointCommand.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "spot_msgs/msg/detail/joint_command__rosidl_typesupport_introspection_c.h"
#include "spot_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "spot_msgs/msg/detail/joint_command__functions.h"
#include "spot_msgs/msg/detail/joint_command__struct.h"


// Include directives for member types
// Member `name`
#include "rosidl_runtime_c/string_functions.h"
// Member `position`
// Member `velocity`
// Member `effort`
// Member `k_q_p`
// Member `k_qd_p`
#include "rosidl_runtime_c/primitives_sequence_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  spot_msgs__msg__JointCommand__init(message_memory);
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_fini_function(void * message_memory)
{
  spot_msgs__msg__JointCommand__fini(message_memory);
}

size_t spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__name(
  const void * untyped_member)
{
  const rosidl_runtime_c__String__Sequence * member =
    (const rosidl_runtime_c__String__Sequence *)(untyped_member);
  return member->size;
}

const void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__name(
  const void * untyped_member, size_t index)
{
  const rosidl_runtime_c__String__Sequence * member =
    (const rosidl_runtime_c__String__Sequence *)(untyped_member);
  return &member->data[index];
}

void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__name(
  void * untyped_member, size_t index)
{
  rosidl_runtime_c__String__Sequence * member =
    (rosidl_runtime_c__String__Sequence *)(untyped_member);
  return &member->data[index];
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__name(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const rosidl_runtime_c__String * item =
    ((const rosidl_runtime_c__String *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__name(untyped_member, index));
  rosidl_runtime_c__String * value =
    (rosidl_runtime_c__String *)(untyped_value);
  *value = *item;
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__name(
  void * untyped_member, size_t index, const void * untyped_value)
{
  rosidl_runtime_c__String * item =
    ((rosidl_runtime_c__String *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__name(untyped_member, index));
  const rosidl_runtime_c__String * value =
    (const rosidl_runtime_c__String *)(untyped_value);
  *item = *value;
}

bool spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__name(
  void * untyped_member, size_t size)
{
  rosidl_runtime_c__String__Sequence * member =
    (rosidl_runtime_c__String__Sequence *)(untyped_member);
  rosidl_runtime_c__String__Sequence__fini(member);
  return rosidl_runtime_c__String__Sequence__init(member, size);
}

size_t spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__position(
  const void * untyped_member)
{
  const rosidl_runtime_c__double__Sequence * member =
    (const rosidl_runtime_c__double__Sequence *)(untyped_member);
  return member->size;
}

const void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__position(
  const void * untyped_member, size_t index)
{
  const rosidl_runtime_c__double__Sequence * member =
    (const rosidl_runtime_c__double__Sequence *)(untyped_member);
  return &member->data[index];
}

void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__position(
  void * untyped_member, size_t index)
{
  rosidl_runtime_c__double__Sequence * member =
    (rosidl_runtime_c__double__Sequence *)(untyped_member);
  return &member->data[index];
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__position(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const double * item =
    ((const double *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__position(untyped_member, index));
  double * value =
    (double *)(untyped_value);
  *value = *item;
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__position(
  void * untyped_member, size_t index, const void * untyped_value)
{
  double * item =
    ((double *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__position(untyped_member, index));
  const double * value =
    (const double *)(untyped_value);
  *item = *value;
}

bool spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__position(
  void * untyped_member, size_t size)
{
  rosidl_runtime_c__double__Sequence * member =
    (rosidl_runtime_c__double__Sequence *)(untyped_member);
  rosidl_runtime_c__double__Sequence__fini(member);
  return rosidl_runtime_c__double__Sequence__init(member, size);
}

size_t spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__velocity(
  const void * untyped_member)
{
  const rosidl_runtime_c__double__Sequence * member =
    (const rosidl_runtime_c__double__Sequence *)(untyped_member);
  return member->size;
}

const void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__velocity(
  const void * untyped_member, size_t index)
{
  const rosidl_runtime_c__double__Sequence * member =
    (const rosidl_runtime_c__double__Sequence *)(untyped_member);
  return &member->data[index];
}

void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__velocity(
  void * untyped_member, size_t index)
{
  rosidl_runtime_c__double__Sequence * member =
    (rosidl_runtime_c__double__Sequence *)(untyped_member);
  return &member->data[index];
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__velocity(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const double * item =
    ((const double *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__velocity(untyped_member, index));
  double * value =
    (double *)(untyped_value);
  *value = *item;
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__velocity(
  void * untyped_member, size_t index, const void * untyped_value)
{
  double * item =
    ((double *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__velocity(untyped_member, index));
  const double * value =
    (const double *)(untyped_value);
  *item = *value;
}

bool spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__velocity(
  void * untyped_member, size_t size)
{
  rosidl_runtime_c__double__Sequence * member =
    (rosidl_runtime_c__double__Sequence *)(untyped_member);
  rosidl_runtime_c__double__Sequence__fini(member);
  return rosidl_runtime_c__double__Sequence__init(member, size);
}

size_t spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__effort(
  const void * untyped_member)
{
  const rosidl_runtime_c__double__Sequence * member =
    (const rosidl_runtime_c__double__Sequence *)(untyped_member);
  return member->size;
}

const void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__effort(
  const void * untyped_member, size_t index)
{
  const rosidl_runtime_c__double__Sequence * member =
    (const rosidl_runtime_c__double__Sequence *)(untyped_member);
  return &member->data[index];
}

void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__effort(
  void * untyped_member, size_t index)
{
  rosidl_runtime_c__double__Sequence * member =
    (rosidl_runtime_c__double__Sequence *)(untyped_member);
  return &member->data[index];
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__effort(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const double * item =
    ((const double *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__effort(untyped_member, index));
  double * value =
    (double *)(untyped_value);
  *value = *item;
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__effort(
  void * untyped_member, size_t index, const void * untyped_value)
{
  double * item =
    ((double *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__effort(untyped_member, index));
  const double * value =
    (const double *)(untyped_value);
  *item = *value;
}

bool spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__effort(
  void * untyped_member, size_t size)
{
  rosidl_runtime_c__double__Sequence * member =
    (rosidl_runtime_c__double__Sequence *)(untyped_member);
  rosidl_runtime_c__double__Sequence__fini(member);
  return rosidl_runtime_c__double__Sequence__init(member, size);
}

size_t spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__k_q_p(
  const void * untyped_member)
{
  const rosidl_runtime_c__double__Sequence * member =
    (const rosidl_runtime_c__double__Sequence *)(untyped_member);
  return member->size;
}

const void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__k_q_p(
  const void * untyped_member, size_t index)
{
  const rosidl_runtime_c__double__Sequence * member =
    (const rosidl_runtime_c__double__Sequence *)(untyped_member);
  return &member->data[index];
}

void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__k_q_p(
  void * untyped_member, size_t index)
{
  rosidl_runtime_c__double__Sequence * member =
    (rosidl_runtime_c__double__Sequence *)(untyped_member);
  return &member->data[index];
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__k_q_p(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const double * item =
    ((const double *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__k_q_p(untyped_member, index));
  double * value =
    (double *)(untyped_value);
  *value = *item;
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__k_q_p(
  void * untyped_member, size_t index, const void * untyped_value)
{
  double * item =
    ((double *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__k_q_p(untyped_member, index));
  const double * value =
    (const double *)(untyped_value);
  *item = *value;
}

bool spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__k_q_p(
  void * untyped_member, size_t size)
{
  rosidl_runtime_c__double__Sequence * member =
    (rosidl_runtime_c__double__Sequence *)(untyped_member);
  rosidl_runtime_c__double__Sequence__fini(member);
  return rosidl_runtime_c__double__Sequence__init(member, size);
}

size_t spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__k_qd_p(
  const void * untyped_member)
{
  const rosidl_runtime_c__double__Sequence * member =
    (const rosidl_runtime_c__double__Sequence *)(untyped_member);
  return member->size;
}

const void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__k_qd_p(
  const void * untyped_member, size_t index)
{
  const rosidl_runtime_c__double__Sequence * member =
    (const rosidl_runtime_c__double__Sequence *)(untyped_member);
  return &member->data[index];
}

void * spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__k_qd_p(
  void * untyped_member, size_t index)
{
  rosidl_runtime_c__double__Sequence * member =
    (rosidl_runtime_c__double__Sequence *)(untyped_member);
  return &member->data[index];
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__k_qd_p(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const double * item =
    ((const double *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__k_qd_p(untyped_member, index));
  double * value =
    (double *)(untyped_value);
  *value = *item;
}

void spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__k_qd_p(
  void * untyped_member, size_t index, const void * untyped_value)
{
  double * item =
    ((double *)
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__k_qd_p(untyped_member, index));
  const double * value =
    (const double *)(untyped_value);
  *item = *value;
}

bool spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__k_qd_p(
  void * untyped_member, size_t size)
{
  rosidl_runtime_c__double__Sequence * member =
    (rosidl_runtime_c__double__Sequence *)(untyped_member);
  rosidl_runtime_c__double__Sequence__fini(member);
  return rosidl_runtime_c__double__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_message_member_array[6] = {
  {
    "name",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(spot_msgs__msg__JointCommand, name),  // bytes offset in struct
    NULL,  // default value
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__name,  // size() function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__name,  // get_const(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__name,  // get(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__name,  // fetch(index, &value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__name,  // assign(index, value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__name  // resize(index) function pointer
  },
  {
    "position",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(spot_msgs__msg__JointCommand, position),  // bytes offset in struct
    NULL,  // default value
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__position,  // size() function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__position,  // get_const(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__position,  // get(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__position,  // fetch(index, &value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__position,  // assign(index, value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__position  // resize(index) function pointer
  },
  {
    "velocity",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(spot_msgs__msg__JointCommand, velocity),  // bytes offset in struct
    NULL,  // default value
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__velocity,  // size() function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__velocity,  // get_const(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__velocity,  // get(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__velocity,  // fetch(index, &value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__velocity,  // assign(index, value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__velocity  // resize(index) function pointer
  },
  {
    "effort",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(spot_msgs__msg__JointCommand, effort),  // bytes offset in struct
    NULL,  // default value
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__effort,  // size() function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__effort,  // get_const(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__effort,  // get(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__effort,  // fetch(index, &value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__effort,  // assign(index, value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__effort  // resize(index) function pointer
  },
  {
    "k_q_p",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(spot_msgs__msg__JointCommand, k_q_p),  // bytes offset in struct
    NULL,  // default value
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__k_q_p,  // size() function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__k_q_p,  // get_const(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__k_q_p,  // get(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__k_q_p,  // fetch(index, &value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__k_q_p,  // assign(index, value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__k_q_p  // resize(index) function pointer
  },
  {
    "k_qd_p",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(spot_msgs__msg__JointCommand, k_qd_p),  // bytes offset in struct
    NULL,  // default value
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__size_function__JointCommand__k_qd_p,  // size() function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_const_function__JointCommand__k_qd_p,  // get_const(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__get_function__JointCommand__k_qd_p,  // get(index) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__fetch_function__JointCommand__k_qd_p,  // fetch(index, &value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__assign_function__JointCommand__k_qd_p,  // assign(index, value) function pointer
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__resize_function__JointCommand__k_qd_p  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_message_members = {
  "spot_msgs__msg",  // message namespace
  "JointCommand",  // message name
  6,  // number of fields
  sizeof(spot_msgs__msg__JointCommand),
  spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_message_member_array,  // message members
  spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_init_function,  // function to initialize message memory (memory has to be allocated)
  spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_message_type_support_handle = {
  0,
  &spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_spot_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, spot_msgs, msg, JointCommand)() {
  if (!spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_message_type_support_handle.typesupport_identifier) {
    spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &spot_msgs__msg__JointCommand__rosidl_typesupport_introspection_c__JointCommand_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
