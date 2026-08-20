// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/ListLogpoints.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__LIST_LOGPOINTS__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__LIST_LOGPOINTS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/list_logpoints__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::ListLogpoints_Request>()
{
  return ::spot_msgs::srv::ListLogpoints_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_ListLogpoints_Response_logpoints
{
public:
  explicit Init_ListLogpoints_Response_logpoints(::spot_msgs::srv::ListLogpoints_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::ListLogpoints_Response logpoints(::spot_msgs::srv::ListLogpoints_Response::_logpoints_type arg)
  {
    msg_.logpoints = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::ListLogpoints_Response msg_;
};

class Init_ListLogpoints_Response_message
{
public:
  explicit Init_ListLogpoints_Response_message(::spot_msgs::srv::ListLogpoints_Response & msg)
  : msg_(msg)
  {}
  Init_ListLogpoints_Response_logpoints message(::spot_msgs::srv::ListLogpoints_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return Init_ListLogpoints_Response_logpoints(msg_);
  }

private:
  ::spot_msgs::srv::ListLogpoints_Response msg_;
};

class Init_ListLogpoints_Response_success
{
public:
  Init_ListLogpoints_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ListLogpoints_Response_message success(::spot_msgs::srv::ListLogpoints_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_ListLogpoints_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::ListLogpoints_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::ListLogpoints_Response>()
{
  return spot_msgs::srv::builder::Init_ListLogpoints_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__LIST_LOGPOINTS__BUILDER_HPP_
