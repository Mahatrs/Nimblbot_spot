// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from synchros2_tutorials_interfaces_example:msg/String.idl
// generated code does not contain a copyright notice

#ifndef SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__STRUCT_H_
#define SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'data'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/String in the package synchros2_tutorials_interfaces_example.
typedef struct synchros2_tutorials_interfaces_example__msg__String
{
  rosidl_runtime_c__String data;
} synchros2_tutorials_interfaces_example__msg__String;

// Struct for a sequence of synchros2_tutorials_interfaces_example__msg__String.
typedef struct synchros2_tutorials_interfaces_example__msg__String__Sequence
{
  synchros2_tutorials_interfaces_example__msg__String * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} synchros2_tutorials_interfaces_example__msg__String__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__DETAIL__STRING__STRUCT_H_
