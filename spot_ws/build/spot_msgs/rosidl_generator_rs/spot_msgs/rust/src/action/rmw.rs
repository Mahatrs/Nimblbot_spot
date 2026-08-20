
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_Goal() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ArmSurfaceContact_Goal__init(msg: *mut ArmSurfaceContact_Goal) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_Goal>, size: usize) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_Goal>);
    fn spot_msgs__action__ArmSurfaceContact_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmSurfaceContact_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_Goal>) -> bool;
}

// Corresponds to spot_msgs__action__ArmSurfaceContact_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmSurfaceContact_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: bosdyn_api_msgs::msg::rmw::ArmSurfaceContactRequest,

}



impl Default for ArmSurfaceContact_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ArmSurfaceContact_Goal__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ArmSurfaceContact_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmSurfaceContact_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmSurfaceContact_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmSurfaceContact_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ArmSurfaceContact_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_Goal() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_Result() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ArmSurfaceContact_Result__init(msg: *mut ArmSurfaceContact_Result) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_Result>, size: usize) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_Result>);
    fn spot_msgs__action__ArmSurfaceContact_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmSurfaceContact_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_Result>) -> bool;
}

// Corresponds to spot_msgs__action__ArmSurfaceContact_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmSurfaceContact_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for ArmSurfaceContact_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ArmSurfaceContact_Result__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ArmSurfaceContact_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmSurfaceContact_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmSurfaceContact_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmSurfaceContact_Result where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ArmSurfaceContact_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_Result() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ArmSurfaceContact_Feedback__init(msg: *mut ArmSurfaceContact_Feedback) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_Feedback>, size: usize) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_Feedback>);
    fn spot_msgs__action__ArmSurfaceContact_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmSurfaceContact_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_Feedback>) -> bool;
}

// Corresponds to spot_msgs__action__ArmSurfaceContact_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmSurfaceContact_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ArmSurfaceContact_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ArmSurfaceContact_Feedback__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ArmSurfaceContact_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmSurfaceContact_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmSurfaceContact_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmSurfaceContact_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ArmSurfaceContact_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_Feedback() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ArmSurfaceContact_FeedbackMessage__init(msg: *mut ArmSurfaceContact_FeedbackMessage) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_FeedbackMessage>, size: usize) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_FeedbackMessage>);
    fn spot_msgs__action__ArmSurfaceContact_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmSurfaceContact_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_FeedbackMessage>) -> bool;
}

// Corresponds to spot_msgs__action__ArmSurfaceContact_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmSurfaceContact_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::ArmSurfaceContact_Feedback,

}



impl Default for ArmSurfaceContact_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ArmSurfaceContact_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ArmSurfaceContact_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmSurfaceContact_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmSurfaceContact_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmSurfaceContact_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ArmSurfaceContact_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_FeedbackMessage() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_Goal() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ExecuteDance_Goal__init(msg: *mut ExecuteDance_Goal) -> bool;
    fn spot_msgs__action__ExecuteDance_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_Goal>, size: usize) -> bool;
    fn spot_msgs__action__ExecuteDance_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_Goal>);
    fn spot_msgs__action__ExecuteDance_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteDance_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_Goal>) -> bool;
}

// Corresponds to spot_msgs__action__ExecuteDance_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteDance_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub choreo_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub choreo_file_content: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub choreo_sequence_serialized: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub start_slice: u32,

}



impl Default for ExecuteDance_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ExecuteDance_Goal__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ExecuteDance_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteDance_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteDance_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteDance_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ExecuteDance_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_Goal() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_Result() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ExecuteDance_Result__init(msg: *mut ExecuteDance_Result) -> bool;
    fn spot_msgs__action__ExecuteDance_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_Result>, size: usize) -> bool;
    fn spot_msgs__action__ExecuteDance_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_Result>);
    fn spot_msgs__action__ExecuteDance_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteDance_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_Result>) -> bool;
}

// Corresponds to spot_msgs__action__ExecuteDance_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteDance_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for ExecuteDance_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ExecuteDance_Result__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ExecuteDance_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteDance_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteDance_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteDance_Result where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ExecuteDance_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_Result() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ExecuteDance_Feedback__init(msg: *mut ExecuteDance_Feedback) -> bool;
    fn spot_msgs__action__ExecuteDance_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_Feedback>, size: usize) -> bool;
    fn spot_msgs__action__ExecuteDance_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_Feedback>);
    fn spot_msgs__action__ExecuteDance_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteDance_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_Feedback>) -> bool;
}

// Corresponds to spot_msgs__action__ExecuteDance_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteDance_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_dancing: bool,

}



