// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from nanotec_msgs:msg/DriverStatus.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "nanotec_msgs/msg/detail/driver_status__rosidl_typesupport_introspection_c.h"
#include "nanotec_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "nanotec_msgs/msg/detail/driver_status__functions.h"
#include "nanotec_msgs/msg/detail/driver_status__struct.h"


// Include directives for member types
// Member `header`
#include "std_msgs/msg/header.h"
// Member `header`
#include "std_msgs/msg/detail/header__rosidl_typesupport_introspection_c.h"
// Member `devices`
#include "nanotec_msgs/msg/device_status.h"
// Member `devices`
#include "nanotec_msgs/msg/detail/device_status__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  nanotec_msgs__msg__DriverStatus__init(message_memory);
}

void nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_fini_function(void * message_memory)
{
  nanotec_msgs__msg__DriverStatus__fini(message_memory);
}

size_t nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__size_function__DriverStatus__devices(
  const void * untyped_member)
{
  const nanotec_msgs__msg__DeviceStatus__Sequence * member =
    (const nanotec_msgs__msg__DeviceStatus__Sequence *)(untyped_member);
  return member->size;
}

const void * nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__get_const_function__DriverStatus__devices(
  const void * untyped_member, size_t index)
{
  const nanotec_msgs__msg__DeviceStatus__Sequence * member =
    (const nanotec_msgs__msg__DeviceStatus__Sequence *)(untyped_member);
  return &member->data[index];
}

void * nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__get_function__DriverStatus__devices(
  void * untyped_member, size_t index)
{
  nanotec_msgs__msg__DeviceStatus__Sequence * member =
    (nanotec_msgs__msg__DeviceStatus__Sequence *)(untyped_member);
  return &member->data[index];
}

void nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__fetch_function__DriverStatus__devices(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const nanotec_msgs__msg__DeviceStatus * item =
    ((const nanotec_msgs__msg__DeviceStatus *)
    nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__get_const_function__DriverStatus__devices(untyped_member, index));
  nanotec_msgs__msg__DeviceStatus * value =
    (nanotec_msgs__msg__DeviceStatus *)(untyped_value);
  *value = *item;
}

void nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__assign_function__DriverStatus__devices(
  void * untyped_member, size_t index, const void * untyped_value)
{
  nanotec_msgs__msg__DeviceStatus * item =
    ((nanotec_msgs__msg__DeviceStatus *)
    nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__get_function__DriverStatus__devices(untyped_member, index));
  const nanotec_msgs__msg__DeviceStatus * value =
    (const nanotec_msgs__msg__DeviceStatus *)(untyped_value);
  *item = *value;
}

bool nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__resize_function__DriverStatus__devices(
  void * untyped_member, size_t size)
{
  nanotec_msgs__msg__DeviceStatus__Sequence * member =
    (nanotec_msgs__msg__DeviceStatus__Sequence *)(untyped_member);
  nanotec_msgs__msg__DeviceStatus__Sequence__fini(member);
  return nanotec_msgs__msg__DeviceStatus__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_message_member_array[2] = {
  {
    "header",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(nanotec_msgs__msg__DriverStatus, header),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "devices",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(nanotec_msgs__msg__DriverStatus, devices),  // bytes offset in struct
    NULL,  // default value
    nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__size_function__DriverStatus__devices,  // size() function pointer
    nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__get_const_function__DriverStatus__devices,  // get_const(index) function pointer
    nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__get_function__DriverStatus__devices,  // get(index) function pointer
    nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__fetch_function__DriverStatus__devices,  // fetch(index, &value) function pointer
    nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__assign_function__DriverStatus__devices,  // assign(index, value) function pointer
    nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__resize_function__DriverStatus__devices  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_message_members = {
  "nanotec_msgs__msg",  // message namespace
  "DriverStatus",  // message name
  2,  // number of fields
  sizeof(nanotec_msgs__msg__DriverStatus),
  nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_message_member_array,  // message members
  nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_init_function,  // function to initialize message memory (memory has to be allocated)
  nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_message_type_support_handle = {
  0,
  &nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_nanotec_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, nanotec_msgs, msg, DriverStatus)() {
  nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, std_msgs, msg, Header)();
  nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, nanotec_msgs, msg, DeviceStatus)();
  if (!nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_message_type_support_handle.typesupport_identifier) {
    nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &nanotec_msgs__msg__DriverStatus__rosidl_typesupport_introspection_c__DriverStatus_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
