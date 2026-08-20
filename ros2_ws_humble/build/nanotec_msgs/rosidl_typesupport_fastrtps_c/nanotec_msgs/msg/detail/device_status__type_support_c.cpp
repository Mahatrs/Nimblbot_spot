// generated from rosidl_typesupport_fastrtps_c/resource/idl__type_support_c.cpp.em
// with input from nanotec_msgs:msg/DeviceStatus.idl
// generated code does not contain a copyright notice
#include "nanotec_msgs/msg/detail/device_status__rosidl_typesupport_fastrtps_c.h"


#include <cassert>
#include <limits>
#include <string>
#include "rosidl_typesupport_fastrtps_c/identifier.h"
#include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "nanotec_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "nanotec_msgs/msg/detail/device_status__struct.h"
#include "nanotec_msgs/msg/detail/device_status__functions.h"
#include "fastcdr/Cdr.h"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

// includes and forward declarations of message dependencies and their conversion functions

#if defined(__cplusplus)
extern "C"
{
#endif

#include "rosidl_runtime_c/string.h"  // name
#include "rosidl_runtime_c/string_functions.h"  // name

// forward declare type support functions


using _DeviceStatus__ros_msg_type = nanotec_msgs__msg__DeviceStatus;

static bool _DeviceStatus__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _DeviceStatus__ros_msg_type * ros_message = static_cast<const _DeviceStatus__ros_msg_type *>(untyped_ros_message);
  // Field name: name
  {
    const rosidl_runtime_c__String * str = &ros_message->name;
    if (str->capacity == 0 || str->capacity <= str->size) {
      fprintf(stderr, "string capacity not greater than size\n");
      return false;
    }
    if (str->data[str->size] != '\0') {
      fprintf(stderr, "string not null-terminated\n");
      return false;
    }
    cdr << str->data;
  }

  // Field name: voltage_power
  {
    cdr << ros_message->voltage_power;
  }

  // Field name: voltage_logic
  {
    cdr << ros_message->voltage_logic;
  }

  // Field name: temperature_motor
  {
    cdr << ros_message->temperature_motor;
  }

  // Field name: temperature_micro_chip
  {
    cdr << ros_message->temperature_micro_chip;
  }

  // Field name: ready_to_switch_on
  {
    cdr << (ros_message->ready_to_switch_on ? true : false);
  }

  // Field name: switched_on
  {
    cdr << (ros_message->switched_on ? true : false);
  }

  // Field name: operation_enabled
  {
    cdr << (ros_message->operation_enabled ? true : false);
  }

  // Field name: fault
  {
    cdr << (ros_message->fault ? true : false);
  }

  // Field name: voltage_enabled
  {
    cdr << (ros_message->voltage_enabled ? true : false);
  }

  // Field name: quick_stop
  {
    cdr << (ros_message->quick_stop ? true : false);
  }

  // Field name: switch_on_disabled
  {
    cdr << (ros_message->switch_on_disabled ? true : false);
  }

  // Field name: warning
  {
    cdr << (ros_message->warning ? true : false);
  }

  // Field name: target_reached
  {
    cdr << (ros_message->target_reached ? true : false);
  }

  // Field name: internal_limit_active
  {
    cdr << (ros_message->internal_limit_active ? true : false);
  }

  // Field name: operation_mode_specific
  {
    cdr << ros_message->operation_mode_specific;
  }

  // Field name: homing_status
  {
    cdr << ros_message->homing_status;
  }

  return true;
}

