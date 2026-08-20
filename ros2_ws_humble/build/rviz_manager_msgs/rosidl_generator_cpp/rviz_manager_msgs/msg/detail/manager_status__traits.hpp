// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from rviz_manager_msgs:msg/ManagerStatus.idl
// generated code does not contain a copyright notice

#ifndef RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__TRAITS_HPP_
#define RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "rviz_manager_msgs/msg/detail/manager_status__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace rviz_manager_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const ManagerStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: id
  {
    out << "id: ";
    rosidl_generator_traits::value_to_yaml(msg.id, out);
    out << ", ";
  }

  // member: status
  {
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << ", ";
  }

  // member: message
  {
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ManagerStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "id: ";
    rosidl_generator_traits::value_to_yaml(msg.id, out);
    out << "\n";
  }

  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << "\n";
  }

  // member: message
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ManagerStatus & msg, bool use_flow_style = false)
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

}  // namespace rviz_manager_msgs

namespace rosidl_generator_traits
{

[[deprecated("use rviz_manager_msgs::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const rviz_manager_msgs::msg::ManagerStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  rviz_manager_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use rviz_manager_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const rviz_manager_msgs::msg::ManagerStatus & msg)
{
  return rviz_manager_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<rviz_manager_msgs::msg::ManagerStatus>()
{
  return "rviz_manager_msgs::msg::ManagerStatus";
}

template<>
inline const char * name<rviz_manager_msgs::msg::ManagerStatus>()
{
  return "rviz_manager_msgs/msg/ManagerStatus";
}

template<>
struct has_fixed_size<rviz_manager_msgs::msg::ManagerStatus>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<rviz_manager_msgs::msg::ManagerStatus>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<rviz_manager_msgs::msg::ManagerStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__TRAITS_HPP_
