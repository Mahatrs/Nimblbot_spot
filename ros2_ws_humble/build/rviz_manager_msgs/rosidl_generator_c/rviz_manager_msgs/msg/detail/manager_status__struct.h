// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from rviz_manager_msgs:msg/ManagerStatus.idl
// generated code does not contain a copyright notice

#ifndef RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__STRUCT_H_
#define RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'status'
// Member 'message'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/ManagerStatus in the package rviz_manager_msgs.
typedef struct rviz_manager_msgs__msg__ManagerStatus
{
  int32_t id;
  /// "running", "stopped", "error"
  rosidl_runtime_c__String status;
  /// logs or error
  rosidl_runtime_c__String message;
} rviz_manager_msgs__msg__ManagerStatus;

// Struct for a sequence of rviz_manager_msgs__msg__ManagerStatus.
typedef struct rviz_manager_msgs__msg__ManagerStatus__Sequence
{
  rviz_manager_msgs__msg__ManagerStatus * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} rviz_manager_msgs__msg__ManagerStatus__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__STRUCT_H_
