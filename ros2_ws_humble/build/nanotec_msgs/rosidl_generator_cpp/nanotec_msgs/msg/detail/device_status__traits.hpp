// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from nanotec_msgs:msg/DeviceStatus.idl
// generated code does not contain a copyright notice

#ifndef NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__TRAITS_HPP_
#define NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "nanotec_msgs/msg/detail/device_status__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace nanotec_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const DeviceStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: name
  {
    out << "name: ";
    rosidl_generator_traits::value_to_yaml(msg.name, out);
    out << ", ";
  }

  // member: voltage_power
  {
    out << "voltage_power: ";
    rosidl_generator_traits::value_to_yaml(msg.voltage_power, out);
    out << ", ";
  }

  // member: voltage_logic
  {
    out << "voltage_logic: ";
    rosidl_generator_traits::value_to_yaml(msg.voltage_logic, out);
    out << ", ";
  }

  // member: temperature_motor
  {
    out << "temperature_motor: ";
    rosidl_generator_traits::value_to_yaml(msg.temperature_motor, out);
    out << ", ";
  }

  // member: temperature_micro_chip
  {
    out << "temperature_micro_chip: ";
    rosidl_generator_traits::value_to_yaml(msg.temperature_micro_chip, out);
    out << ", ";
  }

  // member: ready_to_switch_on
  {
    out << "ready_to_switch_on: ";
    rosidl_generator_traits::value_to_yaml(msg.ready_to_switch_on, out);
    out << ", ";
  }

  // member: switched_on
  {
    out << "switched_on: ";
    rosidl_generator_traits::value_to_yaml(msg.switched_on, out);
    out << ", ";
  }

  // member: operation_enabled
  {
    out << "operation_enabled: ";
    rosidl_generator_traits::value_to_yaml(msg.operation_enabled, out);
    out << ", ";
  }

  // member: fault
  {
    out << "fault: ";
    rosidl_generator_traits::value_to_yaml(msg.fault, out);
    out << ", ";
  }

  // member: voltage_enabled
  {
    out << "voltage_enabled: ";
    rosidl_generator_traits::value_to_yaml(msg.voltage_enabled, out);
    out << ", ";
  }

  // member: quick_stop
  {
    out << "quick_stop: ";
    rosidl_generator_traits::value_to_yaml(msg.quick_stop, out);
    out << ", ";
  }

  // member: switch_on_disabled
  {
    out << "switch_on_disabled: ";
    rosidl_generator_traits::value_to_yaml(msg.switch_on_disabled, out);
    out << ", ";
  }

  // member: warning
  {
    out << "warning: ";
    rosidl_generator_traits::value_to_yaml(msg.warning, out);
    out << ", ";
  }

  // member: target_reached
  {
    out << "target_reached: ";
    rosidl_generator_traits::value_to_yaml(msg.target_reached, out);
    out << ", ";
  }

  // member: internal_limit_active
  {
    out << "internal_limit_active: ";
    rosidl_generator_traits::value_to_yaml(msg.internal_limit_active, out);
    out << ", ";
  }

  // member: operation_mode_specific
  {
    out << "operation_mode_specific: ";
    rosidl_generator_traits::value_to_yaml(msg.operation_mode_specific, out);
    out << ", ";
  }

  // member: homing_status
  {
    out << "homing_status: ";
    rosidl_generator_traits::value_to_yaml(msg.homing_status, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const DeviceStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: name
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "name: ";
    rosidl_generator_traits::value_to_yaml(msg.name, out);
    out << "\n";
  }

  // member: voltage_power
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "voltage_power: ";
    rosidl_generator_traits::value_to_yaml(msg.voltage_power, out);
    out << "\n";
  }

  // member: voltage_logic
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "voltage_logic: ";
    rosidl_generator_traits::value_to_yaml(msg.voltage_logic, out);
    out << "\n";
  }

  // member: temperature_motor
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "temperature_motor: ";
    rosidl_generator_traits::value_to_yaml(msg.temperature_motor, out);
    out << "\n";
  }

  // member: temperature_micro_chip
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "temperature_micro_chip: ";
    rosidl_generator_traits::value_to_yaml(msg.temperature_micro_chip, out);
    out << "\n";
  }

  // member: ready_to_switch_on
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "ready_to_switch_on: ";
    rosidl_generator_traits::value_to_yaml(msg.ready_to_switch_on, out);
    out << "\n";
  }

  // member: switched_on
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "switched_on: ";
    rosidl_generator_traits::value_to_yaml(msg.switched_on, out);
    out << "\n";
  }

  // member: operation_enabled
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "operation_enabled: ";
    rosidl_generator_traits::value_to_yaml(msg.operation_enabled, out);
    out << "\n";
  }

  // member: fault
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "fault: ";
    rosidl_generator_traits::value_to_yaml(msg.fault, out);
    out << "\n";
  }

  // member: voltage_enabled
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "voltage_enabled: ";
    rosidl_generator_traits::value_to_yaml(msg.voltage_enabled, out);
    out << "\n";
  }

  // member: quick_stop
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "quick_stop: ";
    rosidl_generator_traits::value_to_yaml(msg.quick_stop, out);
    out << "\n";
  }

  // member: switch_on_disabled
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "switch_on_disabled: ";
    rosidl_generator_traits::value_to_yaml(msg.switch_on_disabled, out);
    out << "\n";
  }

  // member: warning
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "warning: ";
    rosidl_generator_traits::value_to_yaml(msg.warning, out);
    out << "\n";
  }

  // member: target_reached
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "target_reached: ";
    rosidl_generator_traits::value_to_yaml(msg.target_reached, out);
    out << "\n";
  }

  // member: internal_limit_active
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "internal_limit_active: ";
    rosidl_generator_traits::value_to_yaml(msg.internal_limit_active, out);
    out << "\n";
  }

  // member: operation_mode_specific
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "operation_mode_specific: ";
    rosidl_generator_traits::value_to_yaml(msg.operation_mode_specific, out);
    out << "\n";
  }

  // member: homing_status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "homing_status: ";
    rosidl_generator_traits::value_to_yaml(msg.homing_status, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const DeviceStatus & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace msg

}  // namespace nanotec_msgs

namespace rosidl_generator_traits
{

[[deprecated("use nanotec_msgs::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const nanotec_msgs::msg::DeviceStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  nanotec_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use nanotec_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const nanotec_msgs::msg::DeviceStatus & msg)
{
  return nanotec_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<nanotec_msgs::msg::DeviceStatus>()
{
  return "nanotec_msgs::msg::DeviceStatus";
}

template<>
inline const char * name<nanotec_msgs::msg::DeviceStatus>()
{
  return "nanotec_msgs/msg/DeviceStatus";
}

template<>
struct has_fixed_size<nanotec_msgs::msg::DeviceStatus>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<nanotec_msgs::msg::DeviceStatus>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<nanotec_msgs::msg::DeviceStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__TRAITS_HPP_
