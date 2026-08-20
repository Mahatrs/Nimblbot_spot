// generated from rosidl_typesupport_fastrtps_cpp/resource/idl__rosidl_typesupport_fastrtps_cpp.hpp.em
// with input from synchros2_tutorials_interfaces_example:msg/String.idl
// generated code does not contain a copyright notice

#ifndef SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_
#define SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_

#include "rosidl_runtime_c/message_type_support_struct.h"
#include "rosidl_typesupport_interface/macros.h"
#include "synchros2_tutorials_interfaces_example/msg/rosidl_typesupport_fastrtps_cpp__visibility_control.h"
#include "synchros2_tutorials_interfaces_example/msg/detail/string__struct.hpp"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

#include "fastcdr/Cdr.h"

namespace synchros2_tutorials_interfaces_example
{

namespace msg
{

namespace typesupport_fastrtps_cpp
{

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_synchros2_tutorials_interfaces_example
cdr_serialize(
  const synchros2_tutorials_interfaces_example::msg::String & ros_message,
  eprosima::fastcdr::Cdr & cdr);

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_synchros2_tutorials_interfaces_example
cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  synchros2_tutorials_interfaces_example::msg::String & ros_message);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_synchros2_tutorials_interfaces_example
get_serialized_size(
  const synchros2_tutorials_interfaces_example::msg::String & ros_message,
  size_t current_alignment);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_synchros2_tutorials_interfaces_example
max_serialized_size_String(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

}  // namespace typesupport_fastrtps_cpp

}  // namespace msg

}  // namespace synchros2_tutorials_interfaces_example

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_synchros2_tutorials_interfaces_example
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, synchros2_tutorials_interfaces_example, msg, String)();

#ifdef __cplusplus
}
#endif

#endif  // SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_