impl Default for ExecuteDance_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ExecuteDance_Feedback__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ExecuteDance_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteDance_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteDance_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteDance_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ExecuteDance_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_Feedback() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ExecuteDance_FeedbackMessage__init(msg: *mut ExecuteDance_FeedbackMessage) -> bool;
    fn spot_msgs__action__ExecuteDance_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_FeedbackMessage>, size: usize) -> bool;
    fn spot_msgs__action__ExecuteDance_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_FeedbackMessage>);
    fn spot_msgs__action__ExecuteDance_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteDance_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_FeedbackMessage>) -> bool;
}

// Corresponds to spot_msgs__action__ExecuteDance_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteDance_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::ExecuteDance_Feedback,

}



impl Default for ExecuteDance_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ExecuteDance_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ExecuteDance_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteDance_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteDance_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteDance_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ExecuteDance_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_FeedbackMessage() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_Goal() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__NavigateTo_Goal__init(msg: *mut NavigateTo_Goal) -> bool;
    fn spot_msgs__action__NavigateTo_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_Goal>, size: usize) -> bool;
    fn spot_msgs__action__NavigateTo_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_Goal>);
    fn spot_msgs__action__NavigateTo_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateTo_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_Goal>) -> bool;
}

// Corresponds to spot_msgs__action__NavigateTo_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateTo_Goal {
    /// waypoint ID to navigate to
    pub waypoint_id: rosidl_runtime_rs::String,

}



impl Default for NavigateTo_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__NavigateTo_Goal__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__NavigateTo_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateTo_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateTo_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateTo_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/NavigateTo_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_Goal() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_Result() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__NavigateTo_Result__init(msg: *mut NavigateTo_Result) -> bool;
    fn spot_msgs__action__NavigateTo_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_Result>, size: usize) -> bool;
    fn spot_msgs__action__NavigateTo_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_Result>);
    fn spot_msgs__action__NavigateTo_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateTo_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_Result>) -> bool;
}

// Corresponds to spot_msgs__action__NavigateTo_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateTo_Result {
    /// indicate successful run of triggered service
    pub success: bool,

    /// informational, e.g. for error messages
    pub message: rosidl_runtime_rs::String,

}



impl Default for NavigateTo_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__NavigateTo_Result__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__NavigateTo_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateTo_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateTo_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateTo_Result where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/NavigateTo_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_Result() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__NavigateTo_Feedback__init(msg: *mut NavigateTo_Feedback) -> bool;
    fn spot_msgs__action__NavigateTo_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_Feedback>, size: usize) -> bool;
    fn spot_msgs__action__NavigateTo_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_Feedback>);
    fn spot_msgs__action__NavigateTo_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateTo_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_Feedback>) -> bool;
}

// Corresponds to spot_msgs__action__NavigateTo_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateTo_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub waypoint_id: rosidl_runtime_rs::String,

}



impl Default for NavigateTo_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__NavigateTo_Feedback__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__NavigateTo_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateTo_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateTo_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateTo_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/NavigateTo_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_Feedback() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__NavigateTo_FeedbackMessage__init(msg: *mut NavigateTo_FeedbackMessage) -> bool;
    fn spot_msgs__action__NavigateTo_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_FeedbackMessage>, size: usize) -> bool;
    fn spot_msgs__action__NavigateTo_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_FeedbackMessage>);
    fn spot_msgs__action__NavigateTo_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateTo_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_FeedbackMessage>) -> bool;
}

// Corresponds to spot_msgs__action__NavigateTo_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateTo_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::NavigateTo_Feedback,

}



impl Default for NavigateTo_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__NavigateTo_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__NavigateTo_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateTo_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateTo_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateTo_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/NavigateTo_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_FeedbackMessage() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_Goal() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__RobotCommand_Goal__init(msg: *mut RobotCommand_Goal) -> bool;
    fn spot_msgs__action__RobotCommand_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Goal>, size: usize) -> bool;
    fn spot_msgs__action__RobotCommand_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Goal>);
    fn spot_msgs__action__RobotCommand_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotCommand_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Goal>) -> bool;
}

// Corresponds to spot_msgs__action__RobotCommand_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: bosdyn_api_msgs::msg::rmw::RobotCommand,

}



impl Default for RobotCommand_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__RobotCommand_Goal__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__RobotCommand_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotCommand_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotCommand_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/RobotCommand_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_Goal() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_Result() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__RobotCommand_Result__init(msg: *mut RobotCommand_Result) -> bool;
    fn spot_msgs__action__RobotCommand_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Result>, size: usize) -> bool;
    fn spot_msgs__action__RobotCommand_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Result>);
    fn spot_msgs__action__RobotCommand_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotCommand_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Result>) -> bool;
}

