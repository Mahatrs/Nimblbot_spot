// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/GetPtzPosition.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__GET_PTZ_POSITION__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__GET_PTZ_POSITION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/get_ptz_position__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_GetPtzPosition_Request_name
{
public:
  Init_GetPtzPosition_Request_name()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::GetPtzPosition_Request name(::spot_msgs::srv::GetPtzPosition_Request::_name_type arg)
  {
    msg_.name = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::GetPtzPosition_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::GetPtzPosition_Request>()
{
  return spot_msgs::srv::builder::Init_GetPtzPosition_Request_name();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_GetPtzPosition_Response_position
{
public:
  explicit Init_GetPtzPosition_Response_position(::spot_msgs::srv::GetPtzPosition_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::GetPtzPosition_Response position(::spot_msgs::srv::GetPtzPosition_Response::_position_type arg)
  {
    msg_.position = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::GetPtzPosition_Response msg_;
};

class Init_GetPtzPosition_Response_message
{
public:
  explicit Init_GetPtzPosition_Response_message(::spot_msgs::srv::GetPtzPosition_Response & msg)
  : msg_(msg)
  {}
  Init_GetPtzPosition_Response_position message(::spot_msgs::srv::GetPtzPosition_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return Init_GetPtzPosition_Response_position(msg_);
  }

private:
  ::spot_msgs::srv::GetPtzPosition_Response msg_;
};

class Init_GetPtzPosition_Response_success
{
public:
  Init_GetPtzPosition_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetPtzPosition_Response_message success(::spot_msgs::srv::GetPtzPosition_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_GetPtzPosition_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::GetPtzPosition_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::GetPtzPosition_Response>()
{
  return spot_msgs::srv::builder::Init_GetPtzPosition_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__GET_PTZ_POSITION__BUILDER_HPP_
