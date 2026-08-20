// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/SetStairsMode.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__SET_STAIRS_MODE__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__SET_STAIRS_MODE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/set_stairs_mode__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_SetStairsMode_Request_stairs_mode
{
public:
  Init_SetStairsMode_Request_stairs_mode()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::SetStairsMode_Request stairs_mode(::spot_msgs::srv::SetStairsMode_Request::_stairs_mode_type arg)
  {
    msg_.stairs_mode = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::SetStairsMode_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::SetStairsMode_Request>()
{
  return spot_msgs::srv::builder::Init_SetStairsMode_Request_stairs_mode();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_SetStairsMode_Response_message
{
public:
  explicit Init_SetStairsMode_Response_message(::spot_msgs::srv::SetStairsMode_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::SetStairsMode_Response message(::spot_msgs::srv::SetStairsMode_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::SetStairsMode_Response msg_;
};

class Init_SetStairsMode_Response_success
{
public:
  Init_SetStairsMode_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetStairsMode_Response_message success(::spot_msgs::srv::SetStairsMode_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SetStairsMode_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::SetStairsMode_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::SetStairsMode_Response>()
{
  return spot_msgs::srv::builder::Init_SetStairsMode_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__SET_STAIRS_MODE__BUILDER_HPP_
