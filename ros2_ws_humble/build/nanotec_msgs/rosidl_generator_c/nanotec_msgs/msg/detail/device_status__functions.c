// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from nanotec_msgs:msg/DeviceStatus.idl
// generated code does not contain a copyright notice
#include "nanotec_msgs/msg/detail/device_status__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `name`
#include "rosidl_runtime_c/string_functions.h"

bool
nanotec_msgs__msg__DeviceStatus__init(nanotec_msgs__msg__DeviceStatus * msg)
{
  if (!msg) {
    return false;
  }
  // name
  if (!rosidl_runtime_c__String__init(&msg->name)) {
    nanotec_msgs__msg__DeviceStatus__fini(msg);
    return false;
  }
  // voltage_power
  // voltage_logic
  // temperature_motor
  // temperature_micro_chip
  // ready_to_switch_on
  // switched_on
  // operation_enabled
  // fault
  // voltage_enabled
  // quick_stop
  // switch_on_disabled
  // warning
  // target_reached
  // internal_limit_active
  // operation_mode_specific
  // homing_status
  return true;
}

void
nanotec_msgs__msg__DeviceStatus__fini(nanotec_msgs__msg__DeviceStatus * msg)
{
  if (!msg) {
    return;
  }
  // name
  rosidl_runtime_c__String__fini(&msg->name);
  // voltage_power
  // voltage_logic
  // temperature_motor
  // temperature_micro_chip
  // ready_to_switch_on
  // switched_on
  // operation_enabled
  // fault
  // voltage_enabled
  // quick_stop
  // switch_on_disabled
  // warning
  // target_reached
  // internal_limit_active
  // operation_mode_specific
  // homing_status
}

bool
nanotec_msgs__msg__DeviceStatus__are_equal(const nanotec_msgs__msg__DeviceStatus * lhs, const nanotec_msgs__msg__DeviceStatus * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // name
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->name), &(rhs->name)))
  {
    return false;
  }
  // voltage_power
  if (lhs->voltage_power != rhs->voltage_power) {
    return false;
  }
  // voltage_logic
  if (lhs->voltage_logic != rhs->voltage_logic) {
    return false;
  }
  // temperature_motor
  if (lhs->temperature_motor != rhs->temperature_motor) {
    return false;
  }
  // temperature_micro_chip
  if (lhs->temperature_micro_chip != rhs->temperature_micro_chip) {
    return false;
  }
  // ready_to_switch_on
  if (lhs->ready_to_switch_on != rhs->ready_to_switch_on) {
    return false;
  }
  // switched_on
  if (lhs->switched_on != rhs->switched_on) {
    return false;
  }
  // operation_enabled
  if (lhs->operation_enabled != rhs->operation_enabled) {
    return false;
  }
  // fault
  if (lhs->fault != rhs->fault) {
    return false;
  }
  // voltage_enabled
  if (lhs->voltage_enabled != rhs->voltage_enabled) {
    return false;
  }
  // quick_stop
  if (lhs->quick_stop != rhs->quick_stop) {
    return false;
  }
  // switch_on_disabled
  if (lhs->switch_on_disabled != rhs->switch_on_disabled) {
    return false;
  }
  // warning
  if (lhs->warning != rhs->warning) {
    return false;
  }
  // target_reached
  if (lhs->target_reached != rhs->target_reached) {
    return false;
  }
  // internal_limit_active
  if (lhs->internal_limit_active != rhs->internal_limit_active) {
    return false;
  }
  // operation_mode_specific
  if (lhs->operation_mode_specific != rhs->operation_mode_specific) {
    return false;
  }
  // homing_status
  if (lhs->homing_status != rhs->homing_status) {
    return false;
  }
  return true;
}

