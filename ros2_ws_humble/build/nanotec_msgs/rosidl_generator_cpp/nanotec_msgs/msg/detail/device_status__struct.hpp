// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from nanotec_msgs:msg/DeviceStatus.idl
// generated code does not contain a copyright notice

#ifndef NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__STRUCT_HPP_
#define NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__nanotec_msgs__msg__DeviceStatus __attribute__((deprecated))
#else
# define DEPRECATED__nanotec_msgs__msg__DeviceStatus __declspec(deprecated)
#endif

namespace nanotec_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct DeviceStatus_
{
  using Type = DeviceStatus_<ContainerAllocator>;

  explicit DeviceStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->name = "";
      this->voltage_power = 0.0f;
      this->voltage_logic = 0.0f;
      this->temperature_motor = 0.0f;
      this->temperature_micro_chip = 0.0f;
      this->ready_to_switch_on = false;
      this->switched_on = false;
      this->operation_enabled = false;
      this->fault = false;
      this->voltage_enabled = false;
      this->quick_stop = false;
      this->switch_on_disabled = false;
      this->warning = false;
      this->target_reached = false;
      this->internal_limit_active = false;
      this->operation_mode_specific = 0l;
      this->homing_status = 0l;
    }
  }

  explicit DeviceStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : name(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->name = "";
      this->voltage_power = 0.0f;
      this->voltage_logic = 0.0f;
      this->temperature_motor = 0.0f;
      this->temperature_micro_chip = 0.0f;
      this->ready_to_switch_on = false;
      this->switched_on = false;
      this->operation_enabled = false;
      this->fault = false;
      this->voltage_enabled = false;
      this->quick_stop = false;
      this->switch_on_disabled = false;
      this->warning = false;
      this->target_reached = false;
      this->internal_limit_active = false;
      this->operation_mode_specific = 0l;
      this->homing_status = 0l;
    }
  }

  // field types and members
  using _name_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _name_type name;
  using _voltage_power_type =
    float;
  _voltage_power_type voltage_power;
  using _voltage_logic_type =
    float;
  _voltage_logic_type voltage_logic;
  using _temperature_motor_type =
    float;
  _temperature_motor_type temperature_motor;
  using _temperature_micro_chip_type =
    float;
  _temperature_micro_chip_type temperature_micro_chip;
  using _ready_to_switch_on_type =
    bool;
  _ready_to_switch_on_type ready_to_switch_on;
  using _switched_on_type =
    bool;
  _switched_on_type switched_on;
  using _operation_enabled_type =
    bool;
  _operation_enabled_type operation_enabled;
  using _fault_type =
    bool;
  _fault_type fault;
  using _voltage_enabled_type =
    bool;
  _voltage_enabled_type voltage_enabled;
  using _quick_stop_type =
    bool;
  _quick_stop_type quick_stop;
  using _switch_on_disabled_type =
    bool;
  _switch_on_disabled_type switch_on_disabled;
  using _warning_type =
    bool;
  _warning_type warning;
  using _target_reached_type =
    bool;
  _target_reached_type target_reached;
  using _internal_limit_active_type =
    bool;
  _internal_limit_active_type internal_limit_active;
  using _operation_mode_specific_type =
    int32_t;
  _operation_mode_specific_type operation_mode_specific;
  using _homing_status_type =
    int32_t;
  _homing_status_type homing_status;

  // setters for named parameter idiom
  Type & set__name(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->name = _arg;
    return *this;
  }
  Type & set__voltage_power(
    const float & _arg)
  {
    this->voltage_power = _arg;
    return *this;
  }
  Type & set__voltage_logic(
    const float & _arg)
  {
    this->voltage_logic = _arg;
    return *this;
  }
  Type & set__temperature_motor(
    const float & _arg)
  {
    this->temperature_motor = _arg;
    return *this;
  }
  Type & set__temperature_micro_chip(
    const float & _arg)
  {
    this->temperature_micro_chip = _arg;
    return *this;
  }
  Type & set__ready_to_switch_on(
    const bool & _arg)
  {
    this->ready_to_switch_on = _arg;
    return *this;
  }
  Type & set__switched_on(
    const bool & _arg)
  {
    this->switched_on = _arg;
    return *this;
  }
  Type & set__operation_enabled(
    const bool & _arg)
  {
    this->operation_enabled = _arg;
    return *this;
  }
  Type & set__fault(
    const bool & _arg)
  {
    this->fault = _arg;
    return *this;
  }
  Type & set__voltage_enabled(
    const bool & _arg)
  {
    this->voltage_enabled = _arg;
    return *this;
  }
  Type & set__quick_stop(
    const bool & _arg)
  {
    this->quick_stop = _arg;
    return *this;
  }
  Type & set__switch_on_disabled(
    const bool & _arg)
  {
    this->switch_on_disabled = _arg;
    return *this;
  }
  Type & set__warning(
    const bool & _arg)
  {
    this->warning = _arg;
    return *this;
  }
  Type & set__target_reached(
    const bool & _arg)
  {
    this->target_reached = _arg;
    return *this;
  }
  Type & set__internal_limit_active(
    const bool & _arg)
  {
    this->internal_limit_active = _arg;
    return *this;
  }
  Type & set__operation_mode_specific(
    const int32_t & _arg)
  {
    this->operation_mode_specific = _arg;
    return *this;
  }
  Type & set__homing_status(
    const int32_t & _arg)
  {
    this->homing_status = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    nanotec_msgs::msg::DeviceStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const nanotec_msgs::msg::DeviceStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      nanotec_msgs::msg::DeviceStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      nanotec_msgs::msg::DeviceStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__nanotec_msgs__msg__DeviceStatus
    std::shared_ptr<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__nanotec_msgs__msg__DeviceStatus
    std::shared_ptr<nanotec_msgs::msg::DeviceStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const DeviceStatus_ & other) const
  {
    if (this->name != other.name) {
      return false;
    }
    if (this->voltage_power != other.voltage_power) {
      return false;
    }
    if (this->voltage_logic != other.voltage_logic) {
      return false;
    }
    if (this->temperature_motor != other.temperature_motor) {
      return false;
    }
    if (this->temperature_micro_chip != other.temperature_micro_chip) {
      return false;
    }
    if (this->ready_to_switch_on != other.ready_to_switch_on) {
      return false;
    }
    if (this->switched_on != other.switched_on) {
      return false;
    }
    if (this->operation_enabled != other.operation_enabled) {
      return false;
    }
    if (this->fault != other.fault) {
      return false;
    }
    if (this->voltage_enabled != other.voltage_enabled) {
      return false;
    }
    if (this->quick_stop != other.quick_stop) {
      return false;
    }
    if (this->switch_on_disabled != other.switch_on_disabled) {
      return false;
    }
    if (this->warning != other.warning) {
      return false;
    }
    if (this->target_reached != other.target_reached) {
      return false;
    }
    if (this->internal_limit_active != other.internal_limit_active) {
      return false;
    }
    if (this->operation_mode_specific != other.operation_mode_specific) {
      return false;
    }
    if (this->homing_status != other.homing_status) {
      return false;
    }
    return true;
  }
  bool operator!=(const DeviceStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct DeviceStatus_

// alias to use template instance with default allocator
using DeviceStatus =
  nanotec_msgs::msg::DeviceStatus_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace nanotec_msgs

#endif  // NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__STRUCT_HPP_
