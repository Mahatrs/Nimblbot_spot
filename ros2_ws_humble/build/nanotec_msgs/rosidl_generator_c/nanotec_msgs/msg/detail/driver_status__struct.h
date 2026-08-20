// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from nanotec_msgs:msg/DriverStatus.idl
// generated code does not contain a copyright notice

#ifndef NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__STRUCT_H_
#define NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__struct.h"
// Member 'devices'
#include "nanotec_msgs/msg/detail/device_status__struct.h"

/// Struct defined in msg/DriverStatus in the package nanotec_msgs.
typedef struct nanotec_msgs__msg__DriverStatus
{
  std_msgs__msg__Header header;
  nanotec_msgs__msg__DeviceStatus__Sequence devices;
} nanotec_msgs__msg__DriverStatus;

// Struct for a sequence of nanotec_msgs__msg__DriverStatus.
typedef struct nanotec_msgs__msg__DriverStatus__Sequence
{
  nanotec_msgs__msg__DriverStatus * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} nanotec_msgs__msg__DriverStatus__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // NANOTEC_MSGS__MSG__DETAIL__DRIVER_STATUS__STRUCT_H_
