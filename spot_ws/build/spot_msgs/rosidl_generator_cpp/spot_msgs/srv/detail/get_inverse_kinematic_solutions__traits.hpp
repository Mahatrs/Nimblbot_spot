// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from spot_msgs:srv/GetInverseKinematicSolutions.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__GET_INVERSE_KINEMATIC_SOLUTIONS__TRAITS_HPP_
#define SPOT_MSGS__SRV__DETAIL__GET_INVERSE_KINEMATIC_SOLUTIONS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "spot_msgs/srv/detail/get_inverse_kinematic_solutions__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'request'
#include "bosdyn_spot_api_msgs/msg/detail/inverse_kinematics_request__traits.hpp"

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetInverseKinematicSolutions_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: request
  {
    out << "request: ";
    to_flow_style_yaml(msg.request, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetInverseKinematicSolutions_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: request
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "request:\n";
    to_block_style_yaml(msg.request, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetInverseKinematicSolutions_Request & msg, bool use_flow_style = false)
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
  const spot_msgs::srv::GetInverseKinematicSolutions_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::GetInverseKinematicSolutions_Request & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::GetInverseKinematicSolutions_Request>()
{
  return "spot_msgs::srv::GetInverseKinematicSolutions_Request";
}

template<>
inline const char * name<spot_msgs::srv::GetInverseKinematicSolutions_Request>()
{
  return "spot_msgs/srv/GetInverseKinematicSolutions_Request";
}

template<>
struct has_fixed_size<spot_msgs::srv::GetInverseKinematicSolutions_Request>
  : std::integral_constant<bool, has_fixed_size<bosdyn_spot_api_msgs::msg::InverseKinematicsRequest>::value> {};

template<>
struct has_bounded_size<spot_msgs::srv::GetInverseKinematicSolutions_Request>
  : std::integral_constant<bool, has_bounded_size<bosdyn_spot_api_msgs::msg::InverseKinematicsRequest>::value> {};

template<>
struct is_message<spot_msgs::srv::GetInverseKinematicSolutions_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'response'
#include "bosdyn_spot_api_msgs/msg/detail/inverse_kinematics_response__traits.hpp"

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetInverseKinematicSolutions_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: response
  {
    out << "response: ";
    to_flow_style_yaml(msg.response, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetInverseKinematicSolutions_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: response
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "response:\n";
    to_block_style_yaml(msg.response, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetInverseKinematicSolutions_Response & msg, bool use_flow_style = false)
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
  const spot_msgs::srv::GetInverseKinematicSolutions_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::GetInverseKinematicSolutions_Response & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::GetInverseKinematicSolutions_Response>()
{
  return "spot_msgs::srv::GetInverseKinematicSolutions_Response";
}

template<>
inline const char * name<spot_msgs::srv::GetInverseKinematicSolutions_Response>()
{
  return "spot_msgs/srv/GetInverseKinematicSolutions_Response";
}

template<>
struct has_fixed_size<spot_msgs::srv::GetInverseKinematicSolutions_Response>
  : std::integral_constant<bool, has_fixed_size<bosdyn_spot_api_msgs::msg::InverseKinematicsResponse>::value> {};

template<>
struct has_bounded_size<spot_msgs::srv::GetInverseKinematicSolutions_Response>
  : std::integral_constant<bool, has_bounded_size<bosdyn_spot_api_msgs::msg::InverseKinematicsResponse>::value> {};

template<>
struct is_message<spot_msgs::srv::GetInverseKinematicSolutions_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<spot_msgs::srv::GetInverseKinematicSolutions>()
{
  return "spot_msgs::srv::GetInverseKinematicSolutions";
}

template<>
inline const char * name<spot_msgs::srv::GetInverseKinematicSolutions>()
{
  return "spot_msgs/srv/GetInverseKinematicSolutions";
}

template<>
struct has_fixed_size<spot_msgs::srv::GetInverseKinematicSolutions>
  : std::integral_constant<
    bool,
    has_fixed_size<spot_msgs::srv::GetInverseKinematicSolutions_Request>::value &&
    has_fixed_size<spot_msgs::srv::GetInverseKinematicSolutions_Response>::value
  >
{
};

template<>
struct has_bounded_size<spot_msgs::srv::GetInverseKinematicSolutions>
  : std::integral_constant<
    bool,
    has_bounded_size<spot_msgs::srv::GetInverseKinematicSolutions_Request>::value &&
    has_bounded_size<spot_msgs::srv::GetInverseKinematicSolutions_Response>::value
  >
{
};

template<>
struct is_service<spot_msgs::srv::GetInverseKinematicSolutions>
  : std::true_type
{
};

template<>
struct is_service_request<spot_msgs::srv::GetInverseKinematicSolutions_Request>
  : std::true_type
{
};

template<>
struct is_service_response<spot_msgs::srv::GetInverseKinematicSolutions_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // SPOT_MSGS__SRV__DETAIL__GET_INVERSE_KINEMATIC_SOLUTIONS__TRAITS_HPP_
