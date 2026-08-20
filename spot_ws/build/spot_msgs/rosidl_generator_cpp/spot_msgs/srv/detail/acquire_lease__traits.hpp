// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from spot_msgs:srv/AcquireLease.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__TRAITS_HPP_
#define SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "spot_msgs/srv/detail/acquire_lease__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const AcquireLease_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: client_name
  {
    out << "client_name: ";
    rosidl_generator_traits::value_to_yaml(msg.client_name, out);
    out << ", ";
  }

  // member: resource_name
  {
    out << "resource_name: ";
    rosidl_generator_traits::value_to_yaml(msg.resource_name, out);
    out << ", ";
  }

  // member: force
  {
    out << "force: ";
    rosidl_generator_traits::value_to_yaml(msg.force, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const AcquireLease_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: client_name
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "client_name: ";
    rosidl_generator_traits::value_to_yaml(msg.client_name, out);
    out << "\n";
  }

  // member: resource_name
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "resource_name: ";
    rosidl_generator_traits::value_to_yaml(msg.resource_name, out);
    out << "\n";
  }

  // member: force
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "force: ";
    rosidl_generator_traits::value_to_yaml(msg.force, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const AcquireLease_Request & msg, bool use_flow_style = false)
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
  const spot_msgs::srv::AcquireLease_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::AcquireLease_Request & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::AcquireLease_Request>()
{
  return "spot_msgs::srv::AcquireLease_Request";
}

template<>
inline const char * name<spot_msgs::srv::AcquireLease_Request>()
{
  return "spot_msgs/srv/AcquireLease_Request";
}

template<>
struct has_fixed_size<spot_msgs::srv::AcquireLease_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<spot_msgs::srv::AcquireLease_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<spot_msgs::srv::AcquireLease_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'lease'
#include "bosdyn_api_msgs/msg/detail/lease__traits.hpp"

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const AcquireLease_Response & msg,
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

  // member: lease
  {
    out << "lease: ";
    to_flow_style_yaml(msg.lease, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const AcquireLease_Response & msg,
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

  // member: lease
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "lease:\n";
    to_block_style_yaml(msg.lease, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const AcquireLease_Response & msg, bool use_flow_style = false)
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
  const spot_msgs::srv::AcquireLease_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::AcquireLease_Response & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::AcquireLease_Response>()
{
  return "spot_msgs::srv::AcquireLease_Response";
}

template<>
inline const char * name<spot_msgs::srv::AcquireLease_Response>()
{
  return "spot_msgs/srv/AcquireLease_Response";
}

template<>
struct has_fixed_size<spot_msgs::srv::AcquireLease_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<spot_msgs::srv::AcquireLease_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<spot_msgs::srv::AcquireLease_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<spot_msgs::srv::AcquireLease>()
{
  return "spot_msgs::srv::AcquireLease";
}

template<>
inline const char * name<spot_msgs::srv::AcquireLease>()
{
  return "spot_msgs/srv/AcquireLease";
}

template<>
struct has_fixed_size<spot_msgs::srv::AcquireLease>
  : std::integral_constant<
    bool,
    has_fixed_size<spot_msgs::srv::AcquireLease_Request>::value &&
    has_fixed_size<spot_msgs::srv::AcquireLease_Response>::value
  >
{
};

template<>
struct has_bounded_size<spot_msgs::srv::AcquireLease>
  : std::integral_constant<
    bool,
    has_bounded_size<spot_msgs::srv::AcquireLease_Request>::value &&
    has_bounded_size<spot_msgs::srv::AcquireLease_Response>::value
  >
{
};

template<>
struct is_service<spot_msgs::srv::AcquireLease>
  : std::true_type
{
};

template<>
struct is_service_request<spot_msgs::srv::AcquireLease_Request>
  : std::true_type
{
};

template<>
struct is_service_response<spot_msgs::srv::AcquireLease_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__TRAITS_HPP_