// Corresponds to spot_msgs__action__RobotCommand_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bosdyn_api_msgs::msg::rmw::RobotCommandFeedback,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for RobotCommand_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__RobotCommand_Result__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__RobotCommand_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotCommand_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotCommand_Result where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/RobotCommand_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_Result() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__RobotCommand_Feedback__init(msg: *mut RobotCommand_Feedback) -> bool;
    fn spot_msgs__action__RobotCommand_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Feedback>, size: usize) -> bool;
    fn spot_msgs__action__RobotCommand_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Feedback>);
    fn spot_msgs__action__RobotCommand_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotCommand_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Feedback>) -> bool;
}

// Corresponds to spot_msgs__action__RobotCommand_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: bosdyn_api_msgs::msg::rmw::RobotCommandFeedback,

}



impl Default for RobotCommand_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__RobotCommand_Feedback__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__RobotCommand_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotCommand_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotCommand_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/RobotCommand_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_Feedback() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__RobotCommand_FeedbackMessage__init(msg: *mut RobotCommand_FeedbackMessage) -> bool;
    fn spot_msgs__action__RobotCommand_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_FeedbackMessage>, size: usize) -> bool;
    fn spot_msgs__action__RobotCommand_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_FeedbackMessage>);
    fn spot_msgs__action__RobotCommand_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotCommand_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_FeedbackMessage>) -> bool;
}

// Corresponds to spot_msgs__action__RobotCommand_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::RobotCommand_Feedback,

}



impl Default for RobotCommand_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__RobotCommand_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__RobotCommand_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotCommand_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotCommand_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/RobotCommand_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_FeedbackMessage() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_Goal() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Trajectory_Goal__init(msg: *mut Trajectory_Goal) -> bool;
    fn spot_msgs__action__Trajectory_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_Goal>, size: usize) -> bool;
    fn spot_msgs__action__Trajectory_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_Goal>);
    fn spot_msgs__action__Trajectory_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Trajectory_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Trajectory_Goal>) -> bool;
}

// Corresponds to spot_msgs__action__Trajectory_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trajectory_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target_pose: geometry_msgs::msg::rmw::PoseStamped,

    /// After this duration, the command will time out and the robot will stop. Must be non-zero
    pub duration: builtin_interfaces::msg::rmw::Duration,

    /// If true, the feedback from the trajectory command must indicate that the robot is
    /// at the goal position. If set to false, the robot being near the goal is equivalent to
    /// it being at the goal. This is based on the feedback received from the boston dynamics
    /// API call at
    /// https://dev.bostondynamics.com/protos/bosdyn/api/proto_reference.html?highlight=status_near_goal#se2trajectorycommand-feedback-status
    pub precise_positioning: bool,

    /// If true, turns off the vision body obstacle avoidance in mobility params
    pub disable_obstacle_avoidance: bool,

}



impl Default for Trajectory_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Trajectory_Goal__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Trajectory_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Trajectory_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Trajectory_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Trajectory_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Trajectory_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_Goal() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_Result() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Trajectory_Result__init(msg: *mut Trajectory_Result) -> bool;
    fn spot_msgs__action__Trajectory_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_Result>, size: usize) -> bool;
    fn spot_msgs__action__Trajectory_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_Result>);
    fn spot_msgs__action__Trajectory_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Trajectory_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Trajectory_Result>) -> bool;
}

// Corresponds to spot_msgs__action__Trajectory_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trajectory_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for Trajectory_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Trajectory_Result__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Trajectory_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Trajectory_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Trajectory_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Trajectory_Result where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Trajectory_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_Result() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Trajectory_Feedback__init(msg: *mut Trajectory_Feedback) -> bool;
    fn spot_msgs__action__Trajectory_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_Feedback>, size: usize) -> bool;
    fn spot_msgs__action__Trajectory_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_Feedback>);
    fn spot_msgs__action__Trajectory_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Trajectory_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Trajectory_Feedback>) -> bool;
}

// Corresponds to spot_msgs__action__Trajectory_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trajectory_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: rosidl_runtime_rs::String,

}



impl Default for Trajectory_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Trajectory_Feedback__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Trajectory_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Trajectory_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Trajectory_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Trajectory_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Trajectory_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_Feedback() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Trajectory_FeedbackMessage__init(msg: *mut Trajectory_FeedbackMessage) -> bool;
    fn spot_msgs__action__Trajectory_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_FeedbackMessage>, size: usize) -> bool;
    fn spot_msgs__action__Trajectory_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_FeedbackMessage>);
    fn spot_msgs__action__Trajectory_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Trajectory_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Trajectory_FeedbackMessage>) -> bool;
}

