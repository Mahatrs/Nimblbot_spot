// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from spot_msgs:srv/ChoreographyStartRecordingState.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__TRAITS_HPP_
#define SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "spot_msgs/srv/detail/choreography_start_recording_state__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const ChoreographyStartRecordingState_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: duration_seconds
  {
    out << "duration_seconds: ";
    rosidl_generator_traits::value_to_yaml(msg.duration_seconds, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ChoreographyStartRecordingState_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: duration_seconds
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "duration_seconds: ";
    rosidl_generator_traits::value_to_yaml(msg.duration_seconds, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ChoreographyStartRecordingState_Request & msg, bool use_flow_style = false)
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
  const spot_msgs::srv::ChoreographyStartRecordingState_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::ChoreographyStartRecordingState_Request & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::ChoreographyStartRecordingState_Request>()
{
  return "spot_msgs::srv::ChoreographyStartRecordingState_Request";
}

template<>
inline const char * name<spot_msgs::srv::ChoreographyStartRecordingState_Request>()
{
  return "spot_msgs/srv/ChoreographyStartRecordingState_Request";
}

template<>
struct has_fixed_size<spot_msgs::srv::ChoreographyStartRecordingState_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<spot_msgs::srv::ChoreographyStartRecordingState_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<spot_msgs::srv::ChoreographyStartRecordingState_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace spot_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const ChoreographyStartRecordingState_Response & msg,
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

  // member: status
  {
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << ", ";
  }

  // member: recording_session_id
  {
    out << "recording_session_id: ";
    rosidl_generator_traits::value_to_yaml(msg.recording_session_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ChoreographyStartRecordingState_Response & msg,
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

  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << "\n";
  }

  // member: recording_session_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "recording_session_id: ";
    rosidl_generator_traits::value_to_yaml(msg.recording_session_id, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ChoreographyStartRecordingState_Response & msg, bool use_flow_style = false)
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
  const spot_msgs::srv::ChoreographyStartRecordingState_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::srv::ChoreographyStartRecordingState_Response & msg)
{
  return spot_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::srv::ChoreographyStartRecordingState_Response>()
{
  return "spot_msgs::srv::ChoreographyStartRecordingState_Response";
}

template<>
inline const char * name<spot_msgs::srv::ChoreographyStartRecordingState_Response>()
{
  return "spot_msgs/srv/ChoreographyStartRecordingState_Response";
}

template<>
struct has_fixed_size<spot_msgs::srv::ChoreographyStartRecordingState_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<spot_msgs::srv::ChoreographyStartRecordingState_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<spot_msgs::srv::ChoreographyStartRecordingState_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<spot_msgs::srv::ChoreographyStartRecordingState>()
{
  return "spot_msgs::srv::ChoreographyStartRecordingState";
}

template<>
inline const char * name<spot_msgs::srv::ChoreographyStartRecordingState>()
{
  return "spot_msgs/srv/ChoreographyStartRecordingState";
}

template<>
struct has_fixed_size<spot_msgs::srv::ChoreographyStartRecordingState>
  : std::integral_constant<
    bool,
    has_fixed_size<spot_msgs::srv::ChoreographyStartRecordingState_Request>::value &&
    has_fixed_size<spot_msgs::srv::ChoreographyStartRecordingState_Response>::value
  >
{
};

template<>
struct has_bounded_size<spot_msgs::srv::ChoreographyStartRecordingState>
  : std::integral_constant<
    bool,
    has_bounded_size<spot_msgs::srv::ChoreographyStartRecordingState_Request>::value &&
    has_bounded_size<spot_msgs::srv::ChoreographyStartRecordingState_Response>::value
  >
{
};

template<>
struct is_service<spot_msgs::srv::ChoreographyStartRecordingState>
  : std::true_type
{
};

template<>
struct is_service_request<spot_msgs::srv::ChoreographyStartRecordingState_Request>
  : std::true_type
{
};

template<>
struct is_service_response<spot_msgs::srv::ChoreographyStartRecordingState_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__TRAITS_HPP_
