// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from spot_msgs:action/ExecuteDance.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__ACTION__DETAIL__EXECUTE_DANCE__TRAITS_HPP_
#define SPOT_MSGS__ACTION__DETAIL__EXECUTE_DANCE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "spot_msgs/action/detail/execute_dance__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace spot_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const ExecuteDance_Goal & msg,
  std::ostream & out)
{
  out << "{";
  // member: choreo_name
  {
    out << "choreo_name: ";
    rosidl_generator_traits::value_to_yaml(msg.choreo_name, out);
    out << ", ";
  }

  // member: choreo_file_content
  {
    out << "choreo_file_content: ";
    rosidl_generator_traits::value_to_yaml(msg.choreo_file_content, out);
    out << ", ";
  }

  // member: choreo_sequence_serialized
  {
    if (msg.choreo_sequence_serialized.size() == 0) {
      out << "choreo_sequence_serialized: []";
    } else {
      out << "choreo_sequence_serialized: [";
      size_t pending_items = msg.choreo_sequence_serialized.size();
      for (auto item : msg.choreo_sequence_serialized) {
        rosidl_generator_traits::value_to_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
    out << ", ";
  }

  // member: start_slice
  {
    out << "start_slice: ";
    rosidl_generator_traits::value_to_yaml(msg.start_slice, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ExecuteDance_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: choreo_name
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "choreo_name: ";
    rosidl_generator_traits::value_to_yaml(msg.choreo_name, out);
    out << "\n";
  }

  // member: choreo_file_content
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "choreo_file_content: ";
    rosidl_generator_traits::value_to_yaml(msg.choreo_file_content, out);
    out << "\n";
  }

  // member: choreo_sequence_serialized
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.choreo_sequence_serialized.size() == 0) {
      out << "choreo_sequence_serialized: []\n";
    } else {
      out << "choreo_sequence_serialized:\n";
      for (auto item : msg.choreo_sequence_serialized) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "- ";
        rosidl_generator_traits::value_to_yaml(item, out);
        out << "\n";
      }
    }
  }

  // member: start_slice
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "start_slice: ";
    rosidl_generator_traits::value_to_yaml(msg.start_slice, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ExecuteDance_Goal & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace spot_msgs

namespace rosidl_generator_traits
{

[[deprecated("use spot_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const spot_msgs::action::ExecuteDance_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::action::ExecuteDance_Goal & msg)
{
  return spot_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::action::ExecuteDance_Goal>()
{
  return "spot_msgs::action::ExecuteDance_Goal";
}

template<>
inline const char * name<spot_msgs::action::ExecuteDance_Goal>()
{
  return "spot_msgs/action/ExecuteDance_Goal";
}

template<>
struct has_fixed_size<spot_msgs::action::ExecuteDance_Goal>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<spot_msgs::action::ExecuteDance_Goal>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<spot_msgs::action::ExecuteDance_Goal>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace spot_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const ExecuteDance_Result & msg,
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
  const ExecuteDance_Result & msg,
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

inline std::string to_yaml(const ExecuteDance_Result & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace spot_msgs

namespace rosidl_generator_traits
{

[[deprecated("use spot_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const spot_msgs::action::ExecuteDance_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::action::ExecuteDance_Result & msg)
{
  return spot_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::action::ExecuteDance_Result>()
{
  return "spot_msgs::action::ExecuteDance_Result";
}

template<>
inline const char * name<spot_msgs::action::ExecuteDance_Result>()
{
  return "spot_msgs/action/ExecuteDance_Result";
}

template<>
struct has_fixed_size<spot_msgs::action::ExecuteDance_Result>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<spot_msgs::action::ExecuteDance_Result>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<spot_msgs::action::ExecuteDance_Result>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace spot_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const ExecuteDance_Feedback & msg,
  std::ostream & out)
{
  out << "{";
  // member: is_dancing
  {
    out << "is_dancing: ";
    rosidl_generator_traits::value_to_yaml(msg.is_dancing, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ExecuteDance_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: is_dancing
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "is_dancing: ";
    rosidl_generator_traits::value_to_yaml(msg.is_dancing, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ExecuteDance_Feedback & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace spot_msgs

namespace rosidl_generator_traits
{

[[deprecated("use spot_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const spot_msgs::action::ExecuteDance_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::action::ExecuteDance_Feedback & msg)
{
  return spot_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::action::ExecuteDance_Feedback>()
{
  return "spot_msgs::action::ExecuteDance_Feedback";
}

template<>
inline const char * name<spot_msgs::action::ExecuteDance_Feedback>()
{
  return "spot_msgs/action/ExecuteDance_Feedback";
}

template<>
struct has_fixed_size<spot_msgs::action::ExecuteDance_Feedback>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<spot_msgs::action::ExecuteDance_Feedback>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<spot_msgs::action::ExecuteDance_Feedback>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'goal'
#include "spot_msgs/action/detail/execute_dance__traits.hpp"

namespace spot_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const ExecuteDance_SendGoal_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: goal
  {
    out << "goal: ";
    to_flow_style_yaml(msg.goal, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ExecuteDance_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: goal
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal:\n";
    to_block_style_yaml(msg.goal, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ExecuteDance_SendGoal_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace spot_msgs

namespace rosidl_generator_traits
{

[[deprecated("use spot_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const spot_msgs::action::ExecuteDance_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::action::ExecuteDance_SendGoal_Request & msg)
{
  return spot_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::action::ExecuteDance_SendGoal_Request>()
{
  return "spot_msgs::action::ExecuteDance_SendGoal_Request";
}

template<>
inline const char * name<spot_msgs::action::ExecuteDance_SendGoal_Request>()
{
  return "spot_msgs/action/ExecuteDance_SendGoal_Request";
}

template<>
struct has_fixed_size<spot_msgs::action::ExecuteDance_SendGoal_Request>
  : std::integral_constant<bool, has_fixed_size<spot_msgs::action::ExecuteDance_Goal>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<spot_msgs::action::ExecuteDance_SendGoal_Request>
  : std::integral_constant<bool, has_bounded_size<spot_msgs::action::ExecuteDance_Goal>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<spot_msgs::action::ExecuteDance_SendGoal_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__traits.hpp"

namespace spot_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const ExecuteDance_SendGoal_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: accepted
  {
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << ", ";
  }

  // member: stamp
  {
    out << "stamp: ";
    to_flow_style_yaml(msg.stamp, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ExecuteDance_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: accepted
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << "\n";
  }

  // member: stamp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "stamp:\n";
    to_block_style_yaml(msg.stamp, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ExecuteDance_SendGoal_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace spot_msgs

namespace rosidl_generator_traits
{

[[deprecated("use spot_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const spot_msgs::action::ExecuteDance_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::action::ExecuteDance_SendGoal_Response & msg)
{
  return spot_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::action::ExecuteDance_SendGoal_Response>()
{
  return "spot_msgs::action::ExecuteDance_SendGoal_Response";
}

template<>
inline const char * name<spot_msgs::action::ExecuteDance_SendGoal_Response>()
{
  return "spot_msgs/action/ExecuteDance_SendGoal_Response";
}

template<>
struct has_fixed_size<spot_msgs::action::ExecuteDance_SendGoal_Response>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<spot_msgs::action::ExecuteDance_SendGoal_Response>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<spot_msgs::action::ExecuteDance_SendGoal_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<spot_msgs::action::ExecuteDance_SendGoal>()
{
  return "spot_msgs::action::ExecuteDance_SendGoal";
}

template<>
inline const char * name<spot_msgs::action::ExecuteDance_SendGoal>()
{
  return "spot_msgs/action/ExecuteDance_SendGoal";
}

template<>
struct has_fixed_size<spot_msgs::action::ExecuteDance_SendGoal>
  : std::integral_constant<
    bool,
    has_fixed_size<spot_msgs::action::ExecuteDance_SendGoal_Request>::value &&
    has_fixed_size<spot_msgs::action::ExecuteDance_SendGoal_Response>::value
  >
{
};

template<>
struct has_bounded_size<spot_msgs::action::ExecuteDance_SendGoal>
  : std::integral_constant<
    bool,
    has_bounded_size<spot_msgs::action::ExecuteDance_SendGoal_Request>::value &&
    has_bounded_size<spot_msgs::action::ExecuteDance_SendGoal_Response>::value
  >
{
};

template<>
struct is_service<spot_msgs::action::ExecuteDance_SendGoal>
  : std::true_type
{
};

template<>
struct is_service_request<spot_msgs::action::ExecuteDance_SendGoal_Request>
  : std::true_type
{
};

template<>
struct is_service_response<spot_msgs::action::ExecuteDance_SendGoal_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"

namespace spot_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const ExecuteDance_GetResult_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ExecuteDance_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ExecuteDance_GetResult_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace spot_msgs

namespace rosidl_generator_traits
{

[[deprecated("use spot_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const spot_msgs::action::ExecuteDance_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::action::ExecuteDance_GetResult_Request & msg)
{
  return spot_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::action::ExecuteDance_GetResult_Request>()
{
  return "spot_msgs::action::ExecuteDance_GetResult_Request";
}

template<>
inline const char * name<spot_msgs::action::ExecuteDance_GetResult_Request>()
{
  return "spot_msgs/action/ExecuteDance_GetResult_Request";
}

template<>
struct has_fixed_size<spot_msgs::action::ExecuteDance_GetResult_Request>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<spot_msgs::action::ExecuteDance_GetResult_Request>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<spot_msgs::action::ExecuteDance_GetResult_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'result'
// already included above
// #include "spot_msgs/action/detail/execute_dance__traits.hpp"

namespace spot_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const ExecuteDance_GetResult_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << ", ";
  }

  // member: result
  {
    out << "result: ";
    to_flow_style_yaml(msg.result, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ExecuteDance_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << "\n";
  }

  // member: result
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "result:\n";
    to_block_style_yaml(msg.result, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ExecuteDance_GetResult_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace spot_msgs

namespace rosidl_generator_traits
{

[[deprecated("use spot_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const spot_msgs::action::ExecuteDance_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::action::ExecuteDance_GetResult_Response & msg)
{
  return spot_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::action::ExecuteDance_GetResult_Response>()
{
  return "spot_msgs::action::ExecuteDance_GetResult_Response";
}

template<>
inline const char * name<spot_msgs::action::ExecuteDance_GetResult_Response>()
{
  return "spot_msgs/action/ExecuteDance_GetResult_Response";
}

template<>
struct has_fixed_size<spot_msgs::action::ExecuteDance_GetResult_Response>
  : std::integral_constant<bool, has_fixed_size<spot_msgs::action::ExecuteDance_Result>::value> {};

template<>
struct has_bounded_size<spot_msgs::action::ExecuteDance_GetResult_Response>
  : std::integral_constant<bool, has_bounded_size<spot_msgs::action::ExecuteDance_Result>::value> {};

template<>
struct is_message<spot_msgs::action::ExecuteDance_GetResult_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<spot_msgs::action::ExecuteDance_GetResult>()
{
  return "spot_msgs::action::ExecuteDance_GetResult";
}

template<>
inline const char * name<spot_msgs::action::ExecuteDance_GetResult>()
{
  return "spot_msgs/action/ExecuteDance_GetResult";
}

template<>
struct has_fixed_size<spot_msgs::action::ExecuteDance_GetResult>
  : std::integral_constant<
    bool,
    has_fixed_size<spot_msgs::action::ExecuteDance_GetResult_Request>::value &&
    has_fixed_size<spot_msgs::action::ExecuteDance_GetResult_Response>::value
  >
{
};

template<>
struct has_bounded_size<spot_msgs::action::ExecuteDance_GetResult>
  : std::integral_constant<
    bool,
    has_bounded_size<spot_msgs::action::ExecuteDance_GetResult_Request>::value &&
    has_bounded_size<spot_msgs::action::ExecuteDance_GetResult_Response>::value
  >
{
};

template<>
struct is_service<spot_msgs::action::ExecuteDance_GetResult>
  : std::true_type
{
};

template<>
struct is_service_request<spot_msgs::action::ExecuteDance_GetResult_Request>
  : std::true_type
{
};

template<>
struct is_service_response<spot_msgs::action::ExecuteDance_GetResult_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'feedback'
// already included above
// #include "spot_msgs/action/detail/execute_dance__traits.hpp"

namespace spot_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const ExecuteDance_FeedbackMessage & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: feedback
  {
    out << "feedback: ";
    to_flow_style_yaml(msg.feedback, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ExecuteDance_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: feedback
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "feedback:\n";
    to_block_style_yaml(msg.feedback, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ExecuteDance_FeedbackMessage & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace spot_msgs

namespace rosidl_generator_traits
{

[[deprecated("use spot_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const spot_msgs::action::ExecuteDance_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  spot_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use spot_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const spot_msgs::action::ExecuteDance_FeedbackMessage & msg)
{
  return spot_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<spot_msgs::action::ExecuteDance_FeedbackMessage>()
{
  return "spot_msgs::action::ExecuteDance_FeedbackMessage";
}

template<>
inline const char * name<spot_msgs::action::ExecuteDance_FeedbackMessage>()
{
  return "spot_msgs/action/ExecuteDance_FeedbackMessage";
}

template<>
struct has_fixed_size<spot_msgs::action::ExecuteDance_FeedbackMessage>
  : std::integral_constant<bool, has_fixed_size<spot_msgs::action::ExecuteDance_Feedback>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<spot_msgs::action::ExecuteDance_FeedbackMessage>
  : std::integral_constant<bool, has_bounded_size<spot_msgs::action::ExecuteDance_Feedback>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<spot_msgs::action::ExecuteDance_FeedbackMessage>
  : std::true_type {};

}  // namespace rosidl_generator_traits


namespace rosidl_generator_traits
{

template<>
struct is_action<spot_msgs::action::ExecuteDance>
  : std::true_type
{
};

template<>
struct is_action_goal<spot_msgs::action::ExecuteDance_Goal>
  : std::true_type
{
};

template<>
struct is_action_result<spot_msgs::action::ExecuteDance_Result>
  : std::true_type
{
};

template<>
struct is_action_feedback<spot_msgs::action::ExecuteDance_Feedback>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits


#endif  // SPOT_MSGS__ACTION__DETAIL__EXECUTE_DANCE__TRAITS_HPP_
