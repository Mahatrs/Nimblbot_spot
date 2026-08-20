// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from rviz_manager_msgs:msg/ManagerLaunch.idl
// generated code does not contain a copyright notice
#include "rviz_manager_msgs/msg/detail/manager_launch__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `header`
#include "std_msgs/msg/detail/header__functions.h"
// Member `action`
// Member `ns`
// Member `package`
// Member `executable`
// Member `arguments`
// Member `ros_arguments`
// Member `working_dir`
// Member `session_name`
#include "rosidl_runtime_c/string_functions.h"

bool
rviz_manager_msgs__msg__ManagerLaunch__init(rviz_manager_msgs__msg__ManagerLaunch * msg)
{
  if (!msg) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__init(&msg->header)) {
    rviz_manager_msgs__msg__ManagerLaunch__fini(msg);
    return false;
  }
  // id
  // action
  if (!rosidl_runtime_c__String__init(&msg->action)) {
    rviz_manager_msgs__msg__ManagerLaunch__fini(msg);
    return false;
  }
  // ns
  if (!rosidl_runtime_c__String__init(&msg->ns)) {
    rviz_manager_msgs__msg__ManagerLaunch__fini(msg);
    return false;
  }
  // bash_session
  // is_launch_file
  // package
  if (!rosidl_runtime_c__String__init(&msg->package)) {
    rviz_manager_msgs__msg__ManagerLaunch__fini(msg);
    return false;
  }
  // executable
  if (!rosidl_runtime_c__String__init(&msg->executable)) {
    rviz_manager_msgs__msg__ManagerLaunch__fini(msg);
    return false;
  }
  // arguments
  if (!rosidl_runtime_c__String__Sequence__init(&msg->arguments, 0)) {
    rviz_manager_msgs__msg__ManagerLaunch__fini(msg);
    return false;
  }
  // ros_arguments
  if (!rosidl_runtime_c__String__Sequence__init(&msg->ros_arguments, 0)) {
    rviz_manager_msgs__msg__ManagerLaunch__fini(msg);
    return false;
  }
  // working_dir
  if (!rosidl_runtime_c__String__init(&msg->working_dir)) {
    rviz_manager_msgs__msg__ManagerLaunch__fini(msg);
    return false;
  }
  // session_name
  if (!rosidl_runtime_c__String__init(&msg->session_name)) {
    rviz_manager_msgs__msg__ManagerLaunch__fini(msg);
    return false;
  }
  // use_sim_time
  // timeout
  return true;
}

void
rviz_manager_msgs__msg__ManagerLaunch__fini(rviz_manager_msgs__msg__ManagerLaunch * msg)
{
  if (!msg) {
    return;
  }
  // header
  std_msgs__msg__Header__fini(&msg->header);
  // id
  // action
  rosidl_runtime_c__String__fini(&msg->action);
  // ns
  rosidl_runtime_c__String__fini(&msg->ns);
  // bash_session
  // is_launch_file
  // package
  rosidl_runtime_c__String__fini(&msg->package);
  // executable
  rosidl_runtime_c__String__fini(&msg->executable);
  // arguments
  rosidl_runtime_c__String__Sequence__fini(&msg->arguments);
  // ros_arguments
  rosidl_runtime_c__String__Sequence__fini(&msg->ros_arguments);
  // working_dir
  rosidl_runtime_c__String__fini(&msg->working_dir);
  // session_name
  rosidl_runtime_c__String__fini(&msg->session_name);
  // use_sim_time
  // timeout
}

bool
rviz_manager_msgs__msg__ManagerLaunch__are_equal(const rviz_manager_msgs__msg__ManagerLaunch * lhs, const rviz_manager_msgs__msg__ManagerLaunch * rhs)
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
  // id
  if (lhs->id != rhs->id) {
    return false;
  }
  // action
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->action), &(rhs->action)))
  {
    return false;
  }
  // ns
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->ns), &(rhs->ns)))
  {
    return false;
  }
  // bash_session
  if (lhs->bash_session != rhs->bash_session) {
    return false;
  }
  // is_launch_file
  if (lhs->is_launch_file != rhs->is_launch_file) {
    return false;
  }
  // package
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->package), &(rhs->package)))
  {
    return false;
  }
  // executable
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->executable), &(rhs->executable)))
  {
    return false;
  }
  // arguments
  if (!rosidl_runtime_c__String__Sequence__are_equal(
      &(lhs->arguments), &(rhs->arguments)))
  {
    return false;
  }
  // ros_arguments
  if (!rosidl_runtime_c__String__Sequence__are_equal(
      &(lhs->ros_arguments), &(rhs->ros_arguments)))
  {
    return false;
  }
  // working_dir
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->working_dir), &(rhs->working_dir)))
  {
    return false;
  }
  // session_name
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->session_name), &(rhs->session_name)))
  {
    return false;
  }
  // use_sim_time
  if (lhs->use_sim_time != rhs->use_sim_time) {
    return false;
  }
  // timeout
  if (lhs->timeout != rhs->timeout) {
    return false;
  }
  return true;
}

