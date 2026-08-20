// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from spot_msgs:srv/RobotCommand.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__STRUCT_HPP_
#define SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'command'
#include "bosdyn_api_msgs/msg/detail/robot_command__struct.hpp"
// Member 'duration'
#include "builtin_interfaces/msg/detail/duration__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__RobotCommand_Request __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__RobotCommand_Request __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct RobotCommand_Request_
{
  using Type = RobotCommand_Request_<ContainerAllocator>;

  explicit RobotCommand_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : command(_init),
    duration(_init)
  {
    (void)_init;
  }

  explicit RobotCommand_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : command(_alloc, _init),
    duration(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _command_type =
    bosdyn_api_msgs::msg::RobotCommand_<ContainerAllocator>;
  _command_type command;
  using _duration_type =
    builtin_interfaces::msg::Duration_<ContainerAllocator>;
  _duration_type duration;

  // setters for named parameter idiom
  Type & set__command(
    const bosdyn_api_msgs::msg::RobotCommand_<ContainerAllocator> & _arg)
  {
    this->command = _arg;
    return *this;
  }
  Type & set__duration(
    const builtin_interfaces::msg::Duration_<ContainerAllocator> & _arg)
  {
    this->duration = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::RobotCommand_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::RobotCommand_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::RobotCommand_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::RobotCommand_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::RobotCommand_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::RobotCommand_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::RobotCommand_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::RobotCommand_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::RobotCommand_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::RobotCommand_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__RobotCommand_Request
    std::shared_ptr<spot_msgs::srv::RobotCommand_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__RobotCommand_Request
    std::shared_ptr<spot_msgs::srv::RobotCommand_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const RobotCommand_Request_ & other) const
  {
    if (this->command != other.command) {
      return false;
    }
    if (this->duration != other.duration) {
      return false;
    }
    return true;
  }
  bool operator!=(const RobotCommand_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct RobotCommand_Request_

// alias to use template instance with default allocator
using RobotCommand_Request =
  spot_msgs::srv::RobotCommand_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs


#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__RobotCommand_Response __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__RobotCommand_Response __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct RobotCommand_Response_
{
  using Type = RobotCommand_Response_<ContainerAllocator>;

  explicit RobotCommand_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
      this->robot_command_id = 0ul;
    }
  }

  explicit RobotCommand_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : message(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
      this->robot_command_id = 0ul;
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _message_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _message_type message;
  using _robot_command_id_type =
    uint32_t;
  _robot_command_id_type robot_command_id;

  // setters for named parameter idiom
  Type & set__success(
    const bool & _arg)
  {
    this->success = _arg;
    return *this;
  }
  Type & set__message(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->message = _arg;
    return *this;
  }
  Type & set__robot_command_id(
    const uint32_t & _arg)
  {
    this->robot_command_id = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::RobotCommand_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::RobotCommand_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::RobotCommand_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::RobotCommand_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::RobotCommand_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::RobotCommand_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::RobotCommand_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::RobotCommand_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::RobotCommand_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::RobotCommand_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__RobotCommand_Response
    std::shared_ptr<spot_msgs::srv::RobotCommand_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__RobotCommand_Response
    std::shared_ptr<spot_msgs::srv::RobotCommand_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const RobotCommand_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    if (this->robot_command_id != other.robot_command_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const RobotCommand_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct RobotCommand_Response_

// alias to use template instance with default allocator
using RobotCommand_Response =
  spot_msgs::srv::RobotCommand_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs

namespace spot_msgs
{

namespace srv
{

struct RobotCommand
{
  using Request = spot_msgs::srv::RobotCommand_Request;
  using Response = spot_msgs::srv::RobotCommand_Response;
};

}  // namespace srv

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__ROBOT_COMMAND__STRUCT_HPP_
