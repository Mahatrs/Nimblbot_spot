// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from spot_msgs:srv/ChoreographyStartRecordingState.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__STRUCT_HPP_
#define SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__ChoreographyStartRecordingState_Request __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__ChoreographyStartRecordingState_Request __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct ChoreographyStartRecordingState_Request_
{
  using Type = ChoreographyStartRecordingState_Request_<ContainerAllocator>;

  explicit ChoreographyStartRecordingState_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->duration_seconds = 0.0f;
    }
  }

  explicit ChoreographyStartRecordingState_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->duration_seconds = 0.0f;
    }
  }

  // field types and members
  using _duration_seconds_type =
    float;
  _duration_seconds_type duration_seconds;

  // setters for named parameter idiom
  Type & set__duration_seconds(
    const float & _arg)
  {
    this->duration_seconds = _arg;
    return *this;
  }

  // constant declarations
  static constexpr uint8_t STATUS_UNKNOWN =
    0u;
  static constexpr uint8_t STATUS_OK =
    1u;
  static constexpr uint8_t STATUS_UNKNOWN_RECORDING_SESSION_ID =
    2u;
  static constexpr uint8_t STATUS_RECORDING_BUFFER_FULL =
    3u;

  // pointer types
  using RawPtr =
    spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__ChoreographyStartRecordingState_Request
    std::shared_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__ChoreographyStartRecordingState_Request
    std::shared_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ChoreographyStartRecordingState_Request_ & other) const
  {
    if (this->duration_seconds != other.duration_seconds) {
      return false;
    }
    return true;
  }
  bool operator!=(const ChoreographyStartRecordingState_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ChoreographyStartRecordingState_Request_

// alias to use template instance with default allocator
using ChoreographyStartRecordingState_Request =
  spot_msgs::srv::ChoreographyStartRecordingState_Request_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t ChoreographyStartRecordingState_Request_<ContainerAllocator>::STATUS_UNKNOWN;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t ChoreographyStartRecordingState_Request_<ContainerAllocator>::STATUS_OK;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t ChoreographyStartRecordingState_Request_<ContainerAllocator>::STATUS_UNKNOWN_RECORDING_SESSION_ID;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t ChoreographyStartRecordingState_Request_<ContainerAllocator>::STATUS_RECORDING_BUFFER_FULL;
#endif  // __cplusplus < 201703L

}  // namespace srv

}  // namespace spot_msgs


#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__ChoreographyStartRecordingState_Response __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__ChoreographyStartRecordingState_Response __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct ChoreographyStartRecordingState_Response_
{
  using Type = ChoreographyStartRecordingState_Response_<ContainerAllocator>;

  explicit ChoreographyStartRecordingState_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
      this->status = 0;
      this->recording_session_id = 0ull;
    }
  }

  explicit ChoreographyStartRecordingState_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : message(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
      this->status = 0;
      this->recording_session_id = 0ull;
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
  using _recording_session_id_type =
    uint64_t;
  _recording_session_id_type recording_session_id;

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
  Type & set__recording_session_id(
    const uint64_t & _arg)
  {
    this->recording_session_id = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__ChoreographyStartRecordingState_Response
    std::shared_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__ChoreographyStartRecordingState_Response
    std::shared_ptr<spot_msgs::srv::ChoreographyStartRecordingState_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ChoreographyStartRecordingState_Response_ & other) const
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
    if (this->recording_session_id != other.recording_session_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const ChoreographyStartRecordingState_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ChoreographyStartRecordingState_Response_

// alias to use template instance with default allocator
using ChoreographyStartRecordingState_Response =
  spot_msgs::srv::ChoreographyStartRecordingState_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs

namespace spot_msgs
{

namespace srv
{

struct ChoreographyStartRecordingState
{
  using Request = spot_msgs::srv::ChoreographyStartRecordingState_Request;
  using Response = spot_msgs::srv::ChoreographyStartRecordingState_Response;
};

}  // namespace srv

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__STRUCT_HPP_
