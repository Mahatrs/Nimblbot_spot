// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from synchros2_tutorials_interfaces_example:action/Wait.idl
// generated code does not contain a copyright notice

#ifndef SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__ACTION__DETAIL__WAIT__BUILDER_HPP_
#define SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__ACTION__DETAIL__WAIT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "synchros2_tutorials_interfaces_example/action/detail/wait__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace synchros2_tutorials_interfaces_example
{

namespace action
{

namespace builder
{

class Init_Wait_Goal_n_seconds_to_wait
{
public:
  Init_Wait_Goal_n_seconds_to_wait()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::synchros2_tutorials_interfaces_example::action::Wait_Goal n_seconds_to_wait(::synchros2_tutorials_interfaces_example::action::Wait_Goal::_n_seconds_to_wait_type arg)
  {
    msg_.n_seconds_to_wait = std::move(arg);
    return std::move(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::synchros2_tutorials_interfaces_example::action::Wait_Goal>()
{
  return synchros2_tutorials_interfaces_example::action::builder::Init_Wait_Goal_n_seconds_to_wait();
}

}  // namespace synchros2_tutorials_interfaces_example


namespace synchros2_tutorials_interfaces_example
{

namespace action
{

namespace builder
{

class Init_Wait_Result_n_seconds_waited
{
public:
  Init_Wait_Result_n_seconds_waited()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::synchros2_tutorials_interfaces_example::action::Wait_Result n_seconds_waited(::synchros2_tutorials_interfaces_example::action::Wait_Result::_n_seconds_waited_type arg)
  {
    msg_.n_seconds_waited = std::move(arg);
    return std::move(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::synchros2_tutorials_interfaces_example::action::Wait_Result>()
{
  return synchros2_tutorials_interfaces_example::action::builder::Init_Wait_Result_n_seconds_waited();
}

}  // namespace synchros2_tutorials_interfaces_example


namespace synchros2_tutorials_interfaces_example
{

namespace action
{

namespace builder
{

class Init_Wait_Feedback_n_seconds_remaining
{
public:
  Init_Wait_Feedback_n_seconds_remaining()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::synchros2_tutorials_interfaces_example::action::Wait_Feedback n_seconds_remaining(::synchros2_tutorials_interfaces_example::action::Wait_Feedback::_n_seconds_remaining_type arg)
  {
    msg_.n_seconds_remaining = std::move(arg);
    return std::move(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_Feedback msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::synchros2_tutorials_interfaces_example::action::Wait_Feedback>()
{
  return synchros2_tutorials_interfaces_example::action::builder::Init_Wait_Feedback_n_seconds_remaining();
}

}  // namespace synchros2_tutorials_interfaces_example


namespace synchros2_tutorials_interfaces_example
{

namespace action
{

namespace builder
{

class Init_Wait_SendGoal_Request_goal
{
public:
  explicit Init_Wait_SendGoal_Request_goal(::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request goal(::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request msg_;
};

class Init_Wait_SendGoal_Request_goal_id
{
public:
  Init_Wait_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Wait_SendGoal_Request_goal goal_id(::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_Wait_SendGoal_Request_goal(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Request>()
{
  return synchros2_tutorials_interfaces_example::action::builder::Init_Wait_SendGoal_Request_goal_id();
}

}  // namespace synchros2_tutorials_interfaces_example


namespace synchros2_tutorials_interfaces_example
{

namespace action
{

namespace builder
{

class Init_Wait_SendGoal_Response_stamp
{
public:
  explicit Init_Wait_SendGoal_Response_stamp(::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response stamp(::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response msg_;
};

class Init_Wait_SendGoal_Response_accepted
{
public:
  Init_Wait_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Wait_SendGoal_Response_stamp accepted(::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_Wait_SendGoal_Response_stamp(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::synchros2_tutorials_interfaces_example::action::Wait_SendGoal_Response>()
{
  return synchros2_tutorials_interfaces_example::action::builder::Init_Wait_SendGoal_Response_accepted();
}

}  // namespace synchros2_tutorials_interfaces_example


namespace synchros2_tutorials_interfaces_example
{

namespace action
{

namespace builder
{

class Init_Wait_GetResult_Request_goal_id
{
public:
  Init_Wait_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request goal_id(::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Request>()
{
  return synchros2_tutorials_interfaces_example::action::builder::Init_Wait_GetResult_Request_goal_id();
}

}  // namespace synchros2_tutorials_interfaces_example


namespace synchros2_tutorials_interfaces_example
{

namespace action
{

namespace builder
{

class Init_Wait_GetResult_Response_result
{
public:
  explicit Init_Wait_GetResult_Response_result(::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response result(::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response msg_;
};

class Init_Wait_GetResult_Response_status
{
public:
  Init_Wait_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Wait_GetResult_Response_result status(::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_Wait_GetResult_Response_result(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::synchros2_tutorials_interfaces_example::action::Wait_GetResult_Response>()
{
  return synchros2_tutorials_interfaces_example::action::builder::Init_Wait_GetResult_Response_status();
}

}  // namespace synchros2_tutorials_interfaces_example


namespace synchros2_tutorials_interfaces_example
{

namespace action
{

namespace builder
{

class Init_Wait_FeedbackMessage_feedback
{
public:
  explicit Init_Wait_FeedbackMessage_feedback(::synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage feedback(::synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage msg_;
};

class Init_Wait_FeedbackMessage_goal_id
{
public:
  Init_Wait_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Wait_FeedbackMessage_feedback goal_id(::synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_Wait_FeedbackMessage_feedback(msg_);
  }

private:
  ::synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::synchros2_tutorials_interfaces_example::action::Wait_FeedbackMessage>()
{
  return synchros2_tutorials_interfaces_example::action::builder::Init_Wait_FeedbackMessage_goal_id();
}

}  // namespace synchros2_tutorials_interfaces_example

#endif  // SYNCHROS2_TUTORIALS_INTERFACES_EXAMPLE__ACTION__DETAIL__WAIT__BUILDER_HPP_
