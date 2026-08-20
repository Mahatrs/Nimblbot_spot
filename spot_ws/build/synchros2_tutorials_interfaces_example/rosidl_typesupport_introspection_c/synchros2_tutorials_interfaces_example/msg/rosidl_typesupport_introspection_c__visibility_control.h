// generated from
// rosidl_typesupport_introspection_c/resource/rosidl_typesupport_introspection_c__visibility_control.h.in
// generated code does not contain a copyright notice

#ifndef SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__ROSIDL_TYPESUPPORT_INTROSPECTION_C__VISIBILITY_CONTROL_H_
#define SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__ROSIDL_TYPESUPPORT_INTROSPECTION_C__VISIBILITY_CONTROL_H_

#ifdef __cplusplus
extern "C"
{
#endif

// This logic was borrowed (then namespaced) from the examples on the gcc wiki:
//     https://gcc.gnu.org/wiki/Visibility

#if defined _WIN32 || defined __CYGWIN__
  #ifdef __GNUC__
    #define ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example __attribute__ ((dllexport))
    #define ROSIDL_TYPESUPPORT_INTROSPECTION_C_IMPORT_synchros2_tutorials_interfaces_example __attribute__ ((dllimport))
  #else
    #define ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example __declspec(dllexport)
    #define ROSIDL_TYPESUPPORT_INTROSPECTION_C_IMPORT_synchros2_tutorials_interfaces_example __declspec(dllimport)
  #endif
  #ifdef ROSIDL_TYPESUPPORT_INTROSPECTION_C_BUILDING_DLL_synchros2_tutorials_interfaces_example
    #define ROSIDL_TYPESUPPORT_INTROSPECTION_C_PUBLIC_synchros2_tutorials_interfaces_example ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example
  #else
    #define ROSIDL_TYPESUPPORT_INTROSPECTION_C_PUBLIC_synchros2_tutorials_interfaces_example ROSIDL_TYPESUPPORT_INTROSPECTION_C_IMPORT_synchros2_tutorials_interfaces_example
  #endif
#else
  #define ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_synchros2_tutorials_interfaces_example __attribute__ ((visibility("default")))
  #define ROSIDL_TYPESUPPORT_INTROSPECTION_C_IMPORT_synchros2_tutorials_interfaces_example
  #if __GNUC__ >= 4
    #define ROSIDL_TYPESUPPORT_INTROSPECTION_C_PUBLIC_synchros2_tutorials_interfaces_example __attribute__ ((visibility("default")))
  #else
    #define ROSIDL_TYPESUPPORT_INTROSPECTION_C_PUBLIC_synchros2_tutorials_interfaces_example
  #endif
#endif

#ifdef __cplusplus
}
#endif

#endif  // SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__MSG__ROSIDL_TYPESUPPORT_INTROSPECTION_C__VISIBILITY_CONTROL_H_
