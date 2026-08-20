// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from nanotec_msgs:msg/DeviceStatus.idl
// generated code does not contain a copyright notice

#ifndef NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__BUILDER_HPP_
#define NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "nanotec_msgs/msg/detail/device_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace nanotec_msgs
{

namespace msg
{

namespace builder
{

class Init_DeviceStatus_homing_status
{
public:
  explicit Init_DeviceStatus_homing_status(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  ::nanotec_msgs::msg::DeviceStatus homing_status(::nanotec_msgs::msg::DeviceStatus::_homing_status_type arg)
  {
    msg_.homing_status = std::move(arg);
    return std::move(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_operation_mode_specific
{
public:
  explicit Init_DeviceStatus_operation_mode_specific(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_homing_status operation_mode_specific(::nanotec_msgs::msg::DeviceStatus::_operation_mode_specific_type arg)
  {
    msg_.operation_mode_specific = std::move(arg);
    return Init_DeviceStatus_homing_status(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_internal_limit_active
{
public:
  explicit Init_DeviceStatus_internal_limit_active(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_operation_mode_specific internal_limit_active(::nanotec_msgs::msg::DeviceStatus::_internal_limit_active_type arg)
  {
    msg_.internal_limit_active = std::move(arg);
    return Init_DeviceStatus_operation_mode_specific(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_target_reached
{
public:
  explicit Init_DeviceStatus_target_reached(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_internal_limit_active target_reached(::nanotec_msgs::msg::DeviceStatus::_target_reached_type arg)
  {
    msg_.target_reached = std::move(arg);
    return Init_DeviceStatus_internal_limit_active(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_warning
{
public:
  explicit Init_DeviceStatus_warning(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_target_reached warning(::nanotec_msgs::msg::DeviceStatus::_warning_type arg)
  {
    msg_.warning = std::move(arg);
    return Init_DeviceStatus_target_reached(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_switch_on_disabled
{
public:
  explicit Init_DeviceStatus_switch_on_disabled(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_warning switch_on_disabled(::nanotec_msgs::msg::DeviceStatus::_switch_on_disabled_type arg)
  {
    msg_.switch_on_disabled = std::move(arg);
    return Init_DeviceStatus_warning(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_quick_stop
{
public:
  explicit Init_DeviceStatus_quick_stop(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_switch_on_disabled quick_stop(::nanotec_msgs::msg::DeviceStatus::_quick_stop_type arg)
  {
    msg_.quick_stop = std::move(arg);
    return Init_DeviceStatus_switch_on_disabled(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_voltage_enabled
{
public:
  explicit Init_DeviceStatus_voltage_enabled(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_quick_stop voltage_enabled(::nanotec_msgs::msg::DeviceStatus::_voltage_enabled_type arg)
  {
    msg_.voltage_enabled = std::move(arg);
    return Init_DeviceStatus_quick_stop(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_fault
{
public:
  explicit Init_DeviceStatus_fault(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_voltage_enabled fault(::nanotec_msgs::msg::DeviceStatus::_fault_type arg)
  {
    msg_.fault = std::move(arg);
    return Init_DeviceStatus_voltage_enabled(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_operation_enabled
{
public:
  explicit Init_DeviceStatus_operation_enabled(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_fault operation_enabled(::nanotec_msgs::msg::DeviceStatus::_operation_enabled_type arg)
  {
    msg_.operation_enabled = std::move(arg);
    return Init_DeviceStatus_fault(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_switched_on
{
public:
  explicit Init_DeviceStatus_switched_on(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_operation_enabled switched_on(::nanotec_msgs::msg::DeviceStatus::_switched_on_type arg)
  {
    msg_.switched_on = std::move(arg);
    return Init_DeviceStatus_operation_enabled(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_ready_to_switch_on
{
public:
  explicit Init_DeviceStatus_ready_to_switch_on(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_switched_on ready_to_switch_on(::nanotec_msgs::msg::DeviceStatus::_ready_to_switch_on_type arg)
  {
    msg_.ready_to_switch_on = std::move(arg);
    return Init_DeviceStatus_switched_on(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_temperature_micro_chip
{
public:
  explicit Init_DeviceStatus_temperature_micro_chip(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_ready_to_switch_on temperature_micro_chip(::nanotec_msgs::msg::DeviceStatus::_temperature_micro_chip_type arg)
  {
    msg_.temperature_micro_chip = std::move(arg);
    return Init_DeviceStatus_ready_to_switch_on(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_temperature_motor
{
public:
  explicit Init_DeviceStatus_temperature_motor(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_temperature_micro_chip temperature_motor(::nanotec_msgs::msg::DeviceStatus::_temperature_motor_type arg)
  {
    msg_.temperature_motor = std::move(arg);
    return Init_DeviceStatus_temperature_micro_chip(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_voltage_logic
{
public:
  explicit Init_DeviceStatus_voltage_logic(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_temperature_motor voltage_logic(::nanotec_msgs::msg::DeviceStatus::_voltage_logic_type arg)
  {
    msg_.voltage_logic = std::move(arg);
    return Init_DeviceStatus_temperature_motor(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_voltage_power
{
public:
  explicit Init_DeviceStatus_voltage_power(::nanotec_msgs::msg::DeviceStatus & msg)
  : msg_(msg)
  {}
  Init_DeviceStatus_voltage_logic voltage_power(::nanotec_msgs::msg::DeviceStatus::_voltage_power_type arg)
  {
    msg_.voltage_power = std::move(arg);
    return Init_DeviceStatus_voltage_logic(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

class Init_DeviceStatus_name
{
public:
  Init_DeviceStatus_name()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DeviceStatus_voltage_power name(::nanotec_msgs::msg::DeviceStatus::_name_type arg)
  {
    msg_.name = std::move(arg);
    return Init_DeviceStatus_voltage_power(msg_);
  }

private:
  ::nanotec_msgs::msg::DeviceStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::nanotec_msgs::msg::DeviceStatus>()
{
  return nanotec_msgs::msg::builder::Init_DeviceStatus_name();
}

}  // namespace nanotec_msgs

#endif  // NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__BUILDER_HPP_
