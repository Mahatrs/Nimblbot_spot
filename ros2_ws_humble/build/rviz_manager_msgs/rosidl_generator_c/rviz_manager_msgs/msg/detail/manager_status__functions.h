// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from rviz_manager_msgs:msg/ManagerStatus.idl
// generated code does not contain a copyright notice

#ifndef RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__FUNCTIONS_H_
#define RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "rviz_manager_msgs/msg/rosidl_generator_c__visibility_control.h"

#include "rviz_manager_msgs/msg/detail/manager_status__struct.h"

/// Initialize msg/ManagerStatus message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * rviz_manager_msgs__msg__ManagerStatus
 * )) before or use
 * rviz_manager_msgs__msg__ManagerStatus__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
bool
rviz_manager_msgs__msg__ManagerStatus__init(rviz_manager_msgs__msg__ManagerStatus * msg);

/// Finalize msg/ManagerStatus message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
void
rviz_manager_msgs__msg__ManagerStatus__fini(rviz_manager_msgs__msg__ManagerStatus * msg);

/// Create msg/ManagerStatus message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * rviz_manager_msgs__msg__ManagerStatus__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
rviz_manager_msgs__msg__ManagerStatus *
rviz_manager_msgs__msg__ManagerStatus__create();

/// Destroy msg/ManagerStatus message.
/**
 * It calls
 * rviz_manager_msgs__msg__ManagerStatus__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
void
rviz_manager_msgs__msg__ManagerStatus__destroy(rviz_manager_msgs__msg__ManagerStatus * msg);

/// Check for msg/ManagerStatus message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
bool
rviz_manager_msgs__msg__ManagerStatus__are_equal(const rviz_manager_msgs__msg__ManagerStatus * lhs, const rviz_manager_msgs__msg__ManagerStatus * rhs);

/// Copy a msg/ManagerStatus message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
bool
rviz_manager_msgs__msg__ManagerStatus__copy(
  const rviz_manager_msgs__msg__ManagerStatus * input,
  rviz_manager_msgs__msg__ManagerStatus * output);

/// Initialize array of msg/ManagerStatus messages.
/**
 * It allocates the memory for the number of elements and calls
 * rviz_manager_msgs__msg__ManagerStatus__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
bool
rviz_manager_msgs__msg__ManagerStatus__Sequence__init(rviz_manager_msgs__msg__ManagerStatus__Sequence * array, size_t size);

/// Finalize array of msg/ManagerStatus messages.
/**
 * It calls
 * rviz_manager_msgs__msg__ManagerStatus__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
void
rviz_manager_msgs__msg__ManagerStatus__Sequence__fini(rviz_manager_msgs__msg__ManagerStatus__Sequence * array);

/// Create array of msg/ManagerStatus messages.
/**
 * It allocates the memory for the array and calls
 * rviz_manager_msgs__msg__ManagerStatus__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
rviz_manager_msgs__msg__ManagerStatus__Sequence *
rviz_manager_msgs__msg__ManagerStatus__Sequence__create(size_t size);

/// Destroy array of msg/ManagerStatus messages.
/**
 * It calls
 * rviz_manager_msgs__msg__ManagerStatus__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
void
rviz_manager_msgs__msg__ManagerStatus__Sequence__destroy(rviz_manager_msgs__msg__ManagerStatus__Sequence * array);

/// Check for msg/ManagerStatus message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
bool
rviz_manager_msgs__msg__ManagerStatus__Sequence__are_equal(const rviz_manager_msgs__msg__ManagerStatus__Sequence * lhs, const rviz_manager_msgs__msg__ManagerStatus__Sequence * rhs);

/// Copy an array of msg/ManagerStatus messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_rviz_manager_msgs
bool
rviz_manager_msgs__msg__ManagerStatus__Sequence__copy(
  const rviz_manager_msgs__msg__ManagerStatus__Sequence * input,
  rviz_manager_msgs__msg__ManagerStatus__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_STATUS__FUNCTIONS_H_
