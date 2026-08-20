// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from rviz_manager_msgs:msg/ManagerStatus.idl
// generated code does not contain a copyright notice

#ifndef RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__STRUCT_HPP_
#define RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__rviz_manager_msgs__msg__ManagerStatus __attribute__((deprecated))
#else
# define DEPRECATED__rviz_manager_msgs__msg__ManagerStatus __declspec(deprecated)
#endif

namespace rviz_manager_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct ManagerStatus_
{
  using Type = ManagerStatus_<ContainerAllocator>;

  explicit ManagerStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->id = 0l;
      this->status = "";
      this->message = "";
    }
  }

  explicit ManagerStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : status(_alloc),
    message(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->id = 0l;
      this->status = "";
      this->message = "";
    }
  }

  // field types and members
  using _id_type =
    int32_t;
  _id_type id;
  using _status_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _status_type status;
  using _message_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _message_type message;

  // setters for named parameter idiom
  Type & set__id(
    const int32_t & _arg)
  {
    this->id = _arg;
    return *this;
  }
  Type & set__status(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->status = _arg;
    return *this;
  }
  Type & set__message(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->message = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__rviz_manager_msgs__msg__ManagerStatus
    std::shared_ptr<rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__rviz_manager_msgs__msg__ManagerStatus
    std::shared_ptr<rviz_manager_msgs::msg::ManagerStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ManagerStatus_ & other) const
  {
    if (this->id != other.id) {
      return false;
    }
    if (this->status != other.status) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    return true;
  }
  bool operator!=(const ManagerStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ManagerStatus_

// alias to use template instance with default allocator
using ManagerStatus =
  rviz_manager_msgs::msg::ManagerStatus_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace rviz_manager_msgs

#endif  // RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__STRUCT_HPP_
