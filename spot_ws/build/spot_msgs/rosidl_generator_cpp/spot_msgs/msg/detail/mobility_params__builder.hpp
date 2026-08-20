// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:msg/MobilityParams.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__MSG__DETAIL__MOBILITY_PARAMS__BUILDER_HPP_
#define SPOT_MSGS__MSG__DETAIL__MOBILITY_PARAMS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/msg/detail/mobility_params__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace msg
{

namespace builder
{

class Init_MobilityParams_stairs_mode
{
public:
  explicit Init_MobilityParams_stairs_mode(::spot_msgs::msg::MobilityParams & msg)
  : msg_(msg)
  {}
  ::spot_msgs::msg::MobilityParams stairs_mode(::spot_msgs::msg::MobilityParams::_stairs_mode_type arg)
  {
    msg_.stairs_mode = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::msg::MobilityParams msg_;
};

class Init_MobilityParams_locomotion_hint
{
public:
  explicit Init_MobilityParams_locomotion_hint(::spot_msgs::msg::MobilityParams & msg)
  : msg_(msg)
  {}
  Init_MobilityParams_stairs_mode locomotion_hint(::spot_msgs::msg::MobilityParams::_locomotion_hint_type arg)
  {
    msg_.locomotion_hint = std::move(arg);
    return Init_MobilityParams_stairs_mode(msg_);
  }

private:
  ::spot_msgs::msg::MobilityParams msg_;
};

class Init_MobilityParams_body_control
{
public:
  Init_MobilityParams_body_control()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MobilityParams_locomotion_hint body_control(::spot_msgs::msg::MobilityParams::_body_control_type arg)
  {
    msg_.body_control = std::move(arg);
    return Init_MobilityParams_locomotion_hint(msg_);
  }

private:
  ::spot_msgs::msg::MobilityParams msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::msg::MobilityParams>()
{
  return spot_msgs::msg::builder::Init_MobilityParams_body_control();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__MSG__DETAIL__MOBILITY_PARAMS__BUILDER_HPP_