static bool _DeviceStatus__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _DeviceStatus__ros_msg_type * ros_message = static_cast<_DeviceStatus__ros_msg_type *>(untyped_ros_message);
  // Field name: name
  {
    std::string tmp;
    cdr >> tmp;
    if (!ros_message->name.data) {
      rosidl_runtime_c__String__init(&ros_message->name);
    }
    bool succeeded = rosidl_runtime_c__String__assign(
      &ros_message->name,
      tmp.c_str());
    if (!succeeded) {
      fprintf(stderr, "failed to assign string into field 'name'\n");
      return false;
    }
  }

  // Field name: voltage_power
  {
    cdr >> ros_message->voltage_power;
  }

  // Field name: voltage_logic
  {
    cdr >> ros_message->voltage_logic;
  }

  // Field name: temperature_motor
  {
    cdr >> ros_message->temperature_motor;
  }

  // Field name: temperature_micro_chip
  {
    cdr >> ros_message->temperature_micro_chip;
  }

  // Field name: ready_to_switch_on
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->ready_to_switch_on = tmp ? true : false;
  }

  // Field name: switched_on
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->switched_on = tmp ? true : false;
  }

  // Field name: operation_enabled
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->operation_enabled = tmp ? true : false;
  }

  // Field name: fault
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->fault = tmp ? true : false;
  }

  // Field name: voltage_enabled
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->voltage_enabled = tmp ? true : false;
  }

  // Field name: quick_stop
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->quick_stop = tmp ? true : false;
  }

  // Field name: switch_on_disabled
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->switch_on_disabled = tmp ? true : false;
  }

  // Field name: warning
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->warning = tmp ? true : false;
  }

  // Field name: target_reached
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->target_reached = tmp ? true : false;
  }

  // Field name: internal_limit_active
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->internal_limit_active = tmp ? true : false;
  }

  // Field name: operation_mode_specific
  {
    cdr >> ros_message->operation_mode_specific;
  }

  // Field name: homing_status
  {
    cdr >> ros_message->homing_status;
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_nanotec_msgs
size_t get_serialized_size_nanotec_msgs__msg__DeviceStatus(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _DeviceStatus__ros_msg_type * ros_message = static_cast<const _DeviceStatus__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name name
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message->name.size + 1);
  // field.name voltage_power
  {
    size_t item_size = sizeof(ros_message->voltage_power);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name voltage_logic
  {
    size_t item_size = sizeof(ros_message->voltage_logic);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name temperature_motor
  {
    size_t item_size = sizeof(ros_message->temperature_motor);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name temperature_micro_chip
  {
    size_t item_size = sizeof(ros_message->temperature_micro_chip);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name ready_to_switch_on
  {
    size_t item_size = sizeof(ros_message->ready_to_switch_on);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name switched_on
  {
    size_t item_size = sizeof(ros_message->switched_on);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name operation_enabled
  {
    size_t item_size = sizeof(ros_message->operation_enabled);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name fault
  {
    size_t item_size = sizeof(ros_message->fault);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name voltage_enabled
  {
    size_t item_size = sizeof(ros_message->voltage_enabled);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name quick_stop
  {
    size_t item_size = sizeof(ros_message->quick_stop);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name switch_on_disabled
  {
    size_t item_size = sizeof(ros_message->switch_on_disabled);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name warning
  {
    size_t item_size = sizeof(ros_message->warning);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name target_reached
  {
    size_t item_size = sizeof(ros_message->target_reached);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name internal_limit_active
  {
    size_t item_size = sizeof(ros_message->internal_limit_active);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name operation_mode_specific
  {
    size_t item_size = sizeof(ros_message->operation_mode_specific);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name homing_status
  {
    size_t item_size = sizeof(ros_message->homing_status);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

static uint32_t _DeviceStatus__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_nanotec_msgs__msg__DeviceStatus(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_nanotec_msgs
size_t max_serialized_size_nanotec_msgs__msg__DeviceStatus(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  size_t last_member_size = 0;
  (void)last_member_size;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;

  // member: name
  {
    size_t array_size = 1;

    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }
  // member: voltage_power
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }
  // member: voltage_logic
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }
  // member: temperature_motor
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }
  // member: temperature_micro_chip
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }
  // member: ready_to_switch_on
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: switched_on
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: operation_enabled
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: fault
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: voltage_enabled
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: quick_stop
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: switch_on_disabled
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: warning
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: target_reached
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: internal_limit_active
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: operation_mode_specific
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }
  // member: homing_status
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }

  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = nanotec_msgs__msg__DeviceStatus;
    is_plain =
      (
      offsetof(DataType, homing_status) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static size_t _DeviceStatus__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_nanotec_msgs__msg__DeviceStatus(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_DeviceStatus = {
  "nanotec_msgs::msg",
  "DeviceStatus",
  _DeviceStatus__cdr_serialize,
  _DeviceStatus__cdr_deserialize,
  _DeviceStatus__get_serialized_size,
  _DeviceStatus__max_serialized_size
};

static rosidl_message_type_support_t _DeviceStatus__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_DeviceStatus,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, nanotec_msgs, msg, DeviceStatus)() {
  return &_DeviceStatus__type_support;
}

#if defined(__cplusplus)
}
#endif
