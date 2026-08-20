// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from synchros2_tutorials_interfaces_example:msg/String.idl
// generated code does not contain a copyright notice

#ifndef SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__BUILDER_HPP_
#define SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "synchros2_tutorials_interfaces_example/msg/detail/string__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace synchros2_tutorials_interfaces_example
{

namespace msg
{

namespace builder
{

class Init_String_data
{
public:
  Init_String_data()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::synchros2_tutorials_interfaces_example::msg::String data(::synchros2_tutorials_interfaces_example::msg::String::_data_type arg)
  {
    msg_.data = std::move(arg);
    return std::move(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::msg::String msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::synchros2_tutorials_interfaces_example::msg::String>()
{
  return synchros2_tutorials_interfaces_example::msg::builder::Init_String_data();
}

}  // namespace synchros2_tutorials_interfaces_example

#endif  // SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__BUILDER_HPP_
