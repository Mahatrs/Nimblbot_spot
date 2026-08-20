// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from spot_msgs:srv/RetrieveLogpoint.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__RETRIEVE_LOGPOINT__TRAITS_HPP_
#define SPOT_MSGS__SRV__DETAIL__RETRIEVE_LOGPOINT__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "spot_msgs/srv/detail/retrieve_logpoint__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const RetrieveLogpoint_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: name
  {
    out << "name: ";
    rosidl_generator_traits::value_to_yaml(msg.name, out);
    out << ", ";
  }

  // member: raw
  {
    out << "raw: ";
    rosidl_generator_traits::value_to_yaml(msg.raw, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const RetrieveLogpoint_Request & msg,
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

  // member: raw
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "raw: ";
    rosidl_generator_traits::value_to_yaml(msg.raw, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const RetrieveLogpoint_Request & msg, bool use_flow_style = false)
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
  const spot_msgs::srv::RetrieveLogpoint_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::RetrieveLogpoint_Request & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::RetrieveLogpoint_Request>()
{
  return "spot_msgs::srv::RetrieveLogpoint_Request";
}

template<>
inline const char * name<spot_msgs::srv::RetrieveLogpoint_Request>()
{
  return "spot_msgs/srv/RetrieveLogpoint_Request";
}

template<>
struct has_fixed_size<spot_msgs::srv::RetrieveLogpoint_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<spot_msgs::srv::RetrieveLogpoint_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<spot_msgs::srv::RetrieveLogpoint_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'logpoint'
#include "bosdyn_spot_cam_api_msgs/msg/detail/logpoint__traits.hpp"

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const RetrieveLogpoint_Response & msg,
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

  // member: logpoint
  {
    out << "logpoint: ";
    to_flow_style_yaml(msg.logpoint, out);
    out << ", ";
  }

  // member: data
  {
    if (msg.data.size() == 0) {
      out << "data: []";
    } else {
      out << "data: [";
      size_t pending_items = msg.data.size();
      for (auto item : msg.data) {
        rosidl_generator_traits::value_to_yaml(item, out);
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
  const RetrieveLogpoint_Response & msg,
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

  // member: logpoint
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "logpoint:\n";
    to_block_style_yaml(msg.logpoint, out, indentation + 2);
  }

  // member: data
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.data.size() == 0) {
      out << "data: []\n";
    } else {
      out << "data:\n";
      for (auto item : msg.data) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "- ";
        rosidl_generator_traits::value_to_yaml(item, out);
        out << "\n";
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const RetrieveLogpoint_Response & msg, bool use_flow_style = false)
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
  const spot_msgs::srv::RetrieveLogpoint_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::RetrieveLogpoint_Response & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::RetrieveLogpoint_Response>()
{
  return "spot_msgs::srv::RetrieveLogpoint_Response";
}

template<>
inline const char * name<spot_msgs::srv::RetrieveLogpoint_Response>()
{
  return "spot_msgs/srv/RetrieveLogpoint_Response";
}

template<>
struct has_fixed_size<spot_msgs::srv::RetrieveLogpoint_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<spot_msgs::srv::RetrieveLogpoint_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<spot_msgs::srv::RetrieveLogpoint_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<spot_msgs::srv::RetrieveLogpoint>()
{
  return "spot_msgs::srv::RetrieveLogpoint";
}

template<>
inline const char * name<spot_msgs::srv::RetrieveLogpoint>()
{
  return "spot_msgs/srv/RetrieveLogpoint";
}

template<>
struct has_fixed_size<spot_msgs::srv::RetrieveLogpoint>
  : std::integral_constant<
    bool,
    has_fixed_size<spot_msgs::srv::RetrieveLogpoint_Request>::value &&
    has_fixed_size<spot_msgs::srv::RetrieveLogpoint_Response>::value
  >
{
};

template<>
struct has_bounded_size<spot_msgs::srv::RetrieveLogpoint>
  : std::integral_constant<
    bool,
    has_bounded_size<spot_msgs::srv::RetrieveLogpoint_Request>::value &&
    has_bounded_size<spot_msgs::srv::RetrieveLogpoint_Response>::value
  >
{
};

template<>
struct is_service<spot_msgs::srv::RetrieveLogpoint>
  : std::true_type
{
};

template<>
struct is_service_request<spot_msgs::srv::RetrieveLogpoint_Request>
  : std::true_type
{
};

template<>
struct is_service_response<spot_msgs::srv::RetrieveLogpoint_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // SPOT_MSGS__SRV__DETAIL__RETRIEVE_LOGPOINT__TRAITS_HPP_
