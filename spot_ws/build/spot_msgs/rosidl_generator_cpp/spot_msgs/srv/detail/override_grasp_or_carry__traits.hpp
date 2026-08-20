// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from spot_msgs:srv/OverrideGraspOrCarry.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__OVERRIDE_GRASP_OR_CARRY__TRAITS_HPP_
#define SPOT_MSGS__SRV__DETAIL__OVERRIDE_GRASP_OR_CARRY__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "spot_msgs/srv/detail/override_grasp_or_carry__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'grasp_override'
#include "bosdyn_api_msgs/msg/detail/api_grasp_override_override__traits.hpp"
// Member 'carry_override'
#include "bosdyn_api_msgs/msg/detail/manipulator_state_carry_state__traits.hpp"

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const OverrideGraspOrCarry_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: grasp_override
  {
    out << "grasp_override: ";
    to_flow_style_yaml(msg.grasp_override, out);
    out << ", ";
  }

  // member: carry_override
  {
    out << "carry_override: ";
    to_flow_style_yaml(msg.carry_override, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const OverrideGraspOrCarry_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: grasp_override
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "grasp_override:\n";
    to_block_style_yaml(msg.grasp_override, out, indentation + 2);
  }

  // member: carry_override
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "carry_override:\n";
    to_block_style_yaml(msg.carry_override, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const OverrideGraspOrCarry_Request & msg, bool use_flow_style = false)
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
  const spot_msgs::srv::OverrideGraspOrCarry_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::OverrideGraspOrCarry_Request & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::OverrideGraspOrCarry_Request>()
{
  return "spot_msgs::srv::OverrideGraspOrCarry_Request";
}

template<>
inline const char * name<spot_msgs::srv::OverrideGraspOrCarry_Request>()
{
  return "spot_msgs/srv/OverrideGraspOrCarry_Request";
}

template<>
struct has_fixed_size<spot_msgs::srv::OverrideGraspOrCarry_Request>
  : std::integral_constant<bool, has_fixed_size<bosdyn_api_msgs::msg::ApiGraspOverrideOverride>::value && has_fixed_size<bosdyn_api_msgs::msg::ManipulatorStateCarryState>::value> {};

template<>
struct has_bounded_size<spot_msgs::srv::OverrideGraspOrCarry_Request>
  : std::integral_constant<bool, has_bounded_size<bosdyn_api_msgs::msg::ApiGraspOverrideOverride>::value && has_bounded_size<bosdyn_api_msgs::msg::ManipulatorStateCarryState>::value> {};

template<>
struct is_message<spot_msgs::srv::OverrideGraspOrCarry_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const OverrideGraspOrCarry_Response & msg,
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
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const OverrideGraspOrCarry_Response & msg,
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
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const OverrideGraspOrCarry_Response & msg, bool use_flow_style = false)
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
  const spot_msgs::srv::OverrideGraspOrCarry_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::OverrideGraspOrCarry_Response & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::OverrideGraspOrCarry_Response>()
{
  return "spot_msgs::srv::OverrideGraspOrCarry_Response";
}

template<>
inline const char * name<spot_msgs::srv::OverrideGraspOrCarry_Response>()
{
  return "spot_msgs/srv/OverrideGraspOrCarry_Response";
}

template<>
struct has_fixed_size<spot_msgs::srv::OverrideGraspOrCarry_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<spot_msgs::srv::OverrideGraspOrCarry_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<spot_msgs::srv::OverrideGraspOrCarry_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<spot_msgs::srv::OverrideGraspOrCarry>()
{
  return "spot_msgs::srv::OverrideGraspOrCarry";
}

template<>
inline const char * name<spot_msgs::srv::OverrideGraspOrCarry>()
{
  return "spot_msgs/srv/OverrideGraspOrCarry";
}

template<>
struct has_fixed_size<spot_msgs::srv::OverrideGraspOrCarry>
  : std::integral_constant<
    bool,
    has_fixed_size<spot_msgs::srv::OverrideGraspOrCarry_Request>::value &&
    has_fixed_size<spot_msgs::srv::OverrideGraspOrCarry_Response>::value
  >
{
};

template<>
struct has_bounded_size<spot_msgs::srv::OverrideGraspOrCarry>
  : std::integral_constant<
    bool,
    has_bounded_size<spot_msgs::srv::OverrideGraspOrCarry_Request>::value &&
    has_bounded_size<spot_msgs::srv::OverrideGraspOrCarry_Response>::value
  >
{
};

template<>
struct is_service<spot_msgs::srv::OverrideGraspOrCarry>
  : std::true_type
{
};

template<>
struct is_service_request<spot_msgs::srv::OverrideGraspOrCarry_Request>
  : std::true_type
{
};

template<>
struct is_service_response<spot_msgs::srv::OverrideGraspOrCarry_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // SPOT_MSGS__SRV__DETAIL__OVERRIDE_GRASP_OR_CARRY__TRAITS_HPP_