// Corresponds to spot_msgs__action__Trajectory_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trajectory_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Trajectory_Feedback,

}



impl Default for Trajectory_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Trajectory_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Trajectory_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Trajectory_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Trajectory_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Trajectory_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Trajectory_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_FeedbackMessage() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_Goal() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Manipulation_Goal__init(msg: *mut Manipulation_Goal) -> bool;
    fn spot_msgs__action__Manipulation_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_Goal>, size: usize) -> bool;
    fn spot_msgs__action__Manipulation_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_Goal>);
    fn spot_msgs__action__Manipulation_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Manipulation_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Manipulation_Goal>) -> bool;
}

// Corresponds to spot_msgs__action__Manipulation_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Manipulation_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: bosdyn_api_msgs::msg::rmw::ManipulationApiRequest,

}



impl Default for Manipulation_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Manipulation_Goal__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Manipulation_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Manipulation_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Manipulation_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Manipulation_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Manipulation_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_Goal() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_Result() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Manipulation_Result__init(msg: *mut Manipulation_Result) -> bool;
    fn spot_msgs__action__Manipulation_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_Result>, size: usize) -> bool;
    fn spot_msgs__action__Manipulation_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_Result>);
    fn spot_msgs__action__Manipulation_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Manipulation_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Manipulation_Result>) -> bool;
}

// Corresponds to spot_msgs__action__Manipulation_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Manipulation_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bosdyn_api_msgs::msg::rmw::ManipulationApiFeedbackResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for Manipulation_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Manipulation_Result__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Manipulation_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Manipulation_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Manipulation_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Manipulation_Result where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Manipulation_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_Result() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Manipulation_Feedback__init(msg: *mut Manipulation_Feedback) -> bool;
    fn spot_msgs__action__Manipulation_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_Feedback>, size: usize) -> bool;
    fn spot_msgs__action__Manipulation_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_Feedback>);
    fn spot_msgs__action__Manipulation_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Manipulation_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Manipulation_Feedback>) -> bool;
}

// Corresponds to spot_msgs__action__Manipulation_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Manipulation_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: bosdyn_api_msgs::msg::rmw::ManipulationApiFeedbackResponse,

}



impl Default for Manipulation_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Manipulation_Feedback__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Manipulation_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Manipulation_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Manipulation_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Manipulation_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Manipulation_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_Feedback() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Manipulation_FeedbackMessage__init(msg: *mut Manipulation_FeedbackMessage) -> bool;
    fn spot_msgs__action__Manipulation_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_FeedbackMessage>, size: usize) -> bool;
    fn spot_msgs__action__Manipulation_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_FeedbackMessage>);
    fn spot_msgs__action__Manipulation_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Manipulation_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Manipulation_FeedbackMessage>) -> bool;
}

// Corresponds to spot_msgs__action__Manipulation_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Manipulation_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Manipulation_Feedback,

}



impl Default for Manipulation_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Manipulation_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Manipulation_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Manipulation_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Manipulation_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Manipulation_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Manipulation_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_FeedbackMessage() }
  }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ArmSurfaceContact_SendGoal_Request__init(msg: *mut ArmSurfaceContact_SendGoal_Request) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_SendGoal_Request>, size: usize) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_SendGoal_Request>);
    fn spot_msgs__action__ArmSurfaceContact_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmSurfaceContact_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_SendGoal_Request>) -> bool;
}

// Corresponds to spot_msgs__action__ArmSurfaceContact_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmSurfaceContact_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::ArmSurfaceContact_Goal,

}



impl Default for ArmSurfaceContact_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ArmSurfaceContact_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ArmSurfaceContact_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmSurfaceContact_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmSurfaceContact_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmSurfaceContact_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ArmSurfaceContact_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_SendGoal_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ArmSurfaceContact_SendGoal_Response__init(msg: *mut ArmSurfaceContact_SendGoal_Response) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_SendGoal_Response>, size: usize) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_SendGoal_Response>);
    fn spot_msgs__action__ArmSurfaceContact_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmSurfaceContact_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_SendGoal_Response>) -> bool;
}

