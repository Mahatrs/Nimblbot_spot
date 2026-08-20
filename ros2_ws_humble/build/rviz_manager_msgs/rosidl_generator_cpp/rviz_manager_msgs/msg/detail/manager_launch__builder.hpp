// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from rviz_manager_msgs:msg/ManagerLaunch.idl
// generated code does not contain a copyright notice

#ifndef RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__BUILDER_HPP_
#define RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "rviz_manager_msgs/msg/detail/manager_launch__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace rviz_manager_msgs
{

namespace msg
{

namespace builder
{

class Init_ManagerLaunch_timeout
{
public:
  explicit Init_ManagerLaunch_timeout(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  ::rviz_manager_msgs::msg::ManagerLaunch timeout(::rviz_manager_msgs::msg::ManagerLaunch::_timeout_type arg)
  {
    msg_.timeout = std::move(arg);
    return std::move(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_use_sim_time
{
public:
  explicit Init_ManagerLaunch_use_sim_time(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_timeout use_sim_time(::rviz_manager_msgs::msg::ManagerLaunch::_use_sim_time_type arg)
  {
    msg_.use_sim_time = std::move(arg);
    return Init_ManagerLaunch_timeout(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_session_name
{
public:
  explicit Init_ManagerLaunch_session_name(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_use_sim_time session_name(::rviz_manager_msgs::msg::ManagerLaunch::_session_name_type arg)
  {
    msg_.session_name = std::move(arg);
    return Init_ManagerLaunch_use_sim_time(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_working_dir
{
public:
  explicit Init_ManagerLaunch_working_dir(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_session_name working_dir(::rviz_manager_msgs::msg::ManagerLaunch::_working_dir_type arg)
  {
    msg_.working_dir = std::move(arg);
    return Init_ManagerLaunch_session_name(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_ros_arguments
{
public:
  explicit Init_ManagerLaunch_ros_arguments(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_working_dir ros_arguments(::rviz_manager_msgs::msg::ManagerLaunch::_ros_arguments_type arg)
  {
    msg_.ros_arguments = std::move(arg);
    return Init_ManagerLaunch_working_dir(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_arguments
{
public:
  explicit Init_ManagerLaunch_arguments(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_ros_arguments arguments(::rviz_manager_msgs::msg::ManagerLaunch::_arguments_type arg)
  {
    msg_.arguments = std::move(arg);
    return Init_ManagerLaunch_ros_arguments(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_executable
{
public:
  explicit Init_ManagerLaunch_executable(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_arguments executable(::rviz_manager_msgs::msg::ManagerLaunch::_executable_type arg)
  {
    msg_.executable = std::move(arg);
    return Init_ManagerLaunch_arguments(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_package
{
public:
  explicit Init_ManagerLaunch_package(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_executable package(::rviz_manager_msgs::msg::ManagerLaunch::_package_type arg)
  {
    msg_.package = std::move(arg);
    return Init_ManagerLaunch_executable(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_is_launch_file
{
public:
  explicit Init_ManagerLaunch_is_launch_file(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_package is_launch_file(::rviz_manager_msgs::msg::ManagerLaunch::_is_launch_file_type arg)
  {
    msg_.is_launch_file = std::move(arg);
    return Init_ManagerLaunch_package(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_bash_session
{
public:
  explicit Init_ManagerLaunch_bash_session(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_is_launch_file bash_session(::rviz_manager_msgs::msg::ManagerLaunch::_bash_session_type arg)
  {
    msg_.bash_session = std::move(arg);
    return Init_ManagerLaunch_is_launch_file(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_ns
{
public:
  explicit Init_ManagerLaunch_ns(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_bash_session ns(::rviz_manager_msgs::msg::ManagerLaunch::_ns_type arg)
  {
    msg_.ns = std::move(arg);
    return Init_ManagerLaunch_bash_session(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_action
{
public:
  explicit Init_ManagerLaunch_action(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_ns action(::rviz_manager_msgs::msg::ManagerLaunch::_action_type arg)
  {
    msg_.action = std::move(arg);
    return Init_ManagerLaunch_ns(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_id
{
public:
  explicit Init_ManagerLaunch_id(::rviz_manager_msgs::msg::ManagerLaunch & msg)
  : msg_(msg)
  {}
  Init_ManagerLaunch_action id(::rviz_manager_msgs::msg::ManagerLaunch::_id_type arg)
  {
    msg_.id = std::move(arg);
    return Init_ManagerLaunch_action(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

class Init_ManagerLaunch_header
{
public:
  Init_ManagerLaunch_header()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ManagerLaunch_id header(::rviz_manager_msgs::msg::ManagerLaunch::_header_type arg)
  {
    msg_.header = std::move(arg);
    return Init_ManagerLaunch_id(msg_);
  }

private:
  ::rviz_manager_msgs::msg::ManagerLaunch msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::rviz_manager_msgs::msg::ManagerLaunch>()
{
  return rviz_manager_msgs::msg::builder::Init_ManagerLaunch_header();
}

}  // namespace rviz_manager_msgs

#endif  // RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__BUILDER_HPP_
