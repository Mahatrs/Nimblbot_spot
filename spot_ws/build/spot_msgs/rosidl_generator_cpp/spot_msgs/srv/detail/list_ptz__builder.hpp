// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/ListPtz.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__LIST_PTZ__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__LIST_PTZ__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/list_ptz__struct.hpp"
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
auto build<::spot_msgs::srv::ListPtz_Request>()
{
  return ::spot_msgs::srv::ListPtz_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_ListPtz_Response_descriptions
{
public:
  explicit Init_ListPtz_Response_descriptions(::spot_msgs::srv::ListPtz_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::ListPtz_Response descriptions(::spot_msgs::srv::ListPtz_Response::_descriptions_type arg)
  {
    msg_.descriptions = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::ListPtz_Response msg_;
};

class Init_ListPtz_Response_message
{
public:
  explicit Init_ListPtz_Response_message(::spot_msgs::srv::ListPtz_Response & msg)
  : msg_(msg)
  {}
  Init_ListPtz_Response_descriptions message(::spot_msgs::srv::ListPtz_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return Init_ListPtz_Response_descriptions(msg_);
  }

private:
  ::spot_msgs::srv::ListPtz_Response msg_;
};

class Init_ListPtz_Response_success
{
public:
  Init_ListPtz_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ListPtz_Response_message success(::spot_msgs::srv::ListPtz_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_ListPtz_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::ListPtz_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::ListPtz_Response>()
{
  return spot_msgs::srv::builder::Init_ListPtz_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__LIST_PTZ__BUILDER_HPP_
