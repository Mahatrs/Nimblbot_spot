// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/SetStandHeight.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__SET_STAND_HEIGHT__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__SET_STAND_HEIGHT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/set_stand_height__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_SetStandHeight_Request_height
{
public:
  Init_SetStandHeight_Request_height()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::SetStandHeight_Request height(::spot_msgs::srv::SetStandHeight_Request::_height_type arg)
  {
    msg_.height = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::SetStandHeight_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::SetStandHeight_Request>()
{
  return spot_msgs::srv::builder::Init_SetStandHeight_Request_height();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_SetStandHeight_Response_message
{
public:
  explicit Init_SetStandHeight_Response_message(::spot_msgs::srv::SetStandHeight_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::SetStandHeight_Response message(::spot_msgs::srv::SetStandHeight_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::SetStandHeight_Response msg_;
};

class Init_SetStandHeight_Response_success
{
public:
  Init_SetStandHeight_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetStandHeight_Response_message success(::spot_msgs::srv::SetStandHeight_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SetStandHeight_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::SetStandHeight_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::SetStandHeight_Response>()
{
  return spot_msgs::srv::builder::Init_SetStandHeight_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__SET_STAND_HEIGHT__BUILDER_HPP_
