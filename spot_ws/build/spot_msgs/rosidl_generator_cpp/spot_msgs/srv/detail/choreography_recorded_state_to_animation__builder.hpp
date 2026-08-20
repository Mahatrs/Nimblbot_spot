// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/ChoreographyRecordedStateToAnimation.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_RECORDED_STATE_TO_ANIMATION__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_RECORDED_STATE_TO_ANIMATION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/choreography_recorded_state_to_animation__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_ChoreographyRecordedStateToAnimation_Request_has_arm
{
public:
  Init_ChoreographyRecordedStateToAnimation_Request_has_arm()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request has_arm(::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request::_has_arm_type arg)
  {
    msg_.has_arm = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Request>()
{
  return spot_msgs::srv::builder::Init_ChoreographyRecordedStateToAnimation_Request_has_arm();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_ChoreographyRecordedStateToAnimation_Response_animation_file_contents
{
public:
  explicit Init_ChoreographyRecordedStateToAnimation_Response_animation_file_contents(::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response animation_file_contents(::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response::_animation_file_contents_type arg)
  {
    msg_.animation_file_contents = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response msg_;
};

class Init_ChoreographyRecordedStateToAnimation_Response_message
{
public:
  explicit Init_ChoreographyRecordedStateToAnimation_Response_message(::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response & msg)
  : msg_(msg)
  {}
  Init_ChoreographyRecordedStateToAnimation_Response_animation_file_contents message(::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return Init_ChoreographyRecordedStateToAnimation_Response_animation_file_contents(msg_);
  }

private:
  ::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response msg_;
};

class Init_ChoreographyRecordedStateToAnimation_Response_success
{
public:
  Init_ChoreographyRecordedStateToAnimation_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ChoreographyRecordedStateToAnimation_Response_message success(::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_ChoreographyRecordedStateToAnimation_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::ChoreographyRecordedStateToAnimation_Response>()
{
  return spot_msgs::srv::builder::Init_ChoreographyRecordedStateToAnimation_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__CHOREOGRAPHY_RECORDED_STATE_TO_ANIMATION__BUILDER_HPP_
