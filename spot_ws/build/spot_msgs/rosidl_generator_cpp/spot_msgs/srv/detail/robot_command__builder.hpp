// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/RobotCommand.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/robot_command__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_RobotCommand_Request_duration
{
public:
  explicit Init_RobotCommand_Request_duration(::spot_msgs::srv::RobotCommand_Request & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::RobotCommand_Request duration(::spot_msgs::srv::RobotCommand_Request::_duration_type arg)
  {
    msg_.duration = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::RobotCommand_Request msg_;
};

class Init_RobotCommand_Request_command
{
public:
  Init_RobotCommand_Request_command()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_RobotCommand_Request_duration command(::spot_msgs::srv::RobotCommand_Request::_command_type arg)
  {
    msg_.command = std::move(arg);
    return Init_RobotCommand_Request_duration(msg_);
  }

private:
  ::spot_msgs::srv::RobotCommand_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::RobotCommand_Request>()
{
  return spot_msgs::srv::builder::Init_RobotCommand_Request_command();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_RobotCommand_Response_robot_command_id
{
public:
  explicit Init_RobotCommand_Response_robot_command_id(::spot_msgs::srv::RobotCommand_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::RobotCommand_Response robot_command_id(::spot_msgs::srv::RobotCommand_Response::_robot_command_id_type arg)
  {
    msg_.robot_command_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::RobotCommand_Response msg_;
};

class Init_RobotCommand_Response_message
{
public:
  explicit Init_RobotCommand_Response_message(::spot_msgs::srv::RobotCommand_Response & msg)
  : msg_(msg)
  {}
  Init_RobotCommand_Response_robot_command_id message(::spot_msgs::srv::RobotCommand_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return Init_RobotCommand_Response_robot_command_id(msg_);
  }

private:
  ::spot_msgs::srv::RobotCommand_Response msg_;
};

class Init_RobotCommand_Response_success
{
public:
  Init_RobotCommand_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_RobotCommand_Response_message success(::spot_msgs::srv::RobotCommand_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_RobotCommand_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::RobotCommand_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::RobotCommand_Response>()
{
  return spot_msgs::srv::builder::Init_RobotCommand_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__BUILDER_HPP_