// Corresponds to spot_msgs__action__ArmSurfaceContact_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmSurfaceContact_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for ArmSurfaceContact_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ArmSurfaceContact_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ArmSurfaceContact_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmSurfaceContact_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmSurfaceContact_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmSurfaceContact_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ArmSurfaceContact_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_SendGoal_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ArmSurfaceContact_GetResult_Request__init(msg: *mut ArmSurfaceContact_GetResult_Request) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_GetResult_Request>, size: usize) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_GetResult_Request>);
    fn spot_msgs__action__ArmSurfaceContact_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmSurfaceContact_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_GetResult_Request>) -> bool;
}

// Corresponds to spot_msgs__action__ArmSurfaceContact_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmSurfaceContact_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for ArmSurfaceContact_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ArmSurfaceContact_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ArmSurfaceContact_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmSurfaceContact_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmSurfaceContact_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmSurfaceContact_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ArmSurfaceContact_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_GetResult_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ArmSurfaceContact_GetResult_Response__init(msg: *mut ArmSurfaceContact_GetResult_Response) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_GetResult_Response>, size: usize) -> bool;
    fn spot_msgs__action__ArmSurfaceContact_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_GetResult_Response>);
    fn spot_msgs__action__ArmSurfaceContact_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmSurfaceContact_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmSurfaceContact_GetResult_Response>) -> bool;
}

// Corresponds to spot_msgs__action__ArmSurfaceContact_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmSurfaceContact_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::ArmSurfaceContact_Result,

}



impl Default for ArmSurfaceContact_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ArmSurfaceContact_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ArmSurfaceContact_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmSurfaceContact_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ArmSurfaceContact_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmSurfaceContact_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmSurfaceContact_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ArmSurfaceContact_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ArmSurfaceContact_GetResult_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ExecuteDance_SendGoal_Request__init(msg: *mut ExecuteDance_SendGoal_Request) -> bool;
    fn spot_msgs__action__ExecuteDance_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_SendGoal_Request>, size: usize) -> bool;
    fn spot_msgs__action__ExecuteDance_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_SendGoal_Request>);
    fn spot_msgs__action__ExecuteDance_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteDance_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_SendGoal_Request>) -> bool;
}

// Corresponds to spot_msgs__action__ExecuteDance_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteDance_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::ExecuteDance_Goal,

}



impl Default for ExecuteDance_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ExecuteDance_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ExecuteDance_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteDance_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteDance_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteDance_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ExecuteDance_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_SendGoal_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ExecuteDance_SendGoal_Response__init(msg: *mut ExecuteDance_SendGoal_Response) -> bool;
    fn spot_msgs__action__ExecuteDance_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_SendGoal_Response>, size: usize) -> bool;
    fn spot_msgs__action__ExecuteDance_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_SendGoal_Response>);
    fn spot_msgs__action__ExecuteDance_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteDance_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_SendGoal_Response>) -> bool;
}

// Corresponds to spot_msgs__action__ExecuteDance_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteDance_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for ExecuteDance_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ExecuteDance_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ExecuteDance_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteDance_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteDance_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteDance_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ExecuteDance_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_SendGoal_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ExecuteDance_GetResult_Request__init(msg: *mut ExecuteDance_GetResult_Request) -> bool;
    fn spot_msgs__action__ExecuteDance_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_GetResult_Request>, size: usize) -> bool;
    fn spot_msgs__action__ExecuteDance_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_GetResult_Request>);
    fn spot_msgs__action__ExecuteDance_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteDance_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_GetResult_Request>) -> bool;
}

// Corresponds to spot_msgs__action__ExecuteDance_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteDance_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for ExecuteDance_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ExecuteDance_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ExecuteDance_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteDance_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteDance_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteDance_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ExecuteDance_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_GetResult_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__ExecuteDance_GetResult_Response__init(msg: *mut ExecuteDance_GetResult_Response) -> bool;
    fn spot_msgs__action__ExecuteDance_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_GetResult_Response>, size: usize) -> bool;
    fn spot_msgs__action__ExecuteDance_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_GetResult_Response>);
    fn spot_msgs__action__ExecuteDance_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteDance_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteDance_GetResult_Response>) -> bool;
}

// Corresponds to spot_msgs__action__ExecuteDance_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteDance_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::ExecuteDance_Result,

}



impl Default for ExecuteDance_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__ExecuteDance_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__ExecuteDance_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteDance_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__ExecuteDance_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteDance_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteDance_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/ExecuteDance_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__ExecuteDance_GetResult_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__NavigateTo_SendGoal_Request__init(msg: *mut NavigateTo_SendGoal_Request) -> bool;
    fn spot_msgs__action__NavigateTo_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_SendGoal_Request>, size: usize) -> bool;
    fn spot_msgs__action__NavigateTo_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_SendGoal_Request>);
    fn spot_msgs__action__NavigateTo_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateTo_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_SendGoal_Request>) -> bool;
}

