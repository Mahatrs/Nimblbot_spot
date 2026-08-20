// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/AcquireLease.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/acquire_lease__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_AcquireLease_Request_force
{
public:
  explicit Init_AcquireLease_Request_force(::spot_msgs::srv::AcquireLease_Request & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::AcquireLease_Request force(::spot_msgs::srv::AcquireLease_Request::_force_type arg)
  {
    msg_.force = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::AcquireLease_Request msg_;
};

class Init_AcquireLease_Request_resource_name
{
public:
  explicit Init_AcquireLease_Request_resource_name(::spot_msgs::srv::AcquireLease_Request & msg)
  : msg_(msg)
  {}
  Init_AcquireLease_Request_force resource_name(::spot_msgs::srv::AcquireLease_Request::_resource_name_type arg)
  {
    msg_.resource_name = std::move(arg);
    return Init_AcquireLease_Request_force(msg_);
  }

private:
  ::spot_msgs::srv::AcquireLease_Request msg_;
};

class Init_AcquireLease_Request_client_name
{
public:
  Init_AcquireLease_Request_client_name()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_AcquireLease_Request_resource_name client_name(::spot_msgs::srv::AcquireLease_Request::_client_name_type arg)
  {
    msg_.client_name = std::move(arg);
    return Init_AcquireLease_Request_resource_name(msg_);
  }

private:
  ::spot_msgs::srv::AcquireLease_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::AcquireLease_Request>()
{
  return spot_msgs::srv::builder::Init_AcquireLease_Request_client_name();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_AcquireLease_Response_lease
{
public:
  explicit Init_AcquireLease_Response_lease(::spot_msgs::srv::AcquireLease_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::AcquireLease_Response lease(::spot_msgs::srv::AcquireLease_Response::_lease_type arg)
  {
    msg_.lease = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::AcquireLease_Response msg_;
};

class Init_AcquireLease_Response_message
{
public:
  explicit Init_AcquireLease_Response_message(::spot_msgs::srv::AcquireLease_Response & msg)
  : msg_(msg)
  {}
  Init_AcquireLease_Response_lease message(::spot_msgs::srv::AcquireLease_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return Init_AcquireLease_Response_lease(msg_);
  }

private:
  ::spot_msgs::srv::AcquireLease_Response msg_;
};

class Init_AcquireLease_Response_success
{
public:
  Init_AcquireLease_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_AcquireLease_Response_message success(::spot_msgs::srv::AcquireLease_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_AcquireLease_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::AcquireLease_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::AcquireLease_Response>()
{
  return spot_msgs::srv::builder::Init_AcquireLease_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__BUILDER_HPP_
