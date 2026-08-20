// generated from rosidl_typesupport_fastrtps_cpp/resource/idl__type_support.cpp.em
// with input from nanotec_msgs:msg/DeviceStatus.idl
// generated code does not contain a copyright notice
#include "nanotec_msgs/msg/detail/device_status__rosidl_typesupport_fastrtps_cpp.hpp"
#include "nanotec_msgs/msg/detail/device_status__struct.hpp"

#include <limits>
#include <stdexcept>
#include <string>
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_fastrtps_cpp/identifier.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support_decl.hpp"
#include "rosidl_typesupport_fastrtps_cpp/wstring_conversion.hpp"
#include "fastcdr/Cdr.h"


// forward declaration of message dependencies and their conversion functions

namespace nanotec_msgs
{

namespace msg
{

namespace typesupport_fastrtps_cpp
{

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_nanotec_msgs
cdr_serialize(
  const nanotec_msgs::msg::DeviceStatus & ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  // Member: name
  cdr << ros_message.name;
  // Member: voltage_power
  cdr << ros_message.voltage_power;
  // Member: voltage_logic
  cdr << ros_message.voltage_logic;
  // Member: temperature_motor
  cdr << ros_message.temperature_motor;
  // Member: temperature_micro_chip
  cdr << ros_message.temperature_micro_chip;
  // Member: ready_to_switch_on
  cdr << (ros_message.ready_to_switch_on ? true : false);
  // Member: switched_on
  cdr << (ros_message.switched_on ? true : false);
  // Member: operation_enabled
  cdr << (ros_message.operation_enabled ? true : false);
  // Member: fault
  cdr << (ros_message.fault ? true : false);
  // Member: voltage_enabled
  cdr << (ros_message.voltage_enabled ? true : false);
  // Member: quick_stop
  cdr << (ros_message.quick_stop ? true : false);
  // Member: switch_on_disabled
  cdr << (ros_message.switch_on_disabled ? true : false);
  // Member: warning
  cdr << (ros_message.warning ? true : false);
  // Member: target_reached
  cdr << (ros_message.target_reached ? true : false);
  // Member: internal_limit_active
  cdr << (ros_message.internal_limit_active ? true : false);
  // Member: operation_mode_specific
  cdr << ros_message.operation_mode_specific;
  // Member: homing_status
  cdr << ros_message.homing_status;
  return true;
}

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_nanotec_msgs
cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  nanotec_msgs::msg::DeviceStatus & ros_message)
{
  // Member: name
  cdr >> ros_message.name;

  // Member: voltage_power
  cdr >> ros_message.voltage_power;

  // Member: voltage_logic
  cdr >> ros_message.voltage_logic;

  // Member: temperature_motor
  cdr >> ros_message.temperature_motor;

  // Member: temperature_micro_chip
  cdr >> ros_message.temperature_micro_chip;

  // Member: ready_to_switch_on
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.ready_to_switch_on = tmp ? true : false;
  }

  // Member: switched_on
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.switched_on = tmp ? true : false;
  }

