// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from nanotec_msgs:msg/DeviceStatus.idl
// generated code does not contain a copyright notice
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
#include <Python.h>
#include <stdbool.h>
#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-function"
#endif
#include "numpy/ndarrayobject.h"
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif
#include "rosidl_runtime_c/visibility_control.h"
#include "nanotec_msgs/msg/detail/device_status__struct.h"
#include "nanotec_msgs/msg/detail/device_status__functions.h"

#include "rosidl_runtime_c/string.h"
#include "rosidl_runtime_c/string_functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool nanotec_msgs__msg__device_status__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[45];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("nanotec_msgs.msg._device_status.DeviceStatus", full_classname_dest, 44) == 0);
  }
  nanotec_msgs__msg__DeviceStatus * ros_message = _ros_message;
  {  // name
    PyObject * field = PyObject_GetAttrString(_pymsg, "name");
    if (!field) {
      return false;
    }
    assert(PyUnicode_Check(field));
    PyObject * encoded_field = PyUnicode_AsUTF8String(field);
    if (!encoded_field) {
      Py_DECREF(field);
      return false;
    }
    rosidl_runtime_c__String__assign(&ros_message->name, PyBytes_AS_STRING(encoded_field));
    Py_DECREF(encoded_field);
    Py_DECREF(field);
  }
  {  // voltage_power
    PyObject * field = PyObject_GetAttrString(_pymsg, "voltage_power");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->voltage_power = (float)PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // voltage_logic
    PyObject * field = PyObject_GetAttrString(_pymsg, "voltage_logic");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->voltage_logic = (float)PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // temperature_motor
    PyObject * field = PyObject_GetAttrString(_pymsg, "temperature_motor");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->temperature_motor = (float)PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // temperature_micro_chip
    PyObject * field = PyObject_GetAttrString(_pymsg, "temperature_micro_chip");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->temperature_micro_chip = (float)PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // ready_to_switch_on
    PyObject * field = PyObject_GetAttrString(_pymsg, "ready_to_switch_on");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->ready_to_switch_on = (Py_True == field);
    Py_DECREF(field);
  }
  {  // switched_on
    PyObject * field = PyObject_GetAttrString(_pymsg, "switched_on");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->switched_on = (Py_True == field);
    Py_DECREF(field);
  }
  {  // operation_enabled
    PyObject * field = PyObject_GetAttrString(_pymsg, "operation_enabled");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->operation_enabled = (Py_True == field);
    Py_DECREF(field);
  }
  {  // fault
    PyObject * field = PyObject_GetAttrString(_pymsg, "fault");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->fault = (Py_True == field);
    Py_DECREF(field);
  }
  {  // voltage_enabled
    PyObject * field = PyObject_GetAttrString(_pymsg, "voltage_enabled");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->voltage_enabled = (Py_True == field);
    Py_DECREF(field);
  }
  {  // quick_stop
    PyObject * field = PyObject_GetAttrString(_pymsg, "quick_stop");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->quick_stop = (Py_True == field);
    Py_DECREF(field);
  }
  {  // switch_on_disabled
    PyObject * field = PyObject_GetAttrString(_pymsg, "switch_on_disabled");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->switch_on_disabled = (Py_True == field);
    Py_DECREF(field);
  }
  {  // warning
    PyObject * field = PyObject_GetAttrString(_pymsg, "warning");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->warning = (Py_True == field);
    Py_DECREF(field);
  }
  {  // target_reached
    PyObject * field = PyObject_GetAttrString(_pymsg, "target_reached");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->target_reached = (Py_True == field);
    Py_DECREF(field);
  }
  {  // internal_limit_active
    PyObject * field = PyObject_GetAttrString(_pymsg, "internal_limit_active");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->internal_limit_active = (Py_True == field);
    Py_DECREF(field);
  }
  {  // operation_mode_specific
    PyObject * field = PyObject_GetAttrString(_pymsg, "operation_mode_specific");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->operation_mode_specific = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // homing_status
    PyObject * field = PyObject_GetAttrString(_pymsg, "homing_status");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->homing_status = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * nanotec_msgs__msg__device_status__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of DeviceStatus */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("nanotec_msgs.msg._device_status");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "DeviceStatus");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  nanotec_msgs__msg__DeviceStatus * ros_message = (nanotec_msgs__msg__DeviceStatus *)raw_ros_message;
  {  // name
    PyObject * field = NULL;
    field = PyUnicode_DecodeUTF8(
      ros_message->name.data,
      strlen(ros_message->name.data),
      "replace");
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "name", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // voltage_power
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->voltage_power);
    {
      int rc = PyObject_SetAttrString(_pymessage, "voltage_power", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // voltage_logic
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->voltage_logic);
    {
      int rc = PyObject_SetAttrString(_pymessage, "voltage_logic", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // temperature_motor
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->temperature_motor);
    {
      int rc = PyObject_SetAttrString(_pymessage, "temperature_motor", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // temperature_micro_chip
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->temperature_micro_chip);
    {
      int rc = PyObject_SetAttrString(_pymessage, "temperature_micro_chip", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // ready_to_switch_on
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->ready_to_switch_on ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "ready_to_switch_on", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // switched_on
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->switched_on ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "switched_on", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // operation_enabled
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->operation_enabled ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "operation_enabled", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // fault
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->fault ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "fault", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // voltage_enabled
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->voltage_enabled ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "voltage_enabled", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // quick_stop
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->quick_stop ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "quick_stop", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // switch_on_disabled
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->switch_on_disabled ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "switch_on_disabled", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // warning
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->warning ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "warning", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // target_reached
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->target_reached ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "target_reached", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // internal_limit_active
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->internal_limit_active ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "internal_limit_active", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // operation_mode_specific
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->operation_mode_specific);
    {
      int rc = PyObject_SetAttrString(_pymessage, "operation_mode_specific", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // homing_status
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->homing_status);
    {
      int rc = PyObject_SetAttrString(_pymessage, "homing_status", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
