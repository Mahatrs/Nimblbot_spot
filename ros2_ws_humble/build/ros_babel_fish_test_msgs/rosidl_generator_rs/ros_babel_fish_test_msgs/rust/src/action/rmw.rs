
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_Goal() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__action__SimpleTest_Goal__init(msg: *mut SimpleTest_Goal) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_Goal>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_Goal>);
    fn ros_babel_fish_test_msgs__action__SimpleTest_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SimpleTest_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_Goal>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__action__SimpleTest_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SimpleTest_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target: i32,

}



impl Default for SimpleTest_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__action__SimpleTest_Goal__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__action__SimpleTest_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SimpleTest_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SimpleTest_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SimpleTest_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/action/SimpleTest_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_Goal() }
  }
}


#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_Result() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__action__SimpleTest_Result__init(msg: *mut SimpleTest_Result) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_Result>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_Result>);
    fn ros_babel_fish_test_msgs__action__SimpleTest_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SimpleTest_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_Result>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__action__SimpleTest_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SimpleTest_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub final_value: i32,

}



impl Default for SimpleTest_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__action__SimpleTest_Result__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__action__SimpleTest_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SimpleTest_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SimpleTest_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SimpleTest_Result where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/action/SimpleTest_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_Result() }
  }
}


#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__action__SimpleTest_Feedback__init(msg: *mut SimpleTest_Feedback) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_Feedback>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_Feedback>);
    fn ros_babel_fish_test_msgs__action__SimpleTest_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SimpleTest_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_Feedback>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__action__SimpleTest_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SimpleTest_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_value: i32,

}



impl Default for SimpleTest_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__action__SimpleTest_Feedback__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__action__SimpleTest_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SimpleTest_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SimpleTest_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SimpleTest_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/action/SimpleTest_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_Feedback() }
  }
}


#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage__init(msg: *mut SimpleTest_FeedbackMessage) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_FeedbackMessage>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_FeedbackMessage>);
    fn ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SimpleTest_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_FeedbackMessage>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SimpleTest_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::SimpleTest_Feedback,

}



impl Default for SimpleTest_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SimpleTest_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SimpleTest_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SimpleTest_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/action/SimpleTest_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_FeedbackMessage() }
  }
}




#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request__init(msg: *mut SimpleTest_SendGoal_Request) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_SendGoal_Request>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_SendGoal_Request>);
    fn ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SimpleTest_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_SendGoal_Request>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SimpleTest_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::SimpleTest_Goal,

}



impl Default for SimpleTest_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SimpleTest_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SimpleTest_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SimpleTest_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/action/SimpleTest_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Request() }
  }
}


#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response__init(msg: *mut SimpleTest_SendGoal_Response) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_SendGoal_Response>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_SendGoal_Response>);
    fn ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SimpleTest_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_SendGoal_Response>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SimpleTest_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for SimpleTest_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SimpleTest_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SimpleTest_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SimpleTest_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/action/SimpleTest_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_SendGoal_Response() }
  }
}


#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request__init(msg: *mut SimpleTest_GetResult_Request) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_GetResult_Request>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_GetResult_Request>);
    fn ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SimpleTest_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_GetResult_Request>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SimpleTest_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for SimpleTest_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SimpleTest_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SimpleTest_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SimpleTest_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/action/SimpleTest_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Request() }
  }
}


#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response__init(msg: *mut SimpleTest_GetResult_Response) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_GetResult_Response>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_GetResult_Response>);
    fn ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SimpleTest_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SimpleTest_GetResult_Response>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SimpleTest_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::SimpleTest_Result,

}



impl Default for SimpleTest_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SimpleTest_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SimpleTest_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SimpleTest_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/action/SimpleTest_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_GetResult_Response() }
  }
}






#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to ros_babel_fish_test_msgs__action__SimpleTest_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct SimpleTest_SendGoal;

impl rosidl_runtime_rs::Service for SimpleTest_SendGoal {
    type Request = SimpleTest_SendGoal_Request;
    type Response = SimpleTest_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_SendGoal() }
    }
}




#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to ros_babel_fish_test_msgs__action__SimpleTest_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct SimpleTest_GetResult;

impl rosidl_runtime_rs::Service for SimpleTest_GetResult {
    type Request = SimpleTest_GetResult_Request;
    type Response = SimpleTest_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ros_babel_fish_test_msgs__action__SimpleTest_GetResult() }
    }
}


