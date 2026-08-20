
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_Goal() -> *const std::ffi::c_void;
}

#[link(name = "synchros2_tutorials_interfaces_example__rosidl_generator_c")]
extern "C" {
    fn synchros2_tutorials_interfaces_example__action__Wait_Goal__init(msg: *mut Wait_Goal) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Wait_Goal>, size: usize) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Wait_Goal>);
    fn synchros2_tutorials_interfaces_example__action__Wait_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Wait_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Wait_Goal>) -> bool;
}

// Corresponds to synchros2_tutorials_interfaces_example__action__Wait_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Wait_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub n_seconds_to_wait: f32,

}



impl Default for Wait_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !synchros2_tutorials_interfaces_example__action__Wait_Goal__init(&mut msg as *mut _) {
        panic!("Call to synchros2_tutorials_interfaces_example__action__Wait_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Wait_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Wait_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Wait_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "synchros2_tutorials_interfaces_example/action/Wait_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_Goal() }
  }
}


#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_Result() -> *const std::ffi::c_void;
}

#[link(name = "synchros2_tutorials_interfaces_example__rosidl_generator_c")]
extern "C" {
    fn synchros2_tutorials_interfaces_example__action__Wait_Result__init(msg: *mut Wait_Result) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Wait_Result>, size: usize) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Wait_Result>);
    fn synchros2_tutorials_interfaces_example__action__Wait_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Wait_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Wait_Result>) -> bool;
}

// Corresponds to synchros2_tutorials_interfaces_example__action__Wait_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Wait_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub n_seconds_waited: f32,

}



impl Default for Wait_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !synchros2_tutorials_interfaces_example__action__Wait_Result__init(&mut msg as *mut _) {
        panic!("Call to synchros2_tutorials_interfaces_example__action__Wait_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Wait_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Wait_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Wait_Result where Self: Sized {
  const TYPE_NAME: &'static str = "synchros2_tutorials_interfaces_example/action/Wait_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_Result() }
  }
}


#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "synchros2_tutorials_interfaces_example__rosidl_generator_c")]
extern "C" {
    fn synchros2_tutorials_interfaces_example__action__Wait_Feedback__init(msg: *mut Wait_Feedback) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Wait_Feedback>, size: usize) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Wait_Feedback>);
    fn synchros2_tutorials_interfaces_example__action__Wait_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Wait_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Wait_Feedback>) -> bool;
}

// Corresponds to synchros2_tutorials_interfaces_example__action__Wait_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Wait_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub n_seconds_remaining: f32,

}



impl Default for Wait_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !synchros2_tutorials_interfaces_example__action__Wait_Feedback__init(&mut msg as *mut _) {
        panic!("Call to synchros2_tutorials_interfaces_example__action__Wait_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Wait_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Wait_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Wait_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "synchros2_tutorials_interfaces_example/action/Wait_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_Feedback() }
  }
}


#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "synchros2_tutorials_interfaces_example__rosidl_generator_c")]
extern "C" {
    fn synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__init(msg: *mut Wait_FeedbackMessage) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Wait_FeedbackMessage>, size: usize) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Wait_FeedbackMessage>);
    fn synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Wait_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Wait_FeedbackMessage>) -> bool;
}

// Corresponds to synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Wait_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Wait_Feedback,

}



impl Default for Wait_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Wait_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Wait_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Wait_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "synchros2_tutorials_interfaces_example/action/Wait_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_FeedbackMessage() }
  }
}




#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "synchros2_tutorials_interfaces_example__rosidl_generator_c")]
extern "C" {
    fn synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__init(msg: *mut Wait_SendGoal_Request) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Wait_SendGoal_Request>, size: usize) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Wait_SendGoal_Request>);
    fn synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Wait_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Wait_SendGoal_Request>) -> bool;
}

// Corresponds to synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Wait_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Wait_Goal,

}



impl Default for Wait_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Wait_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Wait_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Wait_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "synchros2_tutorials_interfaces_example/action/Wait_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Request() }
  }
}


#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "synchros2_tutorials_interfaces_example__rosidl_generator_c")]
extern "C" {
    fn synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__init(msg: *mut Wait_SendGoal_Response) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Wait_SendGoal_Response>, size: usize) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Wait_SendGoal_Response>);
    fn synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Wait_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Wait_SendGoal_Response>) -> bool;
}

// Corresponds to synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Wait_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Wait_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Wait_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Wait_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Wait_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "synchros2_tutorials_interfaces_example/action/Wait_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_SendGoal_Response() }
  }
}


#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "synchros2_tutorials_interfaces_example__rosidl_generator_c")]
extern "C" {
    fn synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__init(msg: *mut Wait_GetResult_Request) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Wait_GetResult_Request>, size: usize) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Wait_GetResult_Request>);
    fn synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Wait_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Wait_GetResult_Request>) -> bool;
}

// Corresponds to synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Wait_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Wait_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Wait_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Wait_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Wait_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "synchros2_tutorials_interfaces_example/action/Wait_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_GetResult_Request() }
  }
}


#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "synchros2_tutorials_interfaces_example__rosidl_generator_c")]
extern "C" {
    fn synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__init(msg: *mut Wait_GetResult_Response) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Wait_GetResult_Response>, size: usize) -> bool;
    fn synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Wait_GetResult_Response>);
    fn synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Wait_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Wait_GetResult_Response>) -> bool;
}

// Corresponds to synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Wait_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Wait_Result,

}



impl Default for Wait_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Wait_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Wait_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Wait_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "synchros2_tutorials_interfaces_example/action/Wait_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_GetResult_Response() }
  }
}






#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to synchros2_tutorials_interfaces_example__action__Wait_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Wait_SendGoal;

impl rosidl_runtime_rs::Service for Wait_SendGoal {
    type Request = Wait_SendGoal_Request;
    type Response = Wait_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_SendGoal() }
    }
}




#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to synchros2_tutorials_interfaces_example__action__Wait_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Wait_GetResult;

impl rosidl_runtime_rs::Service for Wait_GetResult {
    type Request = Wait_GetResult_Request;
    type Response = Wait_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__synchros2_tutorials_interfaces_example__action__Wait_GetResult() }
    }
}


