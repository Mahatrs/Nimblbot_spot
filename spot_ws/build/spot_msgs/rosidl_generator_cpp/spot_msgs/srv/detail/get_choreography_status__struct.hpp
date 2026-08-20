// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from spot_msgs:srv/GetChoreographyStatus.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__GET_CHOREOGRAPHY_STATUS__STRUCT_HPP_
#define SPOT_MSGS__SRV__DETAIL__GET_CHOREOGRAPHY_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__GetChoreographyStatus_Request __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__GetChoreographyStatus_Request __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct GetChoreographyStatus_Request_
{
  using Type = GetChoreographyStatus_Request_<ContainerAllocator>;

  explicit GetChoreographyStatus_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  explicit GetChoreographyStatus_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  // field types and members
  using _structure_needs_at_least_one_member_type =
    uint8_t;
  _structure_needs_at_least_one_member_type structure_needs_at_least_one_member;


  // constant declarations
  static constexpr uint8_t STATUS_UNKNOWN =
    0u;
  static constexpr uint8_t STATUS_DANCING =
    1u;
  static constexpr uint8_t STATUS_COMPLETED_SEQUENCE =
    2u;
  static constexpr uint8_t STATUS_PREPPING =
    3u;
  static constexpr uint8_t STATUS_WAITING_FOR_START_TIME =
    4u;
  static constexpr uint8_t STATUS_VALIDATING =
    5u;
  static constexpr uint8_t STATUS_INTERRUPTED =
    6u;
  static constexpr uint8_t STATUS_FALLEN =
    7u;
  static constexpr uint8_t STATUS_POWERED_OFF =
    8u;
  static constexpr uint8_t STATUS_OTHER =
    9u;

  // pointer types
  using RawPtr =
    spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__GetChoreographyStatus_Request
    std::shared_ptr<spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__GetChoreographyStatus_Request
    std::shared_ptr<spot_msgs::srv::GetChoreographyStatus_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const GetChoreographyStatus_Request_ & other) const
  {
    if (this->structure_needs_at_least_one_member != other.structure_needs_at_least_one_member) {
      return false;
    }
    return true;
  }
  bool operator!=(const GetChoreographyStatus_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct GetChoreographyStatus_Request_

// alias to use template instance with default allocator
using GetChoreographyStatus_Request =
  spot_msgs::srv::GetChoreographyStatus_Request_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t GetChoreographyStatus_Request_<ContainerAllocator>::STATUS_UNKNOWN;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t GetChoreographyStatus_Request_<ContainerAllocator>::STATUS_DANCING;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t GetChoreographyStatus_Request_<ContainerAllocator>::STATUS_COMPLETED_SEQUENCE;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t GetChoreographyStatus_Request_<ContainerAllocator>::STATUS_PREPPING;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t GetChoreographyStatus_Request_<ContainerAllocator>::STATUS_WAITING_FOR_START_TIME;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t GetChoreographyStatus_Request_<ContainerAllocator>::STATUS_VALIDATING;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t GetChoreographyStatus_Request_<ContainerAllocator>::STATUS_INTERRUPTED;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t GetChoreographyStatus_Request_<ContainerAllocator>::STATUS_FALLEN;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t GetChoreographyStatus_Request_<ContainerAllocator>::STATUS_POWERED_OFF;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t GetChoreographyStatus_Request_<ContainerAllocator>::STATUS_OTHER;
#endif  // __cplusplus < 201703L

}  // namespace srv

}  // namespace spot_msgs


#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__GetChoreographyStatus_Response __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__GetChoreographyStatus_Response __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct GetChoreographyStatus_Response_
{
  using Type = GetChoreographyStatus_Response_<ContainerAllocator>;

  explicit GetChoreographyStatus_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
      this->status = 0;
      this->execution_id = 0l;
    }
  }

  explicit GetChoreographyStatus_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : message(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
      this->status = 0;
      this->execution_id = 0l;
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _message_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _message_type message;
  using _status_type =
    uint8_t;
  _status_type status;
  using _execution_id_type =
    int32_t;
  _execution_id_type execution_id;

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
  Type & set__status(
    const uint8_t & _arg)
  {
    this->status = _arg;
    return *this;
  }
  Type & set__execution_id(
    const int32_t & _arg)
  {
    this->execution_id = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__GetChoreographyStatus_Response
    std::shared_ptr<spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__GetChoreographyStatus_Response
    std::shared_ptr<spot_msgs::srv::GetChoreographyStatus_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const GetChoreographyStatus_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    if (this->status != other.status) {
      return false;
    }
    if (this->execution_id != other.execution_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const GetChoreographyStatus_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct GetChoreographyStatus_Response_

// alias to use template instance with default allocator
using GetChoreographyStatus_Response =
  spot_msgs::srv::GetChoreographyStatus_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs

namespace spot_msgs
{

namespace srv
{

struct GetChoreographyStatus
{
  using Request = spot_msgs::srv::GetChoreographyStatus_Request;
  using Response = spot_msgs::srv::GetChoreographyStatus_Response;
};

}  // namespace srv

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__GET_CHOREOGRAPHY_STATUS__STRUCT_HPP_