bool
nanotec_msgs__msg__DeviceStatus__copy(
  const nanotec_msgs__msg__DeviceStatus * input,
  nanotec_msgs__msg__DeviceStatus * output)
{
  if (!input || !output) {
    return false;
  }
  // name
  if (!rosidl_runtime_c__String__copy(
      &(input->name), &(output->name)))
  {
    return false;
  }
  // voltage_power
  output->voltage_power = input->voltage_power;
  // voltage_logic
  output->voltage_logic = input->voltage_logic;
  // temperature_motor
  output->temperature_motor = input->temperature_motor;
  // temperature_micro_chip
  output->temperature_micro_chip = input->temperature_micro_chip;
  // ready_to_switch_on
  output->ready_to_switch_on = input->ready_to_switch_on;
  // switched_on
  output->switched_on = input->switched_on;
  // operation_enabled
  output->operation_enabled = input->operation_enabled;
  // fault
  output->fault = input->fault;
  // voltage_enabled
  output->voltage_enabled = input->voltage_enabled;
  // quick_stop
  output->quick_stop = input->quick_stop;
  // switch_on_disabled
  output->switch_on_disabled = input->switch_on_disabled;
  // warning
  output->warning = input->warning;
  // target_reached
  output->target_reached = input->target_reached;
  // internal_limit_active
  output->internal_limit_active = input->internal_limit_active;
  // operation_mode_specific
  output->operation_mode_specific = input->operation_mode_specific;
  // homing_status
  output->homing_status = input->homing_status;
  return true;
}

nanotec_msgs__msg__DeviceStatus *
nanotec_msgs__msg__DeviceStatus__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  nanotec_msgs__msg__DeviceStatus * msg = (nanotec_msgs__msg__DeviceStatus *)allocator.allocate(sizeof(nanotec_msgs__msg__DeviceStatus), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(nanotec_msgs__msg__DeviceStatus));
  bool success = nanotec_msgs__msg__DeviceStatus__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
nanotec_msgs__msg__DeviceStatus__destroy(nanotec_msgs__msg__DeviceStatus * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    nanotec_msgs__msg__DeviceStatus__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
nanotec_msgs__msg__DeviceStatus__Sequence__init(nanotec_msgs__msg__DeviceStatus__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  nanotec_msgs__msg__DeviceStatus * data = NULL;

  if (size) {
    data = (nanotec_msgs__msg__DeviceStatus *)allocator.zero_allocate(size, sizeof(nanotec_msgs__msg__DeviceStatus), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = nanotec_msgs__msg__DeviceStatus__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        nanotec_msgs__msg__DeviceStatus__fini(&data[i - 1]);
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
nanotec_msgs__msg__DeviceStatus__Sequence__fini(nanotec_msgs__msg__DeviceStatus__Sequence * array)
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
      nanotec_msgs__msg__DeviceStatus__fini(&array->data[i]);
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

nanotec_msgs__msg__DeviceStatus__Sequence *
nanotec_msgs__msg__DeviceStatus__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  nanotec_msgs__msg__DeviceStatus__Sequence * array = (nanotec_msgs__msg__DeviceStatus__Sequence *)allocator.allocate(sizeof(nanotec_msgs__msg__DeviceStatus__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = nanotec_msgs__msg__DeviceStatus__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
nanotec_msgs__msg__DeviceStatus__Sequence__destroy(nanotec_msgs__msg__DeviceStatus__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    nanotec_msgs__msg__DeviceStatus__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
nanotec_msgs__msg__DeviceStatus__Sequence__are_equal(const nanotec_msgs__msg__DeviceStatus__Sequence * lhs, const nanotec_msgs__msg__DeviceStatus__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!nanotec_msgs__msg__DeviceStatus__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
nanotec_msgs__msg__DeviceStatus__Sequence__copy(
  const nanotec_msgs__msg__DeviceStatus__Sequence * input,
  nanotec_msgs__msg__DeviceStatus__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(nanotec_msgs__msg__DeviceStatus);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    nanotec_msgs__msg__DeviceStatus * data =
      (nanotec_msgs__msg__DeviceStatus *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!nanotec_msgs__msg__DeviceStatus__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          nanotec_msgs__msg__DeviceStatus__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!nanotec_msgs__msg__DeviceStatus__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
