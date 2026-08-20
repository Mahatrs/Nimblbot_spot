// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from spot_msgs:srv/RobotCommand.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__TRAITS_HPP_
#define SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "spot_msgs/srv/detail/robot_command__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'command'
#include "bosdyn_api_msgs/msg/detail/robot_command__traits.hpp"
// Member 'duration'
#include "builtin_interfaces/msg/detail/duration__traits.hpp"

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const RobotCommand_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: command
  {
    out << "command: ";
    to_flow_style_yaml(msg.command, out);
    out << ", ";
  }

  // member: duration
  {
    out << "duration: ";
    to_flow_style_yaml(msg.duration, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const RobotCommand_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: command
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "command:\n";
    to_block_style_yaml(msg.command, out, indentation + 2);
  }

  // member: duration
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "duration:\n";
    to_block_style_yaml(msg.duration, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const RobotCommand_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace spot_msgs

namespace rosidl_generator_traits
{

[[deprecated("use spot_msgs::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const spot_msgs::srv::RobotCommand_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::RobotCommand_Request & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::RobotCommand_Request>()
{
  return "spot_msgs::srv::RobotCommand_Request";
}

template<>
inline const char * name<spot_msgs::srv::RobotCommand_Request>()
{
  return "spot_msgs/srv/RobotCommand_Request";
}

template<>
struct has_fixed_size<spot_msgs::srv::RobotCommand_Request>
  : std::integral_constant<bool, has_fixed_size<bosdyn_api_msgs::msg::RobotCommand>::value && has_fixed_size<builtin_interfaces::msg::Duration>::value> {};

template<>
struct has_bounded_size<spot_msgs::srv::RobotCommand_Request>
  : std::integral_constant<bool, has_bounded_size<bosdyn_api_msgs::msg::RobotCommand>::value && has_bounded_size<builtin_interfaces::msg::Duration>::value> {};

template<>
struct is_message<spot_msgs::srv::RobotCommand_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const RobotCommand_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: success
  {
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << ", ";
  }

  // member: message
  {
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
    out << ", ";
  }

  // member: robot_command_id
  {
    out << "robot_command_id: ";
    rosidl_generator_traits::value_to_yaml(msg.robot_command_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const RobotCommand_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: success
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
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

  // member: robot_command_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "robot_command_id: ";
    rosidl_generator_traits::value_to_yaml(msg.robot_command_id, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const RobotCommand_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace spot_msgs

namespace rosidl_generator_traits
{

[[deprecated("use spot_msgs::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const spot_msgs::srv::RobotCommand_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::RobotCommand_Response & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::RobotCommand_Response>()
{
  return "spot_msgs::srv::RobotCommand_Response";
}

template<>
inline const char * name<spot_msgs::srv::RobotCommand_Response>()
{
  return "spot_msgs/srv/RobotCommand_Response";
}

template<>
struct has_fixed_size<spot_msgs::srv::RobotCommand_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<spot_msgs::srv::RobotCommand_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<spot_msgs::srv::RobotCommand_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<spot_msgs::srv::RobotCommand>()
{
  return "spot_msgs::srv::RobotCommand";
}

template<>
inline const char * name<spot_msgs::srv::RobotCommand>()
{
  return "spot_msgs/srv/RobotCommand";
}

template<>
struct has_fixed_size<spot_msgs::srv::RobotCommand>
  : std::integral_constant<
    bool,
    has_fixed_size<spot_msgs::srv::RobotCommand_Request>::value &&
    has_fixed_size<spot_msgs::srv::RobotCommand_Response>::value
  >
{
};

template<>
struct has_bounded_size<spot_msgs::srv::RobotCommand>
  : std::integral_constant<
    bool,
    has_bounded_size<spot_msgs::srv::RobotCommand_Request>::value &&
    has_bounded_size<spot_msgs::srv::RobotCommand_Response>::value
  >
{
};

template<>
struct is_service<spot_msgs::srv::RobotCommand>
  : std::true_type
{
};

template<>
struct is_service_request<spot_msgs::srv::RobotCommand_Request>
  : std::true_type
{
};

template<>
struct is_service_response<spot_msgs::srv::RobotCommand_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__TRAITS_HPP_
