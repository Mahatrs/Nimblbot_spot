// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from rviz_manager_msgs:msg/ManagerStatus.idl
// generated code does not contain a copyright notice
#include "rviz_manager_msgs/msg/detail/manager_status__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `status`
// Member `message`
#include "rosidl_runtime_c/string_functions.h"

bool
rviz_manager_msgs__msg__ManagerStatus__init(rviz_manager_msgs__msg__ManagerStatus * msg)
{
  if (!msg) {
    return false;
  }
  // id
  // status
  if (!rosidl_runtime_c__String__init(&msg->status)) {
    rviz_manager_msgs__msg__ManagerStatus__fini(msg);
    return false;
  }
  // message
  if (!rosidl_runtime_c__String__init(&msg->message)) {
    rviz_manager_msgs__msg__ManagerStatus__fini(msg);
    return false;
  }
  return true;
}

void
rviz_manager_msgs__msg__ManagerStatus__fini(rviz_manager_msgs__msg__ManagerStatus * msg)
{
  if (!msg) {
    return;
  }
  // id
  // status
  rosidl_runtime_c__String__fini(&msg->status);
  // message
  rosidl_runtime_c__String__fini(&msg->message);
}

bool
rviz_manager_msgs__msg__ManagerStatus__are_equal(const rviz_manager_msgs__msg__ManagerStatus * lhs, const rviz_manager_msgs__msg__ManagerStatus * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // id
  if (lhs->id != rhs->id) {
    return false;
  }
  // status
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->status), &(rhs->status)))
  {
    return false;
  }
  // message
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->message), &(rhs->message)))
  {
    return false;
  }
  return true;
}

bool
rviz_manager_msgs__msg__ManagerStatus__copy(
  const rviz_manager_msgs__msg__ManagerStatus * input,
  rviz_manager_msgs__msg__ManagerStatus * output)
{
  if (!input || !output) {
    return false;
  }
  // id
  output->id = input->id;
  // status
  if (!rosidl_runtime_c__String__copy(
      &(input->status), &(output->status)))
  {
    return false;
  }
  // message
  if (!rosidl_runtime_c__String__copy(
      &(input->message), &(output->message)))
  {
    return false;
  }
  return true;
}

rviz_manager_msgs__msg__ManagerStatus *
rviz_manager_msgs__msg__ManagerStatus__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  rviz_manager_msgs__msg__ManagerStatus * msg = (rviz_manager_msgs__msg__ManagerStatus *)allocator.allocate(sizeof(rviz_manager_msgs__msg__ManagerStatus), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(rviz_manager_msgs__msg__ManagerStatus));
  bool success = rviz_manager_msgs__msg__ManagerStatus__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
rviz_manager_msgs__msg__ManagerStatus__destroy(rviz_manager_msgs__msg__ManagerStatus * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    rviz_manager_msgs__msg__ManagerStatus__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
rviz_manager_msgs__msg__ManagerStatus__Sequence__init(rviz_manager_msgs__msg__ManagerStatus__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  rviz_manager_msgs__msg__ManagerStatus * data = NULL;

  if (size) {
    data = (rviz_manager_msgs__msg__ManagerStatus *)allocator.zero_allocate(size, sizeof(rviz_manager_msgs__msg__ManagerStatus), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = rviz_manager_msgs__msg__ManagerStatus__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        rviz_manager_msgs__msg__ManagerStatus__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
rviz_manager_msgs__msg__ManagerStatus__Sequence__fini(rviz_manager_msgs__msg__ManagerStatus__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      rviz_manager_msgs__msg__ManagerStatus__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

rviz_manager_msgs__msg__ManagerStatus__Sequence *
rviz_manager_msgs__msg__ManagerStatus__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  rviz_manager_msgs__msg__ManagerStatus__Sequence * array = (rviz_manager_msgs__msg__ManagerStatus__Sequence *)allocator.allocate(sizeof(rviz_manager_msgs__msg__ManagerStatus__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = rviz_manager_msgs__msg__ManagerStatus__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
rviz_manager_msgs__msg__ManagerStatus__Sequence__destroy(rviz_manager_msgs__msg__ManagerStatus__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    rviz_manager_msgs__msg__ManagerStatus__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
rviz_manager_msgs__msg__ManagerStatus__Sequence__are_equal(const rviz_manager_msgs__msg__ManagerStatus__Sequence * lhs, const rviz_manager_msgs__msg__ManagerStatus__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!rviz_manager_msgs__msg__ManagerStatus__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
rviz_manager_msgs__msg__ManagerStatus__Sequence__copy(
  const rviz_manager_msgs__msg__ManagerStatus__Sequence * input,
  rviz_manager_msgs__msg__ManagerStatus__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(rviz_manager_msgs__msg__ManagerStatus);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    rviz_manager_msgs__msg__ManagerStatus * data =
      (rviz_manager_msgs__msg__ManagerStatus *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!rviz_manager_msgs__msg__ManagerStatus__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          rviz_manager_msgs__msg__ManagerStatus__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!rviz_manager_msgs__msg__ManagerStatus__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