// Corresponds to spot_msgs__action__NavigateTo_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateTo_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::NavigateTo_Goal,

}



impl Default for NavigateTo_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__NavigateTo_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__NavigateTo_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateTo_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateTo_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateTo_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/NavigateTo_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_SendGoal_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__NavigateTo_SendGoal_Response__init(msg: *mut NavigateTo_SendGoal_Response) -> bool;
    fn spot_msgs__action__NavigateTo_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_SendGoal_Response>, size: usize) -> bool;
    fn spot_msgs__action__NavigateTo_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_SendGoal_Response>);
    fn spot_msgs__action__NavigateTo_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateTo_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_SendGoal_Response>) -> bool;
}

// Corresponds to spot_msgs__action__NavigateTo_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateTo_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for NavigateTo_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__NavigateTo_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__NavigateTo_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateTo_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateTo_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateTo_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/NavigateTo_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_SendGoal_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__NavigateTo_GetResult_Request__init(msg: *mut NavigateTo_GetResult_Request) -> bool;
    fn spot_msgs__action__NavigateTo_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_GetResult_Request>, size: usize) -> bool;
    fn spot_msgs__action__NavigateTo_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_GetResult_Request>);
    fn spot_msgs__action__NavigateTo_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateTo_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_GetResult_Request>) -> bool;
}

// Corresponds to spot_msgs__action__NavigateTo_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateTo_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for NavigateTo_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__NavigateTo_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__NavigateTo_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateTo_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateTo_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateTo_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/NavigateTo_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_GetResult_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__NavigateTo_GetResult_Response__init(msg: *mut NavigateTo_GetResult_Response) -> bool;
    fn spot_msgs__action__NavigateTo_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_GetResult_Response>, size: usize) -> bool;
    fn spot_msgs__action__NavigateTo_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_GetResult_Response>);
    fn spot_msgs__action__NavigateTo_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateTo_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateTo_GetResult_Response>) -> bool;
}

// Corresponds to spot_msgs__action__NavigateTo_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateTo_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::NavigateTo_Result,

}



impl Default for NavigateTo_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__NavigateTo_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__NavigateTo_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateTo_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__NavigateTo_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateTo_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateTo_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/NavigateTo_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__NavigateTo_GetResult_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__RobotCommand_SendGoal_Request__init(msg: *mut RobotCommand_SendGoal_Request) -> bool;
    fn spot_msgs__action__RobotCommand_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_SendGoal_Request>, size: usize) -> bool;
    fn spot_msgs__action__RobotCommand_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_SendGoal_Request>);
    fn spot_msgs__action__RobotCommand_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotCommand_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_SendGoal_Request>) -> bool;
}

// Corresponds to spot_msgs__action__RobotCommand_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::RobotCommand_Goal,

}



impl Default for RobotCommand_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__RobotCommand_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__RobotCommand_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotCommand_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotCommand_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/RobotCommand_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_SendGoal_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__RobotCommand_SendGoal_Response__init(msg: *mut RobotCommand_SendGoal_Response) -> bool;
    fn spot_msgs__action__RobotCommand_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_SendGoal_Response>, size: usize) -> bool;
    fn spot_msgs__action__RobotCommand_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_SendGoal_Response>);
    fn spot_msgs__action__RobotCommand_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotCommand_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_SendGoal_Response>) -> bool;
}

// Corresponds to spot_msgs__action__RobotCommand_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for RobotCommand_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__RobotCommand_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__RobotCommand_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotCommand_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotCommand_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/RobotCommand_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_SendGoal_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__RobotCommand_GetResult_Request__init(msg: *mut RobotCommand_GetResult_Request) -> bool;
    fn spot_msgs__action__RobotCommand_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_GetResult_Request>, size: usize) -> bool;
    fn spot_msgs__action__RobotCommand_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_GetResult_Request>);
    fn spot_msgs__action__RobotCommand_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotCommand_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_GetResult_Request>) -> bool;
}

// Corresponds to spot_msgs__action__RobotCommand_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for RobotCommand_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__RobotCommand_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__RobotCommand_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotCommand_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotCommand_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/RobotCommand_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_GetResult_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__RobotCommand_GetResult_Response__init(msg: *mut RobotCommand_GetResult_Response) -> bool;
    fn spot_msgs__action__RobotCommand_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_GetResult_Response>, size: usize) -> bool;
    fn spot_msgs__action__RobotCommand_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_GetResult_Response>);
    fn spot_msgs__action__RobotCommand_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotCommand_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_GetResult_Response>) -> bool;
}

