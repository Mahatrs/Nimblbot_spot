// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from nanotec_msgs:msg/DriverStatus.idl
// generated code does not contain a copyright notice

#ifndef NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__BUILDER_HPP_
#define NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "nanotec_msgs/msg/detail/driver_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace nanotec_msgs
{

namespace msg
{

namespace builder
{

class Init_DriverStatus_devices
{
public:
  explicit Init_DriverStatus_devices(::nanotec_msgs::msg::DriverStatus & msg)
  : msg_(msg)
  {}
  ::nanotec_msgs::msg::DriverStatus devices(::nanotec_msgs::msg::DriverStatus::_devices_type arg)
  {
    msg_.devices = std::move(arg);
    return std::move(msg_);
  }

private:
  ::nanotec_msgs::msg::DriverStatus msg_;
};

class Init_DriverStatus_header
{
public:
  Init_DriverStatus_header()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DriverStatus_devices header(::nanotec_msgs::msg::DriverStatus::_header_type arg)
  {
    msg_.header = std::move(arg);
    return Init_DriverStatus_devices(msg_);
  }

private:
  ::nanotec_msgs::msg::DriverStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::nanotec_msgs::msg::DriverStatus>()
{
  return nanotec_msgs::msg::builder::Init_DriverStatus_header();
}

}  // namespace nanotec_msgs

#endif  // NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__BUILDER_HPP_
