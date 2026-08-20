// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from spot_msgs:srv/MutateWorldObject.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__MUTATE_WORLD_OBJECT__STRUCT_HPP_
#define SPOT_MSGS__SRV__DETAIL__MUTATE_WORLD_OBJECT__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'request'
#include "bosdyn_api_msgs/msg/detail/mutate_world_object_request__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__MutateWorldObject_Request __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__MutateWorldObject_Request __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct MutateWorldObject_Request_
{
  using Type = MutateWorldObject_Request_<ContainerAllocator>;

  explicit MutateWorldObject_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : request(_init)
  {
    (void)_init;
  }

  explicit MutateWorldObject_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : request(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _request_type =
    bosdyn_api_msgs::msg::MutateWorldObjectRequest_<ContainerAllocator>;
  _request_type request;

  // setters for named parameter idiom
  Type & set__request(
    const bosdyn_api_msgs::msg::MutateWorldObjectRequest_<ContainerAllocator> & _arg)
  {
    this->request = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__MutateWorldObject_Request
    std::shared_ptr<spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__MutateWorldObject_Request
    std::shared_ptr<spot_msgs::srv::MutateWorldObject_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MutateWorldObject_Request_ & other) const
  {
    if (this->request != other.request) {
      return false;
    }
    return true;
  }
  bool operator!=(const MutateWorldObject_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MutateWorldObject_Request_

// alias to use template instance with default allocator
using MutateWorldObject_Request =
  spot_msgs::srv::MutateWorldObject_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs


// Include directives for member types
// Member 'response'
#include "bosdyn_api_msgs/msg/detail/mutate_world_object_response__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__spot_msgs__srv__MutateWorldObject_Response __attribute__((deprecated))
#else
# define DEPRECATED__spot_msgs__srv__MutateWorldObject_Response __declspec(deprecated)
#endif

namespace spot_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct MutateWorldObject_Response_
{
  using Type = MutateWorldObject_Response_<ContainerAllocator>;

  explicit MutateWorldObject_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : response(_init)
  {
    (void)_init;
  }

  explicit MutateWorldObject_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : response(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _response_type =
    bosdyn_api_msgs::msg::MutateWorldObjectResponse_<ContainerAllocator>;
  _response_type response;

  // setters for named parameter idiom
  Type & set__response(
    const bosdyn_api_msgs::msg::MutateWorldObjectResponse_<ContainerAllocator> & _arg)
  {
    this->response = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__spot_msgs__srv__MutateWorldObject_Response
    std::shared_ptr<spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__spot_msgs__srv__MutateWorldObject_Response
    std::shared_ptr<spot_msgs::srv::MutateWorldObject_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MutateWorldObject_Response_ & other) const
  {
    if (this->response != other.response) {
      return false;
    }
    return true;
  }
  bool operator!=(const MutateWorldObject_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MutateWorldObject_Response_

// alias to use template instance with default allocator
using MutateWorldObject_Response =
  spot_msgs::srv::MutateWorldObject_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace spot_msgs

namespace spot_msgs
{

namespace srv
{

struct MutateWorldObject
{
  using Request = spot_msgs::srv::MutateWorldObject_Request;
  using Response = spot_msgs::srv::MutateWorldObject_Response;
};

}  // namespace srv

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__MUTATE_WORLD_OBJECT__STRUCT_HPP_