// Corresponds to spot_msgs__action__RobotCommand_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::RobotCommand_Result,

}



impl Default for RobotCommand_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__RobotCommand_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__RobotCommand_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotCommand_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__RobotCommand_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotCommand_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/RobotCommand_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__RobotCommand_GetResult_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Trajectory_SendGoal_Request__init(msg: *mut Trajectory_SendGoal_Request) -> bool;
    fn spot_msgs__action__Trajectory_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_SendGoal_Request>, size: usize) -> bool;
    fn spot_msgs__action__Trajectory_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_SendGoal_Request>);
    fn spot_msgs__action__Trajectory_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Trajectory_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Trajectory_SendGoal_Request>) -> bool;
}

// Corresponds to spot_msgs__action__Trajectory_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trajectory_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Trajectory_Goal,

}



impl Default for Trajectory_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Trajectory_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Trajectory_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Trajectory_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Trajectory_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Trajectory_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Trajectory_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_SendGoal_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Trajectory_SendGoal_Response__init(msg: *mut Trajectory_SendGoal_Response) -> bool;
    fn spot_msgs__action__Trajectory_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_SendGoal_Response>, size: usize) -> bool;
    fn spot_msgs__action__Trajectory_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_SendGoal_Response>);
    fn spot_msgs__action__Trajectory_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Trajectory_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Trajectory_SendGoal_Response>) -> bool;
}

// Corresponds to spot_msgs__action__Trajectory_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trajectory_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Trajectory_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Trajectory_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Trajectory_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Trajectory_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Trajectory_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Trajectory_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Trajectory_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_SendGoal_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Trajectory_GetResult_Request__init(msg: *mut Trajectory_GetResult_Request) -> bool;
    fn spot_msgs__action__Trajectory_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_GetResult_Request>, size: usize) -> bool;
    fn spot_msgs__action__Trajectory_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_GetResult_Request>);
    fn spot_msgs__action__Trajectory_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Trajectory_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Trajectory_GetResult_Request>) -> bool;
}

// Corresponds to spot_msgs__action__Trajectory_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trajectory_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Trajectory_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Trajectory_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Trajectory_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Trajectory_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Trajectory_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Trajectory_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Trajectory_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_GetResult_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Trajectory_GetResult_Response__init(msg: *mut Trajectory_GetResult_Response) -> bool;
    fn spot_msgs__action__Trajectory_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_GetResult_Response>, size: usize) -> bool;
    fn spot_msgs__action__Trajectory_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Trajectory_GetResult_Response>);
    fn spot_msgs__action__Trajectory_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Trajectory_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Trajectory_GetResult_Response>) -> bool;
}

// Corresponds to spot_msgs__action__Trajectory_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trajectory_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Trajectory_Result,

}



impl Default for Trajectory_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Trajectory_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Trajectory_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Trajectory_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Trajectory_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Trajectory_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Trajectory_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Trajectory_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Trajectory_GetResult_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Manipulation_SendGoal_Request__init(msg: *mut Manipulation_SendGoal_Request) -> bool;
    fn spot_msgs__action__Manipulation_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_SendGoal_Request>, size: usize) -> bool;
    fn spot_msgs__action__Manipulation_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_SendGoal_Request>);
    fn spot_msgs__action__Manipulation_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Manipulation_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Manipulation_SendGoal_Request>) -> bool;
}

// Corresponds to spot_msgs__action__Manipulation_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Manipulation_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Manipulation_Goal,

}



impl Default for Manipulation_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Manipulation_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Manipulation_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Manipulation_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Manipulation_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Manipulation_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Manipulation_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_SendGoal_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Manipulation_SendGoal_Response__init(msg: *mut Manipulation_SendGoal_Response) -> bool;
    fn spot_msgs__action__Manipulation_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_SendGoal_Response>, size: usize) -> bool;
    fn spot_msgs__action__Manipulation_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_SendGoal_Response>);
    fn spot_msgs__action__Manipulation_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Manipulation_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Manipulation_SendGoal_Response>) -> bool;
}

// Corresponds to spot_msgs__action__Manipulation_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Manipulation_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Manipulation_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Manipulation_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Manipulation_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Manipulation_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Manipulation_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Manipulation_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Manipulation_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_SendGoal_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Manipulation_GetResult_Request__init(msg: *mut Manipulation_GetResult_Request) -> bool;
    fn spot_msgs__action__Manipulation_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_GetResult_Request>, size: usize) -> bool;
    fn spot_msgs__action__Manipulation_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_GetResult_Request>);
    fn spot_msgs__action__Manipulation_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Manipulation_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Manipulation_GetResult_Request>) -> bool;
}

