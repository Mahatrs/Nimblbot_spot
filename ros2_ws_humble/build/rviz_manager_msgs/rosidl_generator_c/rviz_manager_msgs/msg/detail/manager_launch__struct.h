// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from rviz_manager_msgs:msg/ManagerLaunch.idl
// generated code does not contain a copyright notice

#ifndef RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__STRUCT_H_
#define RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__STRUCT_H_

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
// Member 'action'
// Member 'ns'
// Member 'package'
// Member 'executable'
// Member 'arguments'
// Member 'ros_arguments'
// Member 'working_dir'
// Member 'session_name'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/ManagerLaunch in the package rviz_manager_msgs.
typedef struct rviz_manager_msgs__msg__ManagerLaunch
{
  std_msgs__msg__Header header;
  /// unique identifier
  int32_t id;
  /// "start", "stop", "restart", "status"
  rosidl_runtime_c__String action;
  rosidl_runtime_c__String ns;
  /// true = session ros, false = custom cmd
  bool bash_session;
  /// true = launch file, false = node
  bool is_launch_file;
  rosidl_runtime_c__String package;
  /// node or launch file
  rosidl_runtime_c__String executable;
  /// CLI args
  rosidl_runtime_c__String__Sequence arguments;
  /// --ros-args ...
  rosidl_runtime_c__String__Sequence ros_arguments;
  /// optional
  rosidl_runtime_c__String working_dir;
  /// tmux session name (optional override)
  rosidl_runtime_c__String session_name;
  /// optional flag
  bool use_sim_time;
  /// optional (sec)
  int32_t timeout;
} rviz_manager_msgs__msg__ManagerLaunch;

// Struct for a sequence of rviz_manager_msgs__msg__ManagerLaunch.
typedef struct rviz_manager_msgs__msg__ManagerLaunch__Sequence
{
  rviz_manager_msgs__msg__ManagerLaunch * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} rviz_manager_msgs__msg__ManagerLaunch__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__STRUCT_H_