  // Member: operation_enabled
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.operation_enabled = tmp ? true : false;
  }

  // Member: fault
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.fault = tmp ? true : false;
  }

  // Member: voltage_enabled
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.voltage_enabled = tmp ? true : false;
  }

  // Member: quick_stop
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.quick_stop = tmp ? true : false;
  }

  // Member: switch_on_disabled
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.switch_on_disabled = tmp ? true : false;
  }

  // Member: warning
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.warning = tmp ? true : false;
  }

  // Member: target_reached
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.target_reached = tmp ? true : false;
  }

  // Member: internal_limit_active
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.internal_limit_active = tmp ? true : false;
  }

  // Member: operation_mode_specific
  cdr >> ros_message.operation_mode_specific;

  // Member: homing_status
  cdr >> ros_message.homing_status;

  return true;
}  // NOLINT(readability/fn_size)

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_nanotec_msgs
get_serialized_size(
  const nanotec_msgs::msg::DeviceStatus & ros_message,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // Member: name
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message.name.size() + 1);
  // Member: voltage_power
  {
    size_t item_size = sizeof(ros_message.voltage_power);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: voltage_logic
  {
    size_t item_size = sizeof(ros_message.voltage_logic);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: temperature_motor
  {
    size_t item_size = sizeof(ros_message.temperature_motor);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: temperature_micro_chip
  {
    size_t item_size = sizeof(ros_message.temperature_micro_chip);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: ready_to_switch_on
  {
    size_t item_size = sizeof(ros_message.ready_to_switch_on);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: switched_on
  {
    size_t item_size = sizeof(ros_message.switched_on);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: operation_enabled
  {
    size_t item_size = sizeof(ros_message.operation_enabled);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: fault
  {
    size_t item_size = sizeof(ros_message.fault);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: voltage_enabled
  {
    size_t item_size = sizeof(ros_message.voltage_enabled);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: quick_stop
  {
    size_t item_size = sizeof(ros_message.quick_stop);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: switch_on_disabled
  {
    size_t item_size = sizeof(ros_message.switch_on_disabled);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: warning
  {
    size_t item_size = sizeof(ros_message.warning);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: target_reached
  {
    size_t item_size = sizeof(ros_message.target_reached);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: internal_limit_active
  {
    size_t item_size = sizeof(ros_message.internal_limit_active);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: operation_mode_specific
  {
    size_t item_size = sizeof(ros_message.operation_mode_specific);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: homing_status
  {
    size_t item_size = sizeof(ros_message.homing_status);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_nanotec_msgs
max_serialized_size_DeviceStatus(
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


  // Member: name
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

  // Member: voltage_power
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }

  // Member: voltage_logic
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }

  // Member: temperature_motor
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }

  // Member: temperature_micro_chip
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }

  // Member: ready_to_switch_on
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: switched_on
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: operation_enabled
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: fault
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: voltage_enabled
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: quick_stop
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: switch_on_disabled
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: warning
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: target_reached
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: internal_limit_active
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: operation_mode_specific
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }

  // Member: homing_status
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
    using DataType = nanotec_msgs::msg::DeviceStatus;
    is_plain =
      (
      offsetof(DataType, homing_status) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static bool _DeviceStatus__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  auto typed_message =
    static_cast<const nanotec_msgs::msg::DeviceStatus *>(
    untyped_ros_message);
  return cdr_serialize(*typed_message, cdr);
}

static bool _DeviceStatus__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  auto typed_message =
    static_cast<nanotec_msgs::msg::DeviceStatus *>(
    untyped_ros_message);
  return cdr_deserialize(cdr, *typed_message);
}

static uint32_t _DeviceStatus__get_serialized_size(
  const void * untyped_ros_message)
{
  auto typed_message =
    static_cast<const nanotec_msgs::msg::DeviceStatus *>(
    untyped_ros_message);
  return static_cast<uint32_t>(get_serialized_size(*typed_message, 0));
}

static size_t _DeviceStatus__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_DeviceStatus(full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}

static message_type_support_callbacks_t _DeviceStatus__callbacks = {
  "nanotec_msgs::msg",
  "DeviceStatus",
  _DeviceStatus__cdr_serialize,
  _DeviceStatus__cdr_deserialize,
  _DeviceStatus__get_serialized_size,
  _DeviceStatus__max_serialized_size
};

static rosidl_message_type_support_t _DeviceStatus__handle = {
  rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
  &_DeviceStatus__callbacks,
  get_message_typesupport_handle_function,
};

}  // namespace typesupport_fastrtps_cpp

}  // namespace msg

}  // namespace nanotec_msgs

namespace rosidl_typesupport_fastrtps_cpp
{

template<>
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_EXPORT_nanotec_msgs
const rosidl_message_type_support_t *
get_message_type_support_handle<nanotec_msgs::msg::DeviceStatus>()
{
  return &nanotec_msgs::msg::typesupport_fastrtps_cpp::_DeviceStatus__handle;
}

}  // namespace rosidl_typesupport_fastrtps_cpp

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, nanotec_msgs, msg, DeviceStatus)() {
  return &nanotec_msgs::msg::typesupport_fastrtps_cpp::_DeviceStatus__handle;
}

#ifdef __cplusplus
}
#endif