// Corresponds to spot_msgs__action__Manipulation_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Manipulation_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Manipulation_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Manipulation_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Manipulation_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Manipulation_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Manipulation_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Manipulation_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Manipulation_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_GetResult_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__action__Manipulation_GetResult_Response__init(msg: *mut Manipulation_GetResult_Response) -> bool;
    fn spot_msgs__action__Manipulation_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_GetResult_Response>, size: usize) -> bool;
    fn spot_msgs__action__Manipulation_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Manipulation_GetResult_Response>);
    fn spot_msgs__action__Manipulation_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Manipulation_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Manipulation_GetResult_Response>) -> bool;
}

// Corresponds to spot_msgs__action__Manipulation_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Manipulation_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Manipulation_Result,

}



impl Default for Manipulation_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__action__Manipulation_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__action__Manipulation_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Manipulation_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__action__Manipulation_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Manipulation_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Manipulation_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/action/Manipulation_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__action__Manipulation_GetResult_Response() }
  }
}






#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__ArmSurfaceContact_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__ArmSurfaceContact_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct ArmSurfaceContact_SendGoal;

impl rosidl_runtime_rs::Service for ArmSurfaceContact_SendGoal {
    type Request = ArmSurfaceContact_SendGoal_Request;
    type Response = ArmSurfaceContact_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__ArmSurfaceContact_SendGoal() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__ArmSurfaceContact_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__ArmSurfaceContact_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct ArmSurfaceContact_GetResult;

impl rosidl_runtime_rs::Service for ArmSurfaceContact_GetResult {
    type Request = ArmSurfaceContact_GetResult_Request;
    type Response = ArmSurfaceContact_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__ArmSurfaceContact_GetResult() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__ExecuteDance_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__ExecuteDance_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct ExecuteDance_SendGoal;

impl rosidl_runtime_rs::Service for ExecuteDance_SendGoal {
    type Request = ExecuteDance_SendGoal_Request;
    type Response = ExecuteDance_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__ExecuteDance_SendGoal() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__ExecuteDance_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__ExecuteDance_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct ExecuteDance_GetResult;

impl rosidl_runtime_rs::Service for ExecuteDance_GetResult {
    type Request = ExecuteDance_GetResult_Request;
    type Response = ExecuteDance_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__ExecuteDance_GetResult() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__NavigateTo_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__NavigateTo_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct NavigateTo_SendGoal;

impl rosidl_runtime_rs::Service for NavigateTo_SendGoal {
    type Request = NavigateTo_SendGoal_Request;
    type Response = NavigateTo_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__NavigateTo_SendGoal() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__NavigateTo_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__NavigateTo_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct NavigateTo_GetResult;

impl rosidl_runtime_rs::Service for NavigateTo_GetResult {
    type Request = NavigateTo_GetResult_Request;
    type Response = NavigateTo_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__NavigateTo_GetResult() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__RobotCommand_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__RobotCommand_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct RobotCommand_SendGoal;

impl rosidl_runtime_rs::Service for RobotCommand_SendGoal {
    type Request = RobotCommand_SendGoal_Request;
    type Response = RobotCommand_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__RobotCommand_SendGoal() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__RobotCommand_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__RobotCommand_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct RobotCommand_GetResult;

impl rosidl_runtime_rs::Service for RobotCommand_GetResult {
    type Request = RobotCommand_GetResult_Request;
    type Response = RobotCommand_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__RobotCommand_GetResult() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__Trajectory_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__Trajectory_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Trajectory_SendGoal;

impl rosidl_runtime_rs::Service for Trajectory_SendGoal {
    type Request = Trajectory_SendGoal_Request;
    type Response = Trajectory_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__Trajectory_SendGoal() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__Trajectory_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__Trajectory_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Trajectory_GetResult;

impl rosidl_runtime_rs::Service for Trajectory_GetResult {
    type Request = Trajectory_GetResult_Request;
    type Response = Trajectory_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__Trajectory_GetResult() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__Manipulation_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__Manipulation_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Manipulation_SendGoal;

impl rosidl_runtime_rs::Service for Manipulation_SendGoal {
    type Request = Manipulation_SendGoal_Request;
    type Response = Manipulation_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__Manipulation_SendGoal() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__Manipulation_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__action__Manipulation_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Manipulation_GetResult;

impl rosidl_runtime_rs::Service for Manipulation_GetResult {
    type Request = Manipulation_GetResult_Request;
    type Response = Manipulation_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__action__Manipulation_GetResult() }
    }
}


