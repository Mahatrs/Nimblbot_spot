// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from synchros2_tutorials_interfaces_example:action/Wait.idl
// generated code does not contain a copyright notice

#ifndef SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__ACTION__DETAIL__WAIT__TRAITS_HPP_
#define SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__ACTION__DETAIL__WAIT__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace synchros2_tutorials_interfaces_example
{

namespace action
{

inline void to_flow_style_yaml(
  const Wait_Goal & msg,
  std::ostream & out)
{
  out << "{";
  // member: n_seconds_to_wait
  {
    out << "n_seconds_to_wait: ";
    rosidl_generator_traits::value_to_yaml(msg.n_seconds_to_wait, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Wait_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: n_seconds_to_wait
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "n_seconds_to_wait: ";
    rosidl_generator_traits::value_to_yaml(msg.n_seconds_to_wait, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Wait_Goal & msg, bool use_flow_style = false)
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

}  // namespace synchros2_tutorials_interfaces_example

namespace rosidl_generator_traits
{

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const synchros2_tutorials_interfaces_example::action::Wait_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  synchros2_tutorials_interfaces_example::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_yaml() instead")]]
inline std::string to_yaml(const synchros2_tutorials_interfaces_example::action::Wait_Goal & msg)
{
  return synchros2_tutorials_interfaces_example::action::to_yaml(msg);
}

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::action::Wait_Goal>()
{
  return "synchros2_tutorials_interfaces_example::action::Wait_Goal";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::action::Wait_Goal>()
{
  return "synchros2_tutorials_interfaces_example/action/Wait_Goal";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_Goal>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_Goal>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<synchros2_tutorials_interfaces_example::action::Wait_Goal>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace synchros2_tutorials_interfaces_example
{

namespace action
{

inline void to_flow_style_yaml(
  const Wait_Result & msg,
  std::ostream & out)
{
  out << "{";
  // member: n_seconds_waited
  {
    out << "n_seconds_waited: ";
    rosidl_generator_traits::value_to_yaml(msg.n_seconds_waited, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Wait_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: n_seconds_waited
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "n_seconds_waited: ";
    rosidl_generator_traits::value_to_yaml(msg.n_seconds_waited, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Wait_Result & msg, bool use_flow_style = false)
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

}  // namespace synchros2_tutorials_interfaces_example

namespace rosidl_generator_traits
{

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const synchros2_tutorials_interfaces_example::action::Wait_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  synchros2_tutorials_interfaces_example::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_yaml() instead")]]
inline std::string to_yaml(const synchros2_tutorials_interfaces_example::action::Wait_Result & msg)
{
  return synchros2_tutorials_interfaces_example::action::to_yaml(msg);
}

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::action::Wait_Result>()
{
  return "synchros2_tutorials_interfaces_example::action::Wait_Result";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::action::Wait_Result>()
{
  return "synchros2_tutorials_interfaces_example/action/Wait_Result";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_Result>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_Result>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<synchros2_tutorials_interfaces_example::action::Wait_Result>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace synchros2_tutorials_interfaces_example
{

namespace action
{

inline void to_flow_style_yaml(
  const Wait_Feedback & msg,
  std::ostream & out)
{
  out << "{";
  // member: n_seconds_remaining
  {
    out << "n_seconds_remaining: ";
    rosidl_generator_traits::value_to_yaml(msg.n_seconds_remaining, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Wait_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: n_seconds_remaining
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "n_seconds_remaining: ";
    rosidl_generator_traits::value_to_yaml(msg.n_seconds_remaining, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Wait_Feedback & msg, bool use_flow_style = false)
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

}  // namespace synchros2_tutorials_interfaces_example

namespace rosidl_generator_traits
{

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const synchros2_tutorials_interfaces_example::action::Wait_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  synchros2_tutorials_interfaces_example::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_yaml() instead")]]
inline std::string to_yaml(const synchros2_tutorials_interfaces_example::action::Wait_Feedback & msg)
{
  return synchros2_tutorials_interfaces_example::action::to_yaml(msg);
}

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::action::Wait_Feedback>()
{
  return "synchros2_tutorials_interfaces_example::action::Wait_Feedback";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::action::Wait_Feedback>()
{
  return "synchros2_tutorials_interfaces_example/action/Wait_Feedback";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_Feedback>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_Feedback>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<synchros2_tutorials_interfaces_example::action::Wait_Feedback>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'goal'
#include "synchros2_tutorials_interfaces_example/action/detail/wait__traits.hpp"

namespace synchros2_tutorials_interfaces_example
{

namespace action
{

inline void to_flow_style_yaml(
  const Wait_SendGoal_Request & msg,
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
  const Wait_SendGoal_Request & msg,
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

inline std::string to_yaml(const Wait_SendGoal_Request & msg, bool use_flow_style = false)
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

}  // namespace synchros2_tutorials_interfaces_example

namespace rosidl_generator_traits
{

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  synchros2_tutorials_interfaces_example::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_yaml() instead")]]
inline std::string to_yaml(const synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request & msg)
{
  return synchros2_tutorials_interfaces_example::action::to_yaml(msg);
}

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request>()
{
  return "synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request>()
{
  return "synchros2_tutorials_interfaces_example/action/Wait_SendGoal_Request";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request>
  : std::integral_constant<bool, has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_Goal>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request>
  : std::integral_constant<bool, has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_Goal>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__traits.hpp"

namespace synchros2_tutorials_interfaces_example
{

namespace action
{

inline void to_flow_style_yaml(
  const Wait_SendGoal_Response & msg,
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
  const Wait_SendGoal_Response & msg,
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

inline std::string to_yaml(const Wait_SendGoal_Response & msg, bool use_flow_style = false)
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

}  // namespace synchros2_tutorials_interfaces_example

namespace rosidl_generator_traits
{

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  synchros2_tutorials_interfaces_example::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_yaml() instead")]]
inline std::string to_yaml(const synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response & msg)
{
  return synchros2_tutorials_interfaces_example::action::to_yaml(msg);
}

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response>()
{
  return "synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response>()
{
  return "synchros2_tutorials_interfaces_example/action/Wait_SendGoal_Response";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::action::Wait_SendGoal>()
{
  return "synchros2_tutorials_interfaces_example::action::Wait_SendGoal";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::action::Wait_SendGoal>()
{
  return "synchros2_tutorials_interfaces_example/action/Wait_SendGoal";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_SendGoal>
  : std::integral_constant<
    bool,
    has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request>::value &&
    has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response>::value
  >
{
};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_SendGoal>
  : std::integral_constant<
    bool,
    has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request>::value &&
    has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response>::value
  >
{
};

template<>
struct is_service<synchros2_tutorials_interfaces_example::action::Wait_SendGoal>
  : std::true_type
{
};

template<>
struct is_service_request<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request>
  : std::true_type
{
};

template<>
struct is_service_response<synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"

namespace synchros2_tutorials_interfaces_example
{

namespace action
{

inline void to_flow_style_yaml(
  const Wait_GetResult_Request & msg,
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
  const Wait_GetResult_Request & msg,
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

inline std::string to_yaml(const Wait_GetResult_Request & msg, bool use_flow_style = false)
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

}  // namespace synchros2_tutorials_interfaces_example

namespace rosidl_generator_traits
{

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  synchros2_tutorials_interfaces_example::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_yaml() instead")]]
inline std::string to_yaml(const synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request & msg)
{
  return synchros2_tutorials_interfaces_example::action::to_yaml(msg);
}

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request>()
{
  return "synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request>()
{
  return "synchros2_tutorials_interfaces_example/action/Wait_GetResult_Request";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'result'
// already included above
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__traits.hpp"

namespace synchros2_tutorials_interfaces_example
{

namespace action
{

inline void to_flow_style_yaml(
  const Wait_GetResult_Response & msg,
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
  const Wait_GetResult_Response & msg,
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

inline std::string to_yaml(const Wait_GetResult_Response & msg, bool use_flow_style = false)
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

}  // namespace synchros2_tutorials_interfaces_example

namespace rosidl_generator_traits
{

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  synchros2_tutorials_interfaces_example::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_yaml() instead")]]
inline std::string to_yaml(const synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response & msg)
{
  return synchros2_tutorials_interfaces_example::action::to_yaml(msg);
}

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response>()
{
  return "synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response>()
{
  return "synchros2_tutorials_interfaces_example/action/Wait_GetResult_Response";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response>
  : std::integral_constant<bool, has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_Result>::value> {};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response>
  : std::integral_constant<bool, has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_Result>::value> {};

template<>
struct is_message<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::action::Wait_GetResult>()
{
  return "synchros2_tutorials_interfaces_example::action::Wait_GetResult";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::action::Wait_GetResult>()
{
  return "synchros2_tutorials_interfaces_example/action/Wait_GetResult";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_GetResult>
  : std::integral_constant<
    bool,
    has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request>::value &&
    has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response>::value
  >
{
};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_GetResult>
  : std::integral_constant<
    bool,
    has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request>::value &&
    has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response>::value
  >
{
};

template<>
struct is_service<synchros2_tutorials_interfaces_example::action::Wait_GetResult>
  : std::true_type
{
};

template<>
struct is_service_request<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request>
  : std::true_type
{
};

template<>
struct is_service_response<synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response>
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
// #include "synchros2_tutorials_interfaces_example/action/detail/wait__traits.hpp"

namespace synchros2_tutorials_interfaces_example
{

namespace action
{

inline void to_flow_style_yaml(
  const Wait_FeedbackMessage & msg,
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
  const Wait_FeedbackMessage & msg,
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

inline std::string to_yaml(const Wait_FeedbackMessage & msg, bool use_flow_style = false)
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

}  // namespace synchros2_tutorials_interfaces_example

namespace rosidl_generator_traits
{

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  synchros2_tutorials_interfaces_example::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use synchros2_tutorials_interfaces_example::action::to_yaml() instead")]]
inline std::string to_yaml(const synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage & msg)
{
  return synchros2_tutorials_interfaces_example::action::to_yaml(msg);
}

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage>()
{
  return "synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage>()
{
  return "synchros2_tutorials_interfaces_example/action/Wait_FeedbackMessage";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage>
  : std::integral_constant<bool, has_fixed_size<synchros2_tutorials_interfaces_example::action::Wait_Feedback>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage>
  : std::integral_constant<bool, has_bounded_size<synchros2_tutorials_interfaces_example::action::Wait_Feedback>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage>
  : std::true_type {};

}  // namespace rosidl_generator_traits


namespace rosidl_generator_traits
{

template<>
struct is_action<synchros2_tutorials_interfaces_example::action::Wait>
  : std::true_type
{
};

template<>
struct is_action_goal<synchros2_tutorials_interfaces_example::action::Wait_Goal>
  : std::true_type
{
};

template<>
struct is_action_result<synchros2_tutorials_interfaces_example::action::Wait_Result>
  : std::true_type
{
};

template<>
struct is_action_feedback<synchros2_tutorials_interfaces_example::action::Wait_Feedback>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits


#endif  // SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__ACTION__DETAIL__WAIT__TRAITS_HPP_
