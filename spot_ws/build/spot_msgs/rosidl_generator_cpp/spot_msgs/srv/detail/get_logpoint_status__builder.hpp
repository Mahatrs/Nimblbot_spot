// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/GetLogpointStatus.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__GET_LOGPOINT_STATUS__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__GET_LOGPOINT_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/get_logpoint_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_GetLogpointStatus_Request_name
{
public:
  Init_GetLogpointStatus_Request_name()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::GetLogpointStatus_Request name(::spot_msgs::srv::GetLogpointStatus_Request::_name_type arg)
  {
    msg_.name = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::GetLogpointStatus_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::GetLogpointStatus_Request>()
{
  return spot_msgs::srv::builder::Init_GetLogpointStatus_Request_name();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_GetLogpointStatus_Response_status
{
public:
  explicit Init_GetLogpointStatus_Response_status(::spot_msgs::srv::GetLogpointStatus_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::GetLogpointStatus_Response status(::spot_msgs::srv::GetLogpointStatus_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::GetLogpointStatus_Response msg_;
};

class Init_GetLogpointStatus_Response_message
{
public:
  explicit Init_GetLogpointStatus_Response_message(::spot_msgs::srv::GetLogpointStatus_Response & msg)
  : msg_(msg)
  {}
  Init_GetLogpointStatus_Response_status message(::spot_msgs::srv::GetLogpointStatus_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return Init_GetLogpointStatus_Response_status(msg_);
  }

private:
  ::spot_msgs::srv::GetLogpointStatus_Response msg_;
};

class Init_GetLogpointStatus_Response_success
{
public:
  Init_GetLogpointStatus_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetLogpointStatus_Response_message success(::spot_msgs::srv::GetLogpointStatus_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_GetLogpointStatus_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::GetLogpointStatus_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::GetLogpointStatus_Response>()
{
  return spot_msgs::srv::builder::Init_GetLogpointStatus_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__GET_LOGPOINT_STATUS__BUILDER_HPP_
