// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/MutateWorldObject.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__MUTATE_WORLD_OBJECT__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__MUTATE_WORLD_OBJECT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/mutate_world_object__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_MutateWorldObject_Request_request
{
public:
  Init_MutateWorldObject_Request_request()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::MutateWorldObject_Request request(::spot_msgs::srv::MutateWorldObject_Request::_request_type arg)
  {
    msg_.request = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::MutateWorldObject_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::MutateWorldObject_Request>()
{
  return spot_msgs::srv::builder::Init_MutateWorldObject_Request_request();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_MutateWorldObject_Response_response
{
public:
  Init_MutateWorldObject_Response_response()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::MutateWorldObject_Response response(::spot_msgs::srv::MutateWorldObject_Response::_response_type arg)
  {
    msg_.response = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::MutateWorldObject_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::MutateWorldObject_Response>()
{
  return spot_msgs::srv::builder::Init_MutateWorldObject_Response_response();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__MUTATE_WORLD_OBJECT__BUILDER_HPP_
