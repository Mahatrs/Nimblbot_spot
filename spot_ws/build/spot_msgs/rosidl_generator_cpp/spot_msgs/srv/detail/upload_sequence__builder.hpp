// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/UploadSequence.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__UPLOAD_SEQUENCE__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__UPLOAD_SEQUENCE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/upload_sequence__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_UploadSequence_Request_sequence_proto_serialized
{
public:
  Init_UploadSequence_Request_sequence_proto_serialized()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::UploadSequence_Request sequence_proto_serialized(::spot_msgs::srv::UploadSequence_Request::_sequence_proto_serialized_type arg)
  {
    msg_.sequence_proto_serialized = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::UploadSequence_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::UploadSequence_Request>()
{
  return spot_msgs::srv::builder::Init_UploadSequence_Request_sequence_proto_serialized();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_UploadSequence_Response_message
{
public:
  explicit Init_UploadSequence_Response_message(::spot_msgs::srv::UploadSequence_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::srv::UploadSequence_Response message(::spot_msgs::srv::UploadSequence_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::UploadSequence_Response msg_;
};

class Init_UploadSequence_Response_success
{
public:
  Init_UploadSequence_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_UploadSequence_Response_message success(::spot_msgs::srv::UploadSequence_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_UploadSequence_Response_message(msg_);
  }

private:
  ::spot_msgs::srv::UploadSequence_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::UploadSequence_Response>()
{
  return spot_msgs::srv::builder::Init_UploadSequence_Response_success();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__UPLOAD_SEQUENCE__BUILDER_HPP_
