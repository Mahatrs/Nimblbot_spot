// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from spot_msgs:action/ExecuteDance.idl
// generated code does not contain a copyright notice

#ifndef SPOT_MSGS__ACTION__DETAIL__EXECUTE_DANCE__BUILDER_HPP_
#define SPOT_MSGS__ACTION__DETAIL__EXECUTE_DANCE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "spot_msgs/action/detail/execute_dance__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace spot_msgs
{

namespace action
{

namespace builder
{

class Init_ExecuteDance_Goal_start_slice
{
public:
  explicit Init_ExecuteDance_Goal_start_slice(::spot_msgs::action::ExecuteDance_Goal & msg)
  : msg_(msg)
  {}
  ::spot_msgs::action::ExecuteDance_Goal start_slice(::spot_msgs::action::ExecuteDance_Goal::_start_slice_type arg)
  {
    msg_.start_slice = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_Goal msg_;
};

class Init_ExecuteDance_Goal_choreo_sequence_serialized
{
public:
  explicit Init_ExecuteDance_Goal_choreo_sequence_serialized(::spot_msgs::action::ExecuteDance_Goal & msg)
  : msg_(msg)
  {}
  Init_ExecuteDance_Goal_start_slice choreo_sequence_serialized(::spot_msgs::action::ExecuteDance_Goal::_choreo_sequence_serialized_type arg)
  {
    msg_.choreo_sequence_serialized = std::move(arg);
    return Init_ExecuteDance_Goal_start_slice(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_Goal msg_;
};

class Init_ExecuteDance_Goal_choreo_file_content
{
public:
  explicit Init_ExecuteDance_Goal_choreo_file_content(::spot_msgs::action::ExecuteDance_Goal & msg)
  : msg_(msg)
  {}
  Init_ExecuteDance_Goal_choreo_sequence_serialized choreo_file_content(::spot_msgs::action::ExecuteDance_Goal::_choreo_file_content_type arg)
  {
    msg_.choreo_file_content = std::move(arg);
    return Init_ExecuteDance_Goal_choreo_sequence_serialized(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_Goal msg_;
};

class Init_ExecuteDance_Goal_choreo_name
{
public:
  Init_ExecuteDance_Goal_choreo_name()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ExecuteDance_Goal_choreo_file_content choreo_name(::spot_msgs::action::ExecuteDance_Goal::_choreo_name_type arg)
  {
    msg_.choreo_name = std::move(arg);
    return Init_ExecuteDance_Goal_choreo_file_content(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::action::ExecuteDance_Goal>()
{
  return spot_msgs::action::builder::Init_ExecuteDance_Goal_choreo_name();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace action
{

namespace builder
{

class Init_ExecuteDance_Result_message
{
public:
  explicit Init_ExecuteDance_Result_message(::spot_msgs::action::ExecuteDance_Result & msg)
  : msg_(msg)
  {}
  ::spot_msgs::action::ExecuteDance_Result message(::spot_msgs::action::ExecuteDance_Result::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_Result msg_;
};

class Init_ExecuteDance_Result_success
{
public:
  Init_ExecuteDance_Result_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ExecuteDance_Result_message success(::spot_msgs::action::ExecuteDance_Result::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_ExecuteDance_Result_message(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::action::ExecuteDance_Result>()
{
  return spot_msgs::action::builder::Init_ExecuteDance_Result_success();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace action
{

namespace builder
{

class Init_ExecuteDance_Feedback_is_dancing
{
public:
  Init_ExecuteDance_Feedback_is_dancing()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::action::ExecuteDance_Feedback is_dancing(::spot_msgs::action::ExecuteDance_Feedback::_is_dancing_type arg)
  {
    msg_.is_dancing = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_Feedback msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::action::ExecuteDance_Feedback>()
{
  return spot_msgs::action::builder::Init_ExecuteDance_Feedback_is_dancing();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace action
{

namespace builder
{

class Init_ExecuteDance_SendGoal_Request_goal
{
public:
  explicit Init_ExecuteDance_SendGoal_Request_goal(::spot_msgs::action::ExecuteDance_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::spot_msgs::action::ExecuteDance_SendGoal_Request goal(::spot_msgs::action::ExecuteDance_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_SendGoal_Request msg_;
};

class Init_ExecuteDance_SendGoal_Request_goal_id
{
public:
  Init_ExecuteDance_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ExecuteDance_SendGoal_Request_goal goal_id(::spot_msgs::action::ExecuteDance_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_ExecuteDance_SendGoal_Request_goal(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::action::ExecuteDance_SendGoal_Request>()
{
  return spot_msgs::action::builder::Init_ExecuteDance_SendGoal_Request_goal_id();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace action
{

namespace builder
{

class Init_ExecuteDance_SendGoal_Response_stamp
{
public:
  explicit Init_ExecuteDance_SendGoal_Response_stamp(::spot_msgs::action::ExecuteDance_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::action::ExecuteDance_SendGoal_Response stamp(::spot_msgs::action::ExecuteDance_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_SendGoal_Response msg_;
};

class Init_ExecuteDance_SendGoal_Response_accepted
{
public:
  Init_ExecuteDance_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ExecuteDance_SendGoal_Response_stamp accepted(::spot_msgs::action::ExecuteDance_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_ExecuteDance_SendGoal_Response_stamp(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::action::ExecuteDance_SendGoal_Response>()
{
  return spot_msgs::action::builder::Init_ExecuteDance_SendGoal_Response_accepted();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace action
{

namespace builder
{

class Init_ExecuteDance_GetResult_Request_goal_id
{
public:
  Init_ExecuteDance_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::spot_msgs::action::ExecuteDance_GetResult_Request goal_id(::spot_msgs::action::ExecuteDance_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::action::ExecuteDance_GetResult_Request>()
{
  return spot_msgs::action::builder::Init_ExecuteDance_GetResult_Request_goal_id();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace action
{

namespace builder
{

class Init_ExecuteDance_GetResult_Response_result
{
public:
  explicit Init_ExecuteDance_GetResult_Response_result(::spot_msgs::action::ExecuteDance_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::spot_msgs::action::ExecuteDance_GetResult_Response result(::spot_msgs::action::ExecuteDance_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_GetResult_Response msg_;
};

class Init_ExecuteDance_GetResult_Response_status
{
public:
  Init_ExecuteDance_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ExecuteDance_GetResult_Response_result status(::spot_msgs::action::ExecuteDance_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_ExecuteDance_GetResult_Response_result(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::action::ExecuteDance_GetResult_Response>()
{
  return spot_msgs::action::builder::Init_ExecuteDance_GetResult_Response_status();
}

}  // namespace spot_msgs


namespace spot_msgs
{

namespace action
{

namespace builder
{

class Init_ExecuteDance_FeedbackMessage_feedback
{
public:
  explicit Init_ExecuteDance_FeedbackMessage_feedback(::spot_msgs::action::ExecuteDance_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::spot_msgs::action::ExecuteDance_FeedbackMessage feedback(::spot_msgs::action::ExecuteDance_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_FeedbackMessage msg_;
};

class Init_ExecuteDance_FeedbackMessage_goal_id
{
public:
  Init_ExecuteDance_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ExecuteDance_FeedbackMessage_feedback goal_id(::spot_msgs::action::ExecuteDance_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_ExecuteDance_FeedbackMessage_feedback(msg_);
  }

private:
  ::spot_msgs::action::ExecuteDance_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::spot_msgs::action::ExecuteDance_FeedbackMessage>()
{
  return spot_msgs::action::builder::Init_ExecuteDance_FeedbackMessage_goal_id();
}

}  // namespace spot_msgs

#endif  // SPOT_MSGS__ACTION__DETAIL__EXECUTE_DANCE__BUILDER_HPP_
