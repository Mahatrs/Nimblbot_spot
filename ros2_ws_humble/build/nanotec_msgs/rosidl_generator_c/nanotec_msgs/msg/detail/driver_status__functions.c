// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from nanotec_msgs:msg/DriverStatus.idl
// generated code does not contain a copyright notice
#include "nanotec_msgs/msg/detail/driver_status__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `header`
#include "std_msgs/msg/detail/header__functions.h"
// Member `devices`
#include "nanotec_msgs/msg/detail/device_status__functions.h"

bool
nanotec_msgs__msg__DriverStatus__init(nanotec_msgs__msg__DriverStatus * msg)
{
  if (!msg) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__init(&msg->header)) {
    nanotec_msgs__msg__DriverStatus__fini(msg);
    return false;
  }
  // devices
  if (!nanotec_msgs__msg__DeviceStatus__Sequence__init(&msg->devices, 0)) {
    nanotec_msgs__msg__DriverStatus__fini(msg);
    return false;
  }
  return true;
}

void
nanotec_msgs__msg__DriverStatus__fini(nanotec_msgs__msg__DriverStatus * msg)
{
  if (!msg) {
    return;
  }
  // header
  std_msgs__msg__Header__fini(&msg->header);
  // devices
  nanotec_msgs__msg__DeviceStatus__Sequence__fini(&msg->devices);
}

bool
nanotec_msgs__msg__DriverStatus__are_equal(const nanotec_msgs__msg__DriverStatus * lhs, const nanotec_msgs__msg__DriverStatus * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__are_equal(
      &(lhs->header), &(rhs->header)))
  {
    return false;
  }
  // devices
  if (!nanotec_msgs__msg__DeviceStatus__Sequence__are_equal(
      &(lhs->devices), &(rhs->devices)))
  {
    return false;
  }
  return true;
}

bool
nanotec_msgs__msg__DriverStatus__copy(
  const nanotec_msgs__msg__DriverStatus * input,
  nanotec_msgs__msg__DriverStatus * output)
{
  if (!input || !output) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__copy(
      &(input->header), &(output->header)))
  {
    return false;
  }
  // devices
  if (!nanotec_msgs__msg__DeviceStatus__Sequence__copy(
      &(input->devices), &(output->devices)))
  {
    return false;
  }
  return true;
}

nanotec_msgs__msg__DriverStatus *
nanotec_msgs__msg__DriverStatus__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  nanotec_msgs__msg__DriverStatus * msg = (nanotec_msgs__msg__DriverStatus *)allocator.allocate(sizeof(nanotec_msgs__msg__DriverStatus), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(nanotec_msgs__msg__DriverStatus));
  bool success = nanotec_msgs__msg__DriverStatus__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
nanotec_msgs__msg__DriverStatus__destroy(nanotec_msgs__msg__DriverStatus * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    nanotec_msgs__msg__DriverStatus__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
nanotec_msgs__msg__DriverStatus__Sequence__init(nanotec_msgs__msg__DriverStatus__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  nanotec_msgs__msg__DriverStatus * data = NULL;

  if (size) {
    data = (nanotec_msgs__msg__DriverStatus *)allocator.zero_allocate(size, sizeof(nanotec_msgs__msg__DriverStatus), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = nanotec_msgs__msg__DriverStatus__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        nanotec_msgs__msg__DriverStatus__fini(&data[i - 1]);
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
nanotec_msgs__msg__DriverStatus__Sequence__fini(nanotec_msgs__msg__DriverStatus__Sequence * array)
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
      nanotec_msgs__msg__DriverStatus__fini(&array->data[i]);
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

nanotec_msgs__msg__DriverStatus__Sequence *
nanotec_msgs__msg__DriverStatus__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  nanotec_msgs__msg__DriverStatus__Sequence * array = (nanotec_msgs__msg__DriverStatus__Sequence *)allocator.allocate(sizeof(nanotec_msgs__msg__DriverStatus__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = nanotec_msgs__msg__DriverStatus__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
nanotec_msgs__msg__DriverStatus__Sequence__destroy(nanotec_msgs__msg__DriverStatus__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    nanotec_msgs__msg__DriverStatus__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
nanotec_msgs__msg__DriverStatus__Sequence__are_equal(const nanotec_msgs__msg__DriverStatus__Sequence * lhs, const nanotec_msgs__msg__DriverStatus__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!nanotec_msgs__msg__DriverStatus__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
nanotec_msgs__msg__DriverStatus__Sequence__copy(
  const nanotec_msgs__msg__DriverStatus__Sequence * input,
  nanotec_msgs__msg__DriverStatus__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(nanotec_msgs__msg__DriverStatus);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    nanotec_msgs__msg__DriverStatus * data =
      (nanotec_msgs__msg__DriverStatus *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!nanotec_msgs__msg__DriverStatus__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          nanotec_msgs__msg__DriverStatus__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!nanotec_msgs__msg__DriverStatus__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
