// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from rviz_manager_msgs:msg/ManagerLaunch.idl
// generated code does not contain a copyright notice

#ifndef RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__STRUCT_HPP_
#define RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__STRUCT_HPP_

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

#ifndef _WIN32
# define DEPRECATED__rviz_manager_msgs__msg__ManagerLaunch __attribute__((deprecated))
#else
# define DEPRECATED__rviz_manager_msgs__msg__ManagerLaunch __declspec(deprecated)
#endif

namespace rviz_manager_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct ManagerLaunch_
{
  using Type = ManagerLaunch_<ContainerAllocator>;

  explicit ManagerLaunch_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : header(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->id = 0l;
      this->action = "";
      this->ns = "";
      this->bash_session = false;
      this->is_launch_file = false;
      this->package = "";
      this->executable = "";
      this->working_dir = "";
      this->session_name = "";
      this->use_sim_time = false;
      this->timeout = 0l;
    }
  }

  explicit ManagerLaunch_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : header(_alloc, _init),
    action(_alloc),
    ns(_alloc),
    package(_alloc),
    executable(_alloc),
    working_dir(_alloc),
    session_name(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->id = 0l;
      this->action = "";
      this->ns = "";
      this->bash_session = false;
      this->is_launch_file = false;
      this->package = "";
      this->executable = "";
      this->working_dir = "";
      this->session_name = "";
      this->use_sim_time = false;
      this->timeout = 0l;
    }
  }

  // field types and members
  using _header_type =
    std_msgs::msg::Header_<ContainerAllocator>;
  _header_type header;
  using _id_type =
    int32_t;
  _id_type id;
  using _action_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _action_type action;
  using _ns_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _ns_type ns;
  using _bash_session_type =
    bool;
  _bash_session_type bash_session;
  using _is_launch_file_type =
    bool;
  _is_launch_file_type is_launch_file;
  using _package_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _package_type package;
  using _executable_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _executable_type executable;
  using _arguments_type =
    std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>>;
  _arguments_type arguments;
  using _ros_arguments_type =
    std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>>;
  _ros_arguments_type ros_arguments;
  using _working_dir_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _working_dir_type working_dir;
  using _session_name_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _session_name_type session_name;
  using _use_sim_time_type =
    bool;
  _use_sim_time_type use_sim_time;
  using _timeout_type =
    int32_t;
  _timeout_type timeout;

  // setters for named parameter idiom
  Type & set__header(
    const std_msgs::msg::Header_<ContainerAllocator> & _arg)
  {
    this->header = _arg;
    return *this;
  }
  Type & set__id(
    const int32_t & _arg)
  {
    this->id = _arg;
    return *this;
  }
  Type & set__action(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->action = _arg;
    return *this;
  }
  Type & set__ns(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->ns = _arg;
    return *this;
  }
  Type & set__bash_session(
    const bool & _arg)
  {
    this->bash_session = _arg;
    return *this;
  }
  Type & set__is_launch_file(
    const bool & _arg)
  {
    this->is_launch_file = _arg;
    return *this;
  }
  Type & set__package(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->package = _arg;
    return *this;
  }
  Type & set__executable(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->executable = _arg;
    return *this;
  }
  Type & set__arguments(
    const std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>> & _arg)
  {
    this->arguments = _arg;
    return *this;
  }
  Type & set__ros_arguments(
    const std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>> & _arg)
  {
    this->ros_arguments = _arg;
    return *this;
  }
  Type & set__working_dir(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->working_dir = _arg;
    return *this;
  }
  Type & set__session_name(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->session_name = _arg;
    return *this;
  }
  Type & set__use_sim_time(
    const bool & _arg)
  {
    this->use_sim_time = _arg;
    return *this;
  }
  Type & set__timeout(
    const int32_t & _arg)
  {
    this->timeout = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator> *;
  using ConstRawPtr =
    const rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__rviz_manager_msgs__msg__ManagerLaunch
    std::shared_ptr<rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__rviz_manager_msgs__msg__ManagerLaunch
    std::shared_ptr<rviz_manager_msgs::msg::ManagerLaunch_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ManagerLaunch_ & other) const
  {
    if (this->header != other.header) {
      return false;
    }
    if (this->id != other.id) {
      return false;
    }
    if (this->action != other.action) {
      return false;
    }
    if (this->ns != other.ns) {
      return false;
    }
    if (this->bash_session != other.bash_session) {
      return false;
    }
    if (this->is_launch_file != other.is_launch_file) {
      return false;
    }
    if (this->package != other.package) {
      return false;
    }
    if (this->executable != other.executable) {
      return false;
    }
    if (this->arguments != other.arguments) {
      return false;
    }
    if (this->ros_arguments != other.ros_arguments) {
      return false;
    }
    if (this->working_dir != other.working_dir) {
      return false;
    }
    if (this->session_name != other.session_name) {
      return false;
    }
    if (this->use_sim_time != other.use_sim_time) {
      return false;
    }
    if (this->timeout != other.timeout) {
      return false;
    }
    return true;
  }
  bool operator!=(const ManagerLaunch_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ManagerLaunch_

// alias to use template instance with default allocator
using ManagerLaunch =
  rviz_manager_msgs::msg::ManagerLaunch_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace rviz_manager_msgs

#endif  // RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__STRUCT_HPP_
