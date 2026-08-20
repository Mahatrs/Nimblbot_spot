// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from rviz_manager_msgs:msg/ManagerStatus.idl
// generated code does not contain a copyright notice

#ifndef RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__BUILDER_HPP_
#define RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "rviz_manager_msgs/msg/detail/manager_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace rviz_manager_msgs
{

namespace msg
{

namespace builder
{

class Init_ManagerStatus_message
{
public:
  explicit Init_ManagerStatus_message(::rviz_manager_msgs::msg::ManagerStatus & msg)
  : msg_(msg)
  {}
  ::rviz_manager_msgs::msg::ManagerStatus message(::rviz_manager_msgs::msg::ManagerStatus::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerStatus msg_;
};

class Init_ManagerStatus_status
{
public:
  explicit Init_ManagerStatus_status(::rviz_manager_msgs::msg::ManagerStatus & msg)
  : msg_(msg)
  {}
  Init_ManagerStatus_message status(::rviz_manager_msgs::msg::ManagerStatus::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_ManagerStatus_message(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerStatus msg_;
};

class Init_ManagerStatus_id
{
public:
  Init_ManagerStatus_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ManagerStatus_status id(::rviz_manager_msgs::msg::ManagerStatus::_id_type arg)
  {
    msg_.id = std::move(arg);
    return Init_ManagerStatus_status(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::rviz_manager_msgs::msg::ManagerStatus>()
{
  return rviz_manager_msgs::msg::builder::Init_ManagerStatus_id();
}

}  // namespace rviz_manager_msgs

#endif  // RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__BUILDER_HPP_
