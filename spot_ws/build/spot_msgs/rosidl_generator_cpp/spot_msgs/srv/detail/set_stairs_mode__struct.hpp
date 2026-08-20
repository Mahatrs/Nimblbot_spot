// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from spot_msgs:srv/SetStairsMode.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__SET_STAIRS_MODE__STRUCT_HPP_
#define SPOT_MSGS__SRV__DETAIL__SET_STAIRS_MODE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'stairs_mode'
#include "bosdyn_spot_api_msgs/msg/detail/mobility_params_stairs_mode__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__SetStairsMode_Request __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__SetStairsMode_Request __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetStairsMode_Request_
{
  using Type = SetStairsMode_Request_<ContainerAllocator>;

  explicit SetStairsMode_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : stairs_mode(_init)
  {
    (void)_init;
  }

  explicit SetStairsMode_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : stairs_mode(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _stairs_mode_type =
    bosdyn_spot_api_msgs::msg::MobilityParamsStairsMode_<ContainerAllocator>;
  _stairs_mode_type stairs_mode;

  // setters for named parameter idiom
  Type & set__stairs_mode(
    const bosdyn_spot_api_msgs::msg::MobilityParamsStairsMode_<ContainerAllocator> & _arg)
  {
    this->stairs_mode = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__SetStairsMode_Request
    std::shared_ptr<spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__SetStairsMode_Request
    std::shared_ptr<spot_msgs::srv::SetStairsMode_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetStairsMode_Request_ & other) const
  {
    if (this->stairs_mode != other.stairs_mode) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetStairsMode_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetStairsMode_Request_

// alias to use template instance with default allocator
using SetStairsMode_Request =
  spot_msgs::srv::SetStairsMode_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs


#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__SetStairsMode_Response __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__SetStairsMode_Response __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetStairsMode_Response_
{
  using Type = SetStairsMode_Response_<ContainerAllocator>;

  explicit SetStairsMode_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  explicit SetStairsMode_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : message(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _message_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _message_type message;

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

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__SetStairsMode_Response
    std::shared_ptr<spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__SetStairsMode_Response
    std::shared_ptr<spot_msgs::srv::SetStairsMode_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetStairsMode_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetStairsMode_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetStairsMode_Response_

// alias to use template instance with default allocator
using SetStairsMode_Response =
  spot_msgs::srv::SetStairsMode_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs

namespace spot_msgs
{

namespace srv
{

struct SetStairsMode
{
  using Request = spot_msgs::srv::SetStairsMode_Request;
  using Response = spot_msgs::srv::SetStairsMode_Response;
};

}  // namespace srv

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__SET_STAIRS_MODE__STRUCT_HPP_
