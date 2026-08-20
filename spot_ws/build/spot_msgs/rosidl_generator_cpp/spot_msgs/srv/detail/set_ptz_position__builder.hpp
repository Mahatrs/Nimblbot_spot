// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/SetPtzPosition.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__SET_PTZ_POSITION__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__SET_PTZ_POSITION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/set_ptz_position__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_SetPtzPosition_Request_zoom
{
public:
  explicit Init_SetPtzPosition_Request_zoom(::spot_msgs::srv::SetPtzPosition_Request & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::SetPtzPosition_Request zoom(::spot_msgs::srv::SetPtzPosition_Request::_zoom_type arg)
  {
    msg_.zoom = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::SetPtzPosition_Request msg_;
};

class Init_SetPtzPosition_Request_tilt
{
public:
  explicit Init_SetPtzPosition_Request_tilt(::spot_msgs::srv::SetPtzPosition_Request & msg)
  : msg_(msg)
  {}
  Init_SetPtzPosition_Request_zoom tilt(::spot_msgs::srv::SetPtzPosition_Request::_tilt_type arg)
  {
    msg_.tilt = std::move(arg);
    return Init_SetPtzPosition_Request_zoom(msg_);
  }

private:
  ::spot_msgs::srv::SetPtzPosition_Request msg_;
};

class Init_SetPtzPosition_Request_pan
{
public:
  explicit Init_SetPtzPosition_Request_pan(::spot_msgs::srv::SetPtzPosition_Request & msg)
  : msg_(msg)
  {}
  Init_SetPtzPosition_Request_tilt pan(::spot_msgs::srv::SetPtzPosition_Request::_pan_type arg)
  {
    msg_.pan = std::move(arg);
    return Init_SetPtzPosition_Request_tilt(msg_);
  }

private:
  ::spot_msgs::srv::SetPtzPosition_Request msg_;
};

class Init_SetPtzPosition_Request_name
{
public:
  Init_SetPtzPosition_Request_name()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetPtzPosition_Request_pan name(::spot_msgs::srv::SetPtzPosition_Request::_name_type arg)
  {
    msg_.name = std::move(arg);
    return Init_SetPtzPosition_Request_pan(msg_);
  }

private:
  ::spot_msgs::srv::SetPtzPosition_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::SetPtzPosition_Request>()
{
  return spot_msgs::srv::builder::Init_SetPtzPosition_Request_name();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_SetPtzPosition_Response_message
{
public:
  explicit Init_SetPtzPosition_Response_message(::spot_msgs::srv::SetPtzPosition_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::SetPtzPosition_Response message(::spot_msgs::srv::SetPtzPosition_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::SetPtzPosition_Response msg_;
};

class Init_SetPtzPosition_Response_success
{
public:
  Init_SetPtzPosition_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetPtzPosition_Response_message success(::spot_msgs::srv::SetPtzPosition_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SetPtzPosition_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::SetPtzPosition_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::SetPtzPosition_Response>()
{
  return spot_msgs::srv::builder::Init_SetPtzPosition_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__SET_PTZ_POSITION__BUILDER_HPP_
