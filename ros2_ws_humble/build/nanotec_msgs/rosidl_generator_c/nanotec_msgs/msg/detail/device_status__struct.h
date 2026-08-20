// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from nanotec_msgs:msg/DeviceStatus.idl
// generated code does not contain a copyright notice

#ifndef NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__STRUCT_H_
#define NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'name'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/DeviceStatus in the package nanotec_msgs.
typedef struct nanotec_msgs__msg__DeviceStatus
{
  rosidl_runtime_c__String name;
  float voltage_power;
  float voltage_logic;
  float temperature_motor;
  float temperature_micro_chip;
  bool ready_to_switch_on;
  bool switched_on;
  bool operation_enabled;
  bool fault;
  bool voltage_enabled;
  bool quick_stop;
  bool switch_on_disabled;
  bool warning;
  bool target_reached;
  bool internal_limit_active;
  int32_t operation_mode_specific;
  int32_t homing_status;
} nanotec_msgs__msg__DeviceStatus;

// Struct for a sequence of nanotec_msgs__msg__DeviceStatus.
typedef struct nanotec_msgs__msg__DeviceStatus__Sequence
{
  nanotec_msgs__msg__DeviceStatus * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} nanotec_msgs__msg__DeviceStatus__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // NANOTEC_MSGS__MSG__DETAIL__DEVICE_STATUS__STRUCT_H_
