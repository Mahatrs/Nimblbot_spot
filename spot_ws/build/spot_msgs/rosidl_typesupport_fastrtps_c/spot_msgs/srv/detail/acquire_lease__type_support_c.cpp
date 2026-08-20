// generated from rosidl_typesupport_fastrtps_c/resource/idl__type_support_c.cpp.em
// with input from spot_msgs:srv/AcquireLease.idl
// generated code does not contain a copyright notice
#include "spot_msgs/srv/detail/acquire_lease__rosidl_typesupport_fastrtps_c.h"


#include <cassert>
#include <limits>
#include <string>
#include "rosidl_typesupport_fastrtps_c/identifier.h"
#include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "spot_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "spot_msgs/srv/detail/acquire_lease__struct.h"
#include "spot_msgs/srv/detail/acquire_lease__functions.h"
#include "fastcdr/Cdr.h"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

// includes and forward declarations of message dependencies and their conversion functions

#if defined(__cplusplus)
extern "C"
{
#endif

#include "rosidl_runtime_c/string.h"  // client_name, resource_name
#include "rosidl_runtime_c/string_functions.h"  // client_name, resource_name

// forward declare type support functions


using _AcquireLease_Request__ros_msg_type = spot_msgs__srv__AcquireLease_Request;

static bool _AcquireLease_Request__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _AcquireLease_Request__ros_msg_type * ros_message = static_cast<const _AcquireLease_Request__ros_msg_type *>(untyped_ros_message);
  // Field name: client_name
  {
    const rosidl_runtime_c__String * str = &ros_message->client_name;
    if (str->capacity == 0 || str->capacity <= str->size) {
      fprintf(stderr, "string capacity not greater than size\n");
      return false;
    }
    if (str->data[str->size] != '\0') {
      fprintf(stderr, "string not null-terminated\n");
      return false;
    }
    cdr << str->data;
  }

  // Field name: resource_name
  {
    const rosidl_runtime_c__String * str = &ros_message->resource_name;
    if (str->capacity == 0 || str->capacity <= str->size) {
      fprintf(stderr, "string capacity not greater than size\n");
      return false;
    }
    if (str->data[str->size] != '\0') {
      fprintf(stderr, "string not null-terminated\n");
      return false;
    }
    cdr << str->data;
  }

  // Field name: force
  {
    cdr << (ros_message->force ? true : false);
  }

  return true;
}

