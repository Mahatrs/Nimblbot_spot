// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from rviz_manager_msgs:msg/ManagerLaunch.idl
// generated code does not contain a copyright notice

#ifndef RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__TRAITS_HPP_
#define RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "rviz_manager_msgs/msg/detail/manager_launch__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__traits.hpp"

namespace rviz_manager_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const ManagerLaunch & msg,
  std::ostream & out)
{
  out << "{";
  // member: header
  {
    out << "header: ";
    to_flow_style_yaml(msg.header, out);
    out << ", ";
  }

  // member: id
  {
    out << "id: ";
    rosidl_generator_traits::value_to_yaml(msg.id, out);
    out << ", ";
  }

  // member: action
  {
    out << "action: ";
    rosidl_generator_traits::value_to_yaml(msg.action, out);
    out << ", ";
  }

  // member: ns
  {
    out << "ns: ";
    rosidl_generator_traits::value_to_yaml(msg.ns, out);
    out << ", ";
  }

  // member: bash_session
  {
    out << "bash_session: ";
    rosidl_generator_traits::value_to_yaml(msg.bash_session, out);
    out << ", ";
  }

  // member: is_launch_file
  {
    out << "is_launch_file: ";
    rosidl_generator_traits::value_to_yaml(msg.is_launch_file, out);
    out << ", ";
  }

  // member: package
  {
    out << "package: ";
    rosidl_generator_traits::value_to_yaml(msg.package, out);
    out << ", ";
  }

  // member: executable
  {
    out << "executable: ";
    rosidl_generator_traits::value_to_yaml(msg.executable, out);
    out << ", ";
  }

  // member: arguments
  {
    if (msg.arguments.size() == 0) {
      out << "arguments: []";
    } else {
      out << "arguments: [";
      size_t pending_items = msg.arguments.size();
      for (auto item : msg.arguments) {
        rosidl_generator_traits::value_to_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
    out << ", ";
  }

  // member: ros_arguments
  {
    if (msg.ros_arguments.size() == 0) {
      out << "ros_arguments: []";
    } else {
      out << "ros_arguments: [";
      size_t pending_items = msg.ros_arguments.size();
      for (auto item : msg.ros_arguments) {
        rosidl_generator_traits::value_to_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
    out << ", ";
  }

  // member: working_dir
  {
    out << "working_dir: ";
    rosidl_generator_traits::value_to_yaml(msg.working_dir, out);
    out << ", ";
  }

  // member: session_name
  {
    out << "session_name: ";
    rosidl_generator_traits::value_to_yaml(msg.session_name, out);
    out << ", ";
  }

  // member: use_sim_time
  {
    out << "use_sim_time: ";
    rosidl_generator_traits::value_to_yaml(msg.use_sim_time, out);
    out << ", ";
  }

  // member: timeout
  {
    out << "timeout: ";
    rosidl_generator_traits::value_to_yaml(msg.timeout, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ManagerLaunch & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: header
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "header:\n";
    to_block_style_yaml(msg.header, out, indentation + 2);
  }

  // member: id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "id: ";
    rosidl_generator_traits::value_to_yaml(msg.id, out);
    out << "\n";
  }

  // member: action
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "action: ";
    rosidl_generator_traits::value_to_yaml(msg.action, out);
    out << "\n";
  }

  // member: ns
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "ns: ";
    rosidl_generator_traits::value_to_yaml(msg.ns, out);
    out << "\n";
  }

  // member: bash_session
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "bash_session: ";
    rosidl_generator_traits::value_to_yaml(msg.bash_session, out);
    out << "\n";
  }

  // member: is_launch_file
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "is_launch_file: ";
    rosidl_generator_traits::value_to_yaml(msg.is_launch_file, out);
    out << "\n";
  }

  // member: package
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "package: ";
    rosidl_generator_traits::value_to_yaml(msg.package, out);
    out << "\n";
  }

  // member: executable
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "executable: ";
    rosidl_generator_traits::value_to_yaml(msg.executable, out);
    out << "\n";
  }

  // member: arguments
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.arguments.size() == 0) {
      out << "arguments: []\n";
    } else {
      out << "arguments:\n";
      for (auto item : msg.arguments) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "- ";
        rosidl_generator_traits::value_to_yaml(item, out);
        out << "\n";
      }
    }
  }

  // member: ros_arguments
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.ros_arguments.size() == 0) {
      out << "ros_arguments: []\n";
    } else {
      out << "ros_arguments:\n";
      for (auto item : msg.ros_arguments) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "- ";
        rosidl_generator_traits::value_to_yaml(item, out);
        out << "\n";
      }
    }
  }

  // member: working_dir
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "working_dir: ";
    rosidl_generator_traits::value_to_yaml(msg.working_dir, out);
    out << "\n";
  }

  // member: session_name
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "session_name: ";
    rosidl_generator_traits::value_to_yaml(msg.session_name, out);
    out << "\n";
  }

  // member: use_sim_time
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "use_sim_time: ";
    rosidl_generator_traits::value_to_yaml(msg.use_sim_time, out);
    out << "\n";
  }

  // member: timeout
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "timeout: ";
    rosidl_generator_traits::value_to_yaml(msg.timeout, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ManagerLaunch & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace msg

}  // namespace rviz_manager_msgs

namespace rosidl_generator_traits
{

[[deprecated("use rviz_manager_msgs::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const rviz_manager_msgs::msg::ManagerLaunch & msg,
  std::ostream & out, size_t indentation = 0)
{
  rviz_manager_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use rviz_manager_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const rviz_manager_msgs::msg::ManagerLaunch & msg)
{
  return rviz_manager_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<rviz_manager_msgs::msg::ManagerLaunch>()
{
  return "rviz_manager_msgs::msg::ManagerLaunch";
}

template<>
inline const char * name<rviz_manager_msgs::msg::ManagerLaunch>()
{
  return "rviz_manager_msgs/msg/ManagerLaunch";
}

template<>
struct has_fixed_size<rviz_manager_msgs::msg::ManagerLaunch>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<rviz_manager_msgs::msg::ManagerLaunch>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<rviz_manager_msgs::msg::ManagerLaunch>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // RVIZ_MANAGER_MSGS__MSG__DETAIL__MANAGER_LAUNCH__TRAITS_HPP_
