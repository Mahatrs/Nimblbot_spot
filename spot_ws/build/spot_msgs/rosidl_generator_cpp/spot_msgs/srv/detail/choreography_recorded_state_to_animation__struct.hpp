// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from spot_msgs:srv/ChoreographyRecordedStateToAnimation.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_RECORDED_STATE_TO_ANIMATION__STRUCT_HPP_
#define SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_RECORDED_STATE_TO_ANIMATION__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct ChoreographyRecordedStateToAnimation_Request_
{
  using Type = ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator>;

  explicit ChoreographyRecordedStateToAnimation_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->has_arm = false;
    }
  }

  explicit ChoreographyRecordedStateToAnimation_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->has_arm = false;
    }
  }

  // field types and members
  using _has_arm_type =
    bool;
  _has_arm_type has_arm;

  // setters for named parameter idiom
  Type & set__has_arm(
    const bool & _arg)
  {
    this->has_arm = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request
    std::shared_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request
    std::shared_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ChoreographyRecordedStateToAnimation_Request_ & other) const
  {
    if (this->has_arm != other.has_arm) {
      return false;
    }
    return true;
  }
  bool operator!=(const ChoreographyRecordedStateToAnimation_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ChoreographyRecordedStateToAnimation_Request_

// alias to use template instance with default allocator
using ChoreographyRecordedStateToAnimation_Request =
  spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs


#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct ChoreographyRecordedStateToAnimation_Response_
{
  using Type = ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator>;

  explicit ChoreographyRecordedStateToAnimation_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
      this->animation_file_contents = "";
    }
  }

  explicit ChoreographyRecordedStateToAnimation_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : message(_alloc),
    animation_file_contents(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
      this->animation_file_contents = "";
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _message_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _message_type message;
  using _animation_file_contents_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _animation_file_contents_type animation_file_contents;

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
  Type & set__animation_file_contents(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->animation_file_contents = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response
    std::shared_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response
    std::shared_ptr<spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ChoreographyRecordedStateToAnimation_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    if (this->animation_file_contents != other.animation_file_contents) {
      return false;
    }
    return true;
  }
  bool operator!=(const ChoreographyRecordedStateToAnimation_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ChoreographyRecordedStateToAnimation_Response_

// alias to use template instance with default allocator
using ChoreographyRecordedStateToAnimation_Response =
  spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs

namespace spot_msgs
{

namespace srv
{

struct ChoreographyRecordedStateToAnimation
{
  using Request = spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request;
  using Response = spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response;
};

}  // namespace srv

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_RECORDED_STATE_TO_ANIMATION__STRUCT_HPP_
