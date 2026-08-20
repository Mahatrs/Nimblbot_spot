// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from spot_msgs:msg/JointCommand.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__MSG__DETAIL__JOINT_COMMAND__STRUCT_HPP_
#define SPOT_MSGS__MSG__DETAIL__JOINT_COMMAND__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__spot_msgs__msg__JointCommand __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__msg__JointCommand __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct JointCommand_
{
  using Type = JointCommand_<ContainerAllocator>;

  explicit JointCommand_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_init;
  }

  explicit JointCommand_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_init;
    (void)_alloc;
  }

  // field types and members
  using _name_type =
    std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>>;
  _name_type name;
  using _position_type =
    std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>>;
  _position_type position;
  using _velocity_type =
    std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>>;
  _velocity_type velocity;
  using _effort_type =
    std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>>;
  _effort_type effort;
  using _k_q_p_type =
    std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>>;
  _k_q_p_type k_q_p;
  using _k_qd_p_type =
    std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>>;
  _k_qd_p_type k_qd_p;

  // setters for named parameter idiom
  Type & set__name(
    const std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>> & _arg)
  {
    this->name = _arg;
    return *this;
  }
  Type & set__position(
    const std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>> & _arg)
  {
    this->position = _arg;
    return *this;
  }
  Type & set__velocity(
    const std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>> & _arg)
  {
    this->velocity = _arg;
    return *this;
  }
  Type & set__effort(
    const std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>> & _arg)
  {
    this->effort = _arg;
    return *this;
  }
  Type & set__k_q_p(
    const std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>> & _arg)
  {
    this->k_q_p = _arg;
    return *this;
  }
  Type & set__k_qd_p(
    const std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>> & _arg)
  {
    this->k_qd_p = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::msg::JointCommand_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::msg::JointCommand_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::msg::JointCommand_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::msg::JointCommand_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::msg::JointCommand_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::msg::JointCommand_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::msg::JointCommand_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::msg::JointCommand_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::msg::JointCommand_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::msg::JointCommand_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__msg__JointCommand
    std::shared_ptr<spot_msgs::msg::JointCommand_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__msg__JointCommand
    std::shared_ptr<spot_msgs::msg::JointCommand_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const JointCommand_ & other) const
  {
    if (this->name != other.name) {
      return false;
    }
    if (this->position != other.position) {
      return false;
    }
    if (this->velocity != other.velocity) {
      return false;
    }
    if (this->effort != other.effort) {
      return false;
    }
    if (this->k_q_p != other.k_q_p) {
      return false;
    }
    if (this->k_qd_p != other.k_qd_p) {
      return false;
    }
    return true;
  }
  bool operator!=(const JointCommand_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct JointCommand_

// alias to use template instance with default allocator
using JointCommand =
  spot_msgs::msg::JointCommand_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace spot_msgs

#endif  // SPOT_MSGS__MSG__DETAIL__JOINT_COMMAND__STRUCT_HPP_
