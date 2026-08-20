// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from nanotec_msgs:msg/DriverStatus.idl
// generated code does not contain a copyright notice

#ifndef NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__STRUCT_HPP_
#define NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__struct.hpp"
// Member 'devices'
#include "nanotec_msgs/msg/detail/device_status__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__nanotec_msgs__msg__DriverStatus __attribute__((deprecated))
#else
# define DEPRECATED__nanotec_msgs__msg__DriverStatus __declspec(deprecated)
#endif

namespace nanotec_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct DriverStatus_
{
  using Type = DriverStatus_<ContainerAllocator>;

  explicit DriverStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : header(_init)
  {
    (void)_init;
  }

  explicit DriverStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : header(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _header_type =
    std_msgs::msg::Header_<ContainerAllocator>;
  _header_type header;
  using _devices_type =
    std::vector<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator>>>;
  _devices_type devices;

  // setters for named parameter idiom
  Type & set__header(
    const std_msgs::msg::Header_<ContainerAllocator> & _arg)
  {
    this->header = _arg;
    return *this;
  }
  Type & set__devices(
    const std::vector<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator>>> & _arg)
  {
    this->devices = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    nanotec_msgs::msg::DriverStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const nanotec_msgs::msg::DriverStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<nanotec_msgs::msg::DriverStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<nanotec_msgs::msg::DriverStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      nanotec_msgs::msg::DriverStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<nanotec_msgs::msg::DriverStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      nanotec_msgs::msg::DriverStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<nanotec_msgs::msg::DriverStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<nanotec_msgs::msg::DriverStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<nanotec_msgs::msg::DriverStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__nanotec_msgs__msg__DriverStatus
    std::shared_ptr<nanotec_msgs::msg::DriverStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__nanotec_msgs__msg__DriverStatus
    std::shared_ptr<nanotec_msgs::msg::DriverStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const DriverStatus_ & other) const
  {
    if (this->header != other.header) {
      return false;
    }
    if (this->devices != other.devices) {
      return false;
    }
    return true;
  }
  bool operator!=(const DriverStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct DriverStatus_

// alias to use template instance with default allocator
using DriverStatus =
  nanotec_msgs::msg::DriverStatus_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace nanotec_msgs

#endif  // NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__STRUCT_HPP_
