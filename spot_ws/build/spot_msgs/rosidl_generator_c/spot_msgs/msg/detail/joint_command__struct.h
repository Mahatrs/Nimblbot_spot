// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from spot_msgs:msg/JointCommand.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__MSG__DETAIL__JOINT_COMMAND__STRUCT_H_
#define SPOT_MSGS__MSG__DETAIL__JOINT_COMMAND__STRUCT_H_

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
// Member 'position'
// Member 'velocity'
// Member 'effort'
// Member 'k_q_p'
// Member 'k_qd_p'
#include "rosidl_runtime_c/primitives_sequence.h"

/// Struct defined in msg/JointCommand in the package spot_msgs.
/**
  * list of the joint names to control
 */
typedef struct spot_msgs__msg__JointCommand
{
  rosidl_runtime_c__String__Sequence name;
  /// desired position commands for each joint in rad
  rosidl_runtime_c__double__Sequence position;
  /// desired velocity commands for each joint in rad/s
  rosidl_runtime_c__double__Sequence velocity;
  /// desired effort commands for each joint in Nm
  rosidl_runtime_c__double__Sequence effort;
  /// desired k_q_p commands for each joint in Nm/rad
  rosidl_runtime_c__double__Sequence k_q_p;
  /// desired k_qd_p command for each joint in Nms/rad
  rosidl_runtime_c__double__Sequence k_qd_p;
} spot_msgs__msg__JointCommand;

// Struct for a sequence of spot_msgs__msg__JointCommand.
typedef struct spot_msgs__msg__JointCommand__Sequence
{
  spot_msgs__msg__JointCommand * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} spot_msgs__msg__JointCommand__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SPOT_MSGS__MSG__DETAIL__JOINT_COMMAND__STRUCT_H_
