// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:srv/GetInverseKinematicSolutions.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__SRV__DETAIL__GET_INVERSE_KINEMATIC_SOLUTIONS__BUILDER_HPP_
#define SPOT_MSGS__SRV__DETAIL__GET_INVERSE_KINEMATIC_SOLUTIONS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/srv/detail/get_inverse_kinematic_solutions__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_GetInverseKinematicSolutions_Request_request
{
public:
  Init_GetInverseKinematicSolutions_Request_request()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::GetInverseKinematicSolutions_Request request(::spot_msgs::srv::GetInverseKinematicSolutions_Request::_request_type arg)
  {
    msg_.request = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::GetInverseKinematicSolutions_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::GetInverseKinematicSolutions_Request>()
{
  return spot_msgs::srv::builder::Init_GetInverseKinematicSolutions_Request_request();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace srv
{

namespace builder
{

class Init_GetInverseKinematicSolutions_Response_response
{
public:
  Init_GetInverseKinematicSolutions_Response_response()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::srv::GetInverseKinematicSolutions_Response response(::spot_msgs::srv::GetInverseKinematicSolutions_Response::_response_type arg)
  {
    msg_.response = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::srv::GetInverseKinematicSolutions_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::srv::GetInverseKinematicSolutions_Response>()
{
  return spot_msgs::srv::builder::Init_GetInverseKinematicSolutions_Response_response();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__SRV__DETAIL__GET_INVERSE_KINEMATIC_SOLUTIONS__BUILDER_HPP_
