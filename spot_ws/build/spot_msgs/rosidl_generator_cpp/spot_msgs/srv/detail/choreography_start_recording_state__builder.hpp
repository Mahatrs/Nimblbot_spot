// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/ChoreographyStartRecordingState.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/choreography_start_recording_state__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_ChoreographyStartRecordingState_Request_duration_seconds
{
public:
  Init_ChoreographyStartRecordingState_Request_duration_seconds()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::ChoreographyStartRecordingState_Request duration_seconds(::spot_msgs::srv::ChoreographyStartRecordingState_Request::_duration_seconds_type arg)
  {
    msg_.duration_seconds = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::ChoreographyStartRecordingState_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::ChoreographyStartRecordingState_Request>()
{
  return spot_msgs::srv::builder::Init_ChoreographyStartRecordingState_Request_duration_seconds();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_ChoreographyStartRecordingState_Response_recording_session_id
{
public:
  explicit Init_ChoreographyStartRecordingState_Response_recording_session_id(::spot_msgs::srv::ChoreographyStartRecordingState_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::ChoreographyStartRecordingState_Response recording_session_id(::spot_msgs::srv::ChoreographyStartRecordingState_Response::_recording_session_id_type arg)
  {
    msg_.recording_session_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::ChoreographyStartRecordingState_Response msg_;
};

class Init_ChoreographyStartRecordingState_Response_status
{
public:
  explicit Init_ChoreographyStartRecordingState_Response_status(::spot_msgs::srv::ChoreographyStartRecordingState_Response & msg)
  : msg_(msg)
  {}
  Init_ChoreographyStartRecordingState_Response_recording_session_id status(::spot_msgs::srv::ChoreographyStartRecordingState_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_ChoreographyStartRecordingState_Response_recording_session_id(msg_);
  }

private:
  ::spot_msgs::srv::ChoreographyStartRecordingState_Response msg_;
};

class Init_ChoreographyStartRecordingState_Response_message
{
public:
  explicit Init_ChoreographyStartRecordingState_Response_message(::spot_msgs::srv::ChoreographyStartRecordingState_Response & msg)
  : msg_(msg)
  {}
  Init_ChoreographyStartRecordingState_Response_status message(::spot_msgs::srv::ChoreographyStartRecordingState_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return Init_ChoreographyStartRecordingState_Response_status(msg_);
  }

private:
  ::spot_msgs::srv::ChoreographyStartRecordingState_Response msg_;
};

class Init_ChoreographyStartRecordingState_Response_success
{
public:
  Init_ChoreographyStartRecordingState_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ChoreographyStartRecordingState_Response_message success(::spot_msgs::srv::ChoreographyStartRecordingState_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_ChoreographyStartRecordingState_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::ChoreographyStartRecordingState_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::ChoreographyStartRecordingState_Response>()
{
  return spot_msgs::srv::builder::Init_ChoreographyStartRecordingState_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_START_RECORDING_STATE__BUILDER_HPP_