static bool _AcquireLease_Request__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _AcquireLease_Request__ros_msg_type * ros_message = static_cast<_AcquireLease_Request__ros_msg_type *>(untyped_ros_message);
  // Field name: client_name
  {
    std::string tmp;
    cdr >> tmp;
    if (!ros_message->client_name.data) {
      rosidl_runtime_c__String__init(&ros_message->client_name);
    }
    bool succeeded = rosidl_runtime_c__String__assign(
      &ros_message->client_name,
      tmp.c_str());
    if (!succeeded) {
      fprintf(stderr, "failed to assign string into field 'client_name'\n");
      return false;
    }
  }

  // Field name: resource_name
  {
    std::string tmp;
    cdr >> tmp;
    if (!ros_message->resource_name.data) {
      rosidl_runtime_c__String__init(&ros_message->resource_name);
    }
    bool succeeded = rosidl_runtime_c__String__assign(
      &ros_message->resource_name,
      tmp.c_str());
    if (!succeeded) {
      fprintf(stderr, "failed to assign string into field 'resource_name'\n");
      return false;
    }
  }

  // Field name: force
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->force = tmp ? true : false;
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_spot_msgs
size_t get_serialized_size_spot_msgs__srv__AcquireLease_Request(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _AcquireLease_Request__ros_msg_type * ros_message = static_cast<const _AcquireLease_Request__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name client_name
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message->client_name.size + 1);
  // field.name resource_name
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message->resource_name.size + 1);
  // field.name force
  {
    size_t item_size = sizeof(ros_message->force);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

static uint32_t _AcquireLease_Request__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_spot_msgs__srv__AcquireLease_Request(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_spot_msgs
size_t max_serialized_size_spot_msgs__srv__AcquireLease_Request(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  size_t last_member_size = 0;
  (void)last_member_size;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;

  // member: client_name
  {
    size_t array_size = 1;

    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }
  // member: resource_name
  {
    size_t array_size = 1;

    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }
  // member: force
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = spot_msgs__srv__AcquireLease_Request;
    is_plain =
      (
      offsetof(DataType, force) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static size_t _AcquireLease_Request__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_spot_msgs__srv__AcquireLease_Request(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_AcquireLease_Request = {
  "spot_msgs::srv",
  "AcquireLease_Request",
  _AcquireLease_Request__cdr_serialize,
  _AcquireLease_Request__cdr_deserialize,
  _AcquireLease_Request__get_serialized_size,
  _AcquireLease_Request__max_serialized_size
};

static rosidl_message_type_support_t _AcquireLease_Request__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_AcquireLease_Request,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, spot_msgs, srv, AcquireLease_Request)() {
  return &_AcquireLease_Request__type_support;
}

#if defined(__cplusplus)
}
#endif

// already included above
// #include <cassert>
// already included above
// #include <limits>
// already included above
// #include <string>
// already included above
// #include "rosidl_typesupport_fastrtps_c/identifier.h"
// already included above
// #include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
// already included above
// #include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
// already included above
// #include "spot_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
// already included above
// #include "spot_msgs/srv/detail/acquire_lease__struct.h"
// already included above
// #include "spot_msgs/srv/detail/acquire_lease__functions.h"
// already included above
// #include "fastcdr/Cdr.h"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

// includes and forward declarations of message dependencies and their conversion functions

#if defined(__cplusplus)
extern "C"
{
#endif

#include "bosdyn_api_msgs/msg/detail/lease__functions.h"  // lease
// already included above
// #include "rosidl_runtime_c/string.h"  // message
// already included above
// #include "rosidl_runtime_c/string_functions.h"  // message

// forward declare type support functions
ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_spot_msgs
size_t get_serialized_size_bosdyn_api_msgs__msg__Lease(
  const void * untyped_ros_message,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_spot_msgs
size_t max_serialized_size_bosdyn_api_msgs__msg__Lease(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_spot_msgs
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, bosdyn_api_msgs, msg, Lease)();


using _AcquireLease_Response__ros_msg_type = spot_msgs__srv__AcquireLease_Response;

static bool _AcquireLease_Response__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _AcquireLease_Response__ros_msg_type * ros_message = static_cast<const _AcquireLease_Response__ros_msg_type *>(untyped_ros_message);
  // Field name: success
  {
    cdr << (ros_message->success ? true : false);
  }

  // Field name: message
  {
    const rosidl_runtime_c__String * str = &ros_message->message;
    if (str->capacity == 0 || str->capacity <= str->size) {
      fprintf(stderr, "string capacity not greater than size\n");
      return false;
    }
    if (str->data[str->size] != '\0') {
      fprintf(stderr, "string not null-terminated\n");
      return false;
    }
    cdr << str->data;
  }

  // Field name: lease
  {
    const message_type_support_callbacks_t * callbacks =
      static_cast<const message_type_support_callbacks_t *>(
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
        rosidl_typesupport_fastrtps_c, bosdyn_api_msgs, msg, Lease
      )()->data);
    if (!callbacks->cdr_serialize(
        &ros_message->lease, cdr))
    {
      return false;
    }
  }

  return true;
}

static bool _AcquireLease_Response__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _AcquireLease_Response__ros_msg_type * ros_message = static_cast<_AcquireLease_Response__ros_msg_type *>(untyped_ros_message);
  // Field name: success
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->success = tmp ? true : false;
  }

  // Field name: message
  {
    std::string tmp;
    cdr >> tmp;
    if (!ros_message->message.data) {
      rosidl_runtime_c__String__init(&ros_message->message);
    }
    bool succeeded = rosidl_runtime_c__String__assign(
      &ros_message->message,
      tmp.c_str());
    if (!succeeded) {
      fprintf(stderr, "failed to assign string into field 'message'\n");
      return false;
    }
  }

  // Field name: lease
  {
    const message_type_support_callbacks_t * callbacks =
      static_cast<const message_type_support_callbacks_t *>(
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
        rosidl_typesupport_fastrtps_c, bosdyn_api_msgs, msg, Lease
      )()->data);
    if (!callbacks->cdr_deserialize(
        cdr, &ros_message->lease))
    {
      return false;
    }
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_spot_msgs
size_t get_serialized_size_spot_msgs__srv__AcquireLease_Response(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _AcquireLease_Response__ros_msg_type * ros_message = static_cast<const _AcquireLease_Response__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name success
  {
    size_t item_size = sizeof(ros_message->success);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name message
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message->message.size + 1);
  // field.name lease

  current_alignment += get_serialized_size_bosdyn_api_msgs__msg__Lease(
    &(ros_message->lease), current_alignment);

  return current_alignment - initial_alignment;
}

static uint32_t _AcquireLease_Response__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_spot_msgs__srv__AcquireLease_Response(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_spot_msgs
size_t max_serialized_size_spot_msgs__srv__AcquireLease_Response(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  size_t last_member_size = 0;
  (void)last_member_size;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;

  // member: success
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: message
  {
    size_t array_size = 1;

    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }
  // member: lease
  {
    size_t array_size = 1;


    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_bosdyn_api_msgs__msg__Lease(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = spot_msgs__srv__AcquireLease_Response;
    is_plain =
      (
      offsetof(DataType, lease) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static size_t _AcquireLease_Response__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_spot_msgs__srv__AcquireLease_Response(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_AcquireLease_Response = {
  "spot_msgs::srv",
  "AcquireLease_Response",
  _AcquireLease_Response__cdr_serialize,
  _AcquireLease_Response__cdr_deserialize,
  _AcquireLease_Response__get_serialized_size,
  _AcquireLease_Response__max_serialized_size
};

static rosidl_message_type_support_t _AcquireLease_Response__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_AcquireLease_Response,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, spot_msgs, srv, AcquireLease_Response)() {
  return &_AcquireLease_Response__type_support;
}

#if defined(__cplusplus)
}
#endif

#include "rosidl_typesupport_fastrtps_cpp/service_type_support.h"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_fastrtps_c/identifier.h"
// already included above
// #include "spot_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "spot_msgs/srv/acquire_lease.h"

#if defined(__cplusplus)
extern "C"
{
#endif

static service_type_support_callbacks_t AcquireLease__callbacks = {
  "spot_msgs::srv",
  "AcquireLease",
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, spot_msgs, srv, AcquireLease_Request)(),
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, spot_msgs, srv, AcquireLease_Response)(),
};

static rosidl_service_type_support_t AcquireLease__handle = {
  rosidl_typesupport_fastrtps_c__identifier,
  &AcquireLease__callbacks,
  get_service_typesupport_handle_function,
};

const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, spot_msgs, srv, AcquireLease)() {
  return &AcquireLease__handle;
}

#if defined(__cplusplus)
}
#endif
