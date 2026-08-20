// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:msg/JointCommand.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__MSG__DETAIL__JOINT_COMMAND__BUILDER_HPP_
#define SPOT_MSGS__MSG__DETAIL__JOINT_COMMAND__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/msg/detail/joint_command__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace msg
{

namespace builder
{

class Init_JointCommand_k_qd_p
{
public:
  explicit Init_JointCommand_k_qd_p(::spot_msgs::msg::JointCommand & msg)
  : msg_(msg)
  {}
  ::spot_msgs::msg::JointCommand k_qd_p(::spot_msgs::msg::JointCommand::_k_qd_p_type arg)
  {
    msg_.k_qd_p = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::msg::JointCommand msg_;
};

class Init_JointCommand_k_q_p
{
public:
  explicit Init_JointCommand_k_q_p(::spot_msgs::msg::JointCommand & msg)
  : msg_(msg)
  {}
  Init_JointCommand_k_qd_p k_q_p(::spot_msgs::msg::JointCommand::_k_q_p_type arg)
  {
    msg_.k_q_p = std::move(arg);
    return Init_JointCommand_k_qd_p(msg_);
  }

private:
  ::spot_msgs::msg::JointCommand msg_;
};

class Init_JointCommand_effort
{
public:
  explicit Init_JointCommand_effort(::spot_msgs::msg::JointCommand & msg)
  : msg_(msg)
  {}
  Init_JointCommand_k_q_p effort(::spot_msgs::msg::JointCommand::_effort_type arg)
  {
    msg_.effort = std::move(arg);
    return Init_JointCommand_k_q_p(msg_);
  }

private:
  ::spot_msgs::msg::JointCommand msg_;
};

class Init_JointCommand_velocity
{
public:
  explicit Init_JointCommand_velocity(::spot_msgs::msg::JointCommand & msg)
  : msg_(msg)
  {}
  Init_JointCommand_effort velocity(::spot_msgs::msg::JointCommand::_velocity_type arg)
  {
    msg_.velocity = std::move(arg);
    return Init_JointCommand_effort(msg_);
  }

private:
  ::spot_msgs::msg::JointCommand msg_;
};

class Init_JointCommand_position
{
public:
  explicit Init_JointCommand_position(::spot_msgs::msg::JointCommand & msg)
  : msg_(msg)
  {}
  Init_JointCommand_velocity position(::spot_msgs::msg::JointCommand::_position_type arg)
  {
    msg_.position = std::move(arg);
    return Init_JointCommand_velocity(msg_);
  }

private:
  ::spot_msgs::msg::JointCommand msg_;
};

class Init_JointCommand_name
{
public:
  Init_JointCommand_name()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_JointCommand_position name(::spot_msgs::msg::JointCommand::_name_type arg)
  {
    msg_.name = std::move(arg);
    return Init_JointCommand_position(msg_);
  }

private:
  ::spot_msgs::msg::JointCommand msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::msg::JointCommand>()
{
  return spot_msgs::msg::builder::Init_JointCommand_name();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__MSG__DETAIL__JOINT_COMMAND__BUILDER_HPP_
