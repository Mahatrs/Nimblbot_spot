// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from spot_msgs:srv/StoreLogpoint.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__STORE_LOGPOINT__STRUCT_HPP_
#define SPOT_MSGS__SRV__DETAIL__STORE_LOGPOINT__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__StoreLogpoint_Request __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__StoreLogpoint_Request __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct StoreLogpoint_Request_
{
  using Type = StoreLogpoint_Request_<ContainerAllocator>;

  explicit StoreLogpoint_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->name = "";
      this->tag = "";
    }
  }

  explicit StoreLogpoint_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : name(_alloc),
    tag(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->name = "";
      this->tag = "";
    }
  }

  // field types and members
  using _name_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _name_type name;
  using _tag_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _tag_type tag;

  // setters for named parameter idiom
  Type & set__name(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->name = _arg;
    return *this;
  }
  Type & set__tag(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->tag = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__StoreLogpoint_Request
    std::shared_ptr<spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__StoreLogpoint_Request
    std::shared_ptr<spot_msgs::srv::StoreLogpoint_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const StoreLogpoint_Request_ & other) const
  {
    if (this->name != other.name) {
      return false;
    }
    if (this->tag != other.tag) {
      return false;
    }
    return true;
  }
  bool operator!=(const StoreLogpoint_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct StoreLogpoint_Request_

// alias to use template instance with default allocator
using StoreLogpoint_Request =
  spot_msgs::srv::StoreLogpoint_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs


// Include directives for member types
// Member 'logpoint'
#include "bosdyn_spot_cam_api_msgs/msg/detail/logpoint__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__StoreLogpoint_Response __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__StoreLogpoint_Response __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct StoreLogpoint_Response_
{
  using Type = StoreLogpoint_Response_<ContainerAllocator>;

  explicit StoreLogpoint_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : logpoint(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  explicit StoreLogpoint_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : message(_alloc),
    logpoint(_alloc, _init)
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
  using _logpoint_type =
    bosdyn_spot_cam_api_msgs::msg::Logpoint_<ContainerAllocator>;
  _logpoint_type logpoint;

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
  Type & set__logpoint(
    const bosdyn_spot_cam_api_msgs::msg::Logpoint_<ContainerAllocator> & _arg)
  {
    this->logpoint = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__StoreLogpoint_Response
    std::shared_ptr<spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__StoreLogpoint_Response
    std::shared_ptr<spot_msgs::srv::StoreLogpoint_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const StoreLogpoint_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    if (this->logpoint != other.logpoint) {
      return false;
    }
    return true;
  }
  bool operator!=(const StoreLogpoint_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct StoreLogpoint_Response_

// alias to use template instance with default allocator
using StoreLogpoint_Response =
  spot_msgs::srv::StoreLogpoint_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs

namespace spot_msgs
{

namespace srv
{

struct StoreLogpoint
{
  using Request = spot_msgs::srv::StoreLogpoint_Request;
  using Response = spot_msgs::srv::StoreLogpoint_Response;
};

}  // namespace srv

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__STORE_LOGPOINT__STRUCT_HPP_
