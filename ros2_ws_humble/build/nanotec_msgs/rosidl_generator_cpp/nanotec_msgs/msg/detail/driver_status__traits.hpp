// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from nanotec_msgs:msg/DriverStatus.idl
// generated code does not contain a copyright notice

#ifndef NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__TRAITS_HPP_
#define NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "nanotec_msgs/msg/detail/driver_status__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__traits.hpp"
// Member 'devices'
#include "nanotec_msgs/msg/detail/device_status__traits.hpp"

namespace nanotec_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const DriverStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: header
  {
    out << "header: ";
    to_flow_style_yaml(msg.header, out);
    out << ", ";
  }

  // member: devices
  {
    if (msg.devices.size() == 0) {
      out << "devices: []";
    } else {
      out << "devices: [";
      size_t pending_items = msg.devices.size();
      for (auto item : msg.devices) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const DriverStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: header
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "header:\n";
    to_block_style_yaml(msg.header, out, indentation + 2);
  }

  // member: devices
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.devices.size() == 0) {
      out << "devices: []\n";
    } else {
      out << "devices:\n";
      for (auto item : msg.devices) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const DriverStatus & msg, bool use_flow_style = false)
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
  const nanotec_msgs::msg::DriverStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  nanotec_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use nanotec_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const nanotec_msgs::msg::DriverStatus & msg)
{
  return nanotec_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<nanotec_msgs::msg::DriverStatus>()
{
  return "nanotec_msgs::msg::DriverStatus";
}

template<>
inline const char * name<nanotec_msgs::msg::DriverStatus>()
{
  return "nanotec_msgs/msg/DriverStatus";
}

template<>
struct has_fixed_size<nanotec_msgs::msg::DriverStatus>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<nanotec_msgs::msg::DriverStatus>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<nanotec_msgs::msg::DriverStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__TRAITS_HPP_