bool
rviz_manager_msgs__msg__ManagerLaunch__copy(
  const rviz_manager_msgs__msg__ManagerLaunch * input,
  rviz_manager_msgs__msg__ManagerLaunch * output)
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
  // id
  output->id = input->id;
  // action
  if (!rosidl_runtime_c__String__copy(
      &(input->action), &(output->action)))
  {
    return false;
  }
  // ns
  if (!rosidl_runtime_c__String__copy(
      &(input->ns), &(output->ns)))
  {
    return false;
  }
  // bash_session
  output->bash_session = input->bash_session;
  // is_launch_file
  output->is_launch_file = input->is_launch_file;
  // package
  if (!rosidl_runtime_c__String__copy(
      &(input->package), &(output->package)))
  {
    return false;
  }
  // executable
  if (!rosidl_runtime_c__String__copy(
      &(input->executable), &(output->executable)))
  {
    return false;
  }
  // arguments
  if (!rosidl_runtime_c__String__Sequence__copy(
      &(input->arguments), &(output->arguments)))
  {
    return false;
  }
  // ros_arguments
  if (!rosidl_runtime_c__String__Sequence__copy(
      &(input->ros_arguments), &(output->ros_arguments)))
  {
    return false;
  }
  // working_dir
  if (!rosidl_runtime_c__String__copy(
      &(input->working_dir), &(output->working_dir)))
  {
    return false;
  }
  // session_name
  if (!rosidl_runtime_c__String__copy(
      &(input->session_name), &(output->session_name)))
  {
    return false;
  }
  // use_sim_time
  output->use_sim_time = input->use_sim_time;
  // timeout
  output->timeout = input->timeout;
  return true;
}

rviz_manager_msgs__msg__ManagerLaunch *
rviz_manager_msgs__msg__ManagerLaunch__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  rviz_manager_msgs__msg__ManagerLaunch * msg = (rviz_manager_msgs__msg__ManagerLaunch *)allocator.allocate(sizeof(rviz_manager_msgs__msg__ManagerLaunch), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(rviz_manager_msgs__msg__ManagerLaunch));
  bool success = rviz_manager_msgs__msg__ManagerLaunch__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
rviz_manager_msgs__msg__ManagerLaunch__destroy(rviz_manager_msgs__msg__ManagerLaunch * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    rviz_manager_msgs__msg__ManagerLaunch__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
rviz_manager_msgs__msg__ManagerLaunch__Sequence__init(rviz_manager_msgs__msg__ManagerLaunch__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  rviz_manager_msgs__msg__ManagerLaunch * data = NULL;

  if (size) {
    data = (rviz_manager_msgs__msg__ManagerLaunch *)allocator.zero_allocate(size, sizeof(rviz_manager_msgs__msg__ManagerLaunch), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = rviz_manager_msgs__msg__ManagerLaunch__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        rviz_manager_msgs__msg__ManagerLaunch__fini(&data[i - 1]);
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
rviz_manager_msgs__msg__ManagerLaunch__Sequence__fini(rviz_manager_msgs__msg__ManagerLaunch__Sequence * array)
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
      rviz_manager_msgs__msg__ManagerLaunch__fini(&array->data[i]);
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

rviz_manager_msgs__msg__ManagerLaunch__Sequence *
rviz_manager_msgs__msg__ManagerLaunch__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  rviz_manager_msgs__msg__ManagerLaunch__Sequence * array = (rviz_manager_msgs__msg__ManagerLaunch__Sequence *)allocator.allocate(sizeof(rviz_manager_msgs__msg__ManagerLaunch__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = rviz_manager_msgs__msg__ManagerLaunch__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
rviz_manager_msgs__msg__ManagerLaunch__Sequence__destroy(rviz_manager_msgs__msg__ManagerLaunch__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    rviz_manager_msgs__msg__ManagerLaunch__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
rviz_manager_msgs__msg__ManagerLaunch__Sequence__are_equal(const rviz_manager_msgs__msg__ManagerLaunch__Sequence * lhs, const rviz_manager_msgs__msg__ManagerLaunch__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!rviz_manager_msgs__msg__ManagerLaunch__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
rviz_manager_msgs__msg__ManagerLaunch__Sequence__copy(
  const rviz_manager_msgs__msg__ManagerLaunch__Sequence * input,
  rviz_manager_msgs__msg__ManagerLaunch__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(rviz_manager_msgs__msg__ManagerLaunch);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    rviz_manager_msgs__msg__ManagerLaunch * data =
      (rviz_manager_msgs__msg__ManagerLaunch *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!rviz_manager_msgs__msg__ManagerLaunch__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          rviz_manager_msgs__msg__ManagerLaunch__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!rviz_manager_msgs__msg__ManagerLaunch__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
