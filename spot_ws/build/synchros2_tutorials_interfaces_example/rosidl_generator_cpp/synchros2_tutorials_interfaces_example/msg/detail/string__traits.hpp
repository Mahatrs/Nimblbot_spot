// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from synchros2_tutorials_interfaces_example:msg/String.idl
// generated code does not contain a copyright notice

#ifndef SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__TRAITS_HPP_
#define SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "synchros2_tutorials_interfaces_example/msg/detail/string__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace synchros2_tutorials_interfaces_example
{

namespace msg
{

inline void to_flow_style_yaml(
  const String & msg,
  std::ostream & out)
{
  out << "{";
  // member: data
  {
    out << "data: ";
    rosidl_generator_traits::value_to_yaml(msg.data, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const String & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: data
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "data: ";
    rosidl_generator_traits::value_to_yaml(msg.data, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const String & msg, bool use_flow_style = false)
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

}  // namespace synchros2_tutorials_interfaces_example

namespace rosidl_generator_traits
{

[[deprecated("use synchros2_tutorials_interfaces_example::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const synchros2_tutorials_interfaces_example::msg::String & msg,
  std::ostream & out, size_t indentation = 0)
{
  synchros2_tutorials_interfaces_example::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use synchros2_tutorials_interfaces_example::msg::to_yaml() instead")]]
inline std::string to_yaml(const synchros2_tutorials_interfaces_example::msg::String & msg)
{
  return synchros2_tutorials_interfaces_example::msg::to_yaml(msg);
}

template<>
inline const char * data_type<synchros2_tutorials_interfaces_example::msg::String>()
{
  return "synchros2_tutorials_interfaces_example::msg::String";
}

template<>
inline const char * name<synchros2_tutorials_interfaces_example::msg::String>()
{
  return "synchros2_tutorials_interfaces_example/msg/String";
}

template<>
struct has_fixed_size<synchros2_tutorials_interfaces_example::msg::String>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<synchros2_tutorials_interfaces_example::msg::String>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<synchros2_tutorials_interfaces_example::msg::String>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__TRAITS_HPP_
