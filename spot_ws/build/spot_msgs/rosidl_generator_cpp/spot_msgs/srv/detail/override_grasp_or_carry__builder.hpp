// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/OverrideGraspOrCarry.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__OVERRIDE_GRASP_OR_CARRY__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__OVERRIDE_GRASP_OR_CARRY__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/override_grasp_or_carry__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_OverrideGraspOrCarry_Request_carry_override
{
public:
  explicit Init_OverrideGraspOrCarry_Request_carry_override(::spot_msgs::srv::OverrideGraspOrCarry_Request & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::OverrideGraspOrCarry_Request carry_override(::spot_msgs::srv::OverrideGraspOrCarry_Request::_carry_override_type arg)
  {
    msg_.carry_override = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::OverrideGraspOrCarry_Request msg_;
};

class Init_OverrideGraspOrCarry_Request_grasp_override
{
public:
  Init_OverrideGraspOrCarry_Request_grasp_override()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_OverrideGraspOrCarry_Request_carry_override grasp_override(::spot_msgs::srv::OverrideGraspOrCarry_Request::_grasp_override_type arg)
  {
    msg_.grasp_override = std::move(arg);
    return Init_OverrideGraspOrCarry_Request_carry_override(msg_);
  }

private:
  ::spot_msgs::srv::OverrideGraspOrCarry_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::OverrideGraspOrCarry_Request>()
{
  return spot_msgs::srv::builder::Init_OverrideGraspOrCarry_Request_grasp_override();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_OverrideGraspOrCarry_Response_message
{
public:
  explicit Init_OverrideGraspOrCarry_Response_message(::spot_msgs::srv::OverrideGraspOrCarry_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::OverrideGraspOrCarry_Response message(::spot_msgs::srv::OverrideGraspOrCarry_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::OverrideGraspOrCarry_Response msg_;
};

class Init_OverrideGraspOrCarry_Response_success
{
public:
  Init_OverrideGraspOrCarry_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_OverrideGraspOrCarry_Response_message success(::spot_msgs::srv::OverrideGraspOrCarry_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_OverrideGraspOrCarry_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::OverrideGraspOrCarry_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::OverrideGraspOrCarry_Response>()
{
  return spot_msgs::srv::builder::Init_OverrideGraspOrCarry_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__OVERRIDE_GRASP_OR_CARRY__BUILDER_HPP_
