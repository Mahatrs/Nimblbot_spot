// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from spot_msgs:srv/AcquireLease.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__STRUCT_HPP_
#define SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__AcquireLease_Request __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__AcquireLease_Request __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct AcquireLease_Request_
{
  using Type = AcquireLease_Request_<ContainerAllocator>;

  explicit AcquireLease_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->client_name = "";
      this->resource_name = "";
      this->force = false;
    }
  }

  explicit AcquireLease_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : client_name(_alloc),
    resource_name(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->client_name = "";
      this->resource_name = "";
      this->force = false;
    }
  }

  // field types and members
  using _client_name_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _client_name_type client_name;
  using _resource_name_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _resource_name_type resource_name;
  using _force_type =
    bool;
  _force_type force;

  // setters for named parameter idiom
  Type & set__client_name(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->client_name = _arg;
    return *this;
  }
  Type & set__resource_name(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->resource_name = _arg;
    return *this;
  }
  Type & set__force(
    const bool & _arg)
  {
    this->force = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::AcquireLease_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::AcquireLease_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::AcquireLease_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::AcquireLease_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::AcquireLease_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::AcquireLease_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::AcquireLease_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::AcquireLease_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::AcquireLease_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::AcquireLease_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__AcquireLease_Request
    std::shared_ptr<spot_msgs::srv::AcquireLease_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__AcquireLease_Request
    std::shared_ptr<spot_msgs::srv::AcquireLease_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const AcquireLease_Request_ & other) const
  {
    if (this->client_name != other.client_name) {
      return false;
    }
    if (this->resource_name != other.resource_name) {
      return false;
    }
    if (this->force != other.force) {
      return false;
    }
    return true;
  }
  bool operator!=(const AcquireLease_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct AcquireLease_Request_

// alias to use template instance with default allocator
using AcquireLease_Request =
  spot_msgs::srv::AcquireLease_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs


// Include directives for member types
// Member 'lease'
#include "bosdyn_api_msgs/msg/detail/lease__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__AcquireLease_Response __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__AcquireLease_Response __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct AcquireLease_Response_
{
  using Type = AcquireLease_Response_<ContainerAllocator>;

  explicit AcquireLease_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : lease(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  explicit AcquireLease_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : message(_alloc),
    lease(_alloc, _init)
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
  using _lease_type =
    bosdyn_api_msgs::msg::Lease_<ContainerAllocator>;
  _lease_type lease;

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
  Type & set__lease(
    const bosdyn_api_msgs::msg::Lease_<ContainerAllocator> & _arg)
  {
    this->lease = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::AcquireLease_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::AcquireLease_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::AcquireLease_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::AcquireLease_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::AcquireLease_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::AcquireLease_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::AcquireLease_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::AcquireLease_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::AcquireLease_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::AcquireLease_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__AcquireLease_Response
    std::shared_ptr<spot_msgs::srv::AcquireLease_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__AcquireLease_Response
    std::shared_ptr<spot_msgs::srv::AcquireLease_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const AcquireLease_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    if (this->lease != other.lease) {
      return false;
    }
    return true;
  }
  bool operator!=(const AcquireLease_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct AcquireLease_Response_

// alias to use template instance with default allocator
using AcquireLease_Response =
  spot_msgs::srv::AcquireLease_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs

namespace spot_msgs
{

namespace srv
{

struct AcquireLease
{
  using Request = spot_msgs::srv::AcquireLease_Request;
  using Response = spot_msgs::srv::AcquireLease_Response;
};

}  // namespace srv

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__ACQUIRE_LEASE__STRUCT_HPP_
