#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__AcquireLease_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__AcquireLease_Request__init(msg: *mut AcquireLease_Request) -> bool;
    fn spot_msgs__srv__AcquireLease_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AcquireLease_Request>, size: usize) -> bool;
    fn spot_msgs__srv__AcquireLease_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AcquireLease_Request>);
    fn spot_msgs__srv__AcquireLease_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AcquireLease_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AcquireLease_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__AcquireLease_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AcquireLease_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub client_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub resource_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub force: bool,

}



impl Default for AcquireLease_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__AcquireLease_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__AcquireLease_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AcquireLease_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__AcquireLease_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__AcquireLease_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__AcquireLease_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AcquireLease_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AcquireLease_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/AcquireLease_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__AcquireLease_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__AcquireLease_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__AcquireLease_Response__init(msg: *mut AcquireLease_Response) -> bool;
    fn spot_msgs__srv__AcquireLease_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AcquireLease_Response>, size: usize) -> bool;
    fn spot_msgs__srv__AcquireLease_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AcquireLease_Response>);
    fn spot_msgs__srv__AcquireLease_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AcquireLease_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AcquireLease_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__AcquireLease_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AcquireLease_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lease: bosdyn_api_msgs::msg::rmw::Lease,

}



impl Default for AcquireLease_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__AcquireLease_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__AcquireLease_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AcquireLease_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__AcquireLease_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__AcquireLease_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__AcquireLease_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AcquireLease_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AcquireLease_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/AcquireLease_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__AcquireLease_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__init(msg: *mut ChoreographyRecordedStateToAnimation_Request) -> bool;
    fn spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyRecordedStateToAnimation_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyRecordedStateToAnimation_Request>);
    fn spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChoreographyRecordedStateToAnimation_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ChoreographyRecordedStateToAnimation_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyRecordedStateToAnimation_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub has_arm: bool,

}



impl Default for ChoreographyRecordedStateToAnimation_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChoreographyRecordedStateToAnimation_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChoreographyRecordedStateToAnimation_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChoreographyRecordedStateToAnimation_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ChoreographyRecordedStateToAnimation_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__init(msg: *mut ChoreographyRecordedStateToAnimation_Response) -> bool;
    fn spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyRecordedStateToAnimation_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyRecordedStateToAnimation_Response>);
    fn spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChoreographyRecordedStateToAnimation_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ChoreographyRecordedStateToAnimation_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyRecordedStateToAnimation_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub animation_file_contents: rosidl_runtime_rs::String,

}



impl Default for ChoreographyRecordedStateToAnimation_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChoreographyRecordedStateToAnimation_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChoreographyRecordedStateToAnimation_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChoreographyRecordedStateToAnimation_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ChoreographyRecordedStateToAnimation_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyStartRecordingState_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ChoreographyStartRecordingState_Request__init(msg: *mut ChoreographyStartRecordingState_Request) -> bool;
    fn spot_msgs__srv__ChoreographyStartRecordingState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStartRecordingState_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ChoreographyStartRecordingState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStartRecordingState_Request>);
    fn spot_msgs__srv__ChoreographyStartRecordingState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChoreographyStartRecordingState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStartRecordingState_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ChoreographyStartRecordingState_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyStartRecordingState_Request {
    /// Start Message
    pub duration_seconds: f32,

}

impl ChoreographyStartRecordingState_Request {
    /// Start Replicated Enum
    pub const STATUS_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_OK: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_UNKNOWN_RECORDING_SESSION_ID: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_RECORDING_BUFFER_FULL: u8 = 3;

}


impl Default for ChoreographyStartRecordingState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ChoreographyStartRecordingState_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ChoreographyStartRecordingState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChoreographyStartRecordingState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStartRecordingState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStartRecordingState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStartRecordingState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChoreographyStartRecordingState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChoreographyStartRecordingState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ChoreographyStartRecordingState_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyStartRecordingState_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyStartRecordingState_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ChoreographyStartRecordingState_Response__init(msg: *mut ChoreographyStartRecordingState_Response) -> bool;
    fn spot_msgs__srv__ChoreographyStartRecordingState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStartRecordingState_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ChoreographyStartRecordingState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStartRecordingState_Response>);
    fn spot_msgs__srv__ChoreographyStartRecordingState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChoreographyStartRecordingState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStartRecordingState_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ChoreographyStartRecordingState_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyStartRecordingState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub recording_session_id: u64,

}



impl Default for ChoreographyStartRecordingState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ChoreographyStartRecordingState_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ChoreographyStartRecordingState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChoreographyStartRecordingState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStartRecordingState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStartRecordingState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStartRecordingState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChoreographyStartRecordingState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChoreographyStartRecordingState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ChoreographyStartRecordingState_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyStartRecordingState_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyStopRecordingState_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ChoreographyStopRecordingState_Request__init(msg: *mut ChoreographyStopRecordingState_Request) -> bool;
    fn spot_msgs__srv__ChoreographyStopRecordingState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStopRecordingState_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ChoreographyStopRecordingState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStopRecordingState_Request>);
    fn spot_msgs__srv__ChoreographyStopRecordingState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChoreographyStopRecordingState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStopRecordingState_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ChoreographyStopRecordingState_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyStopRecordingState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ChoreographyStopRecordingState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ChoreographyStopRecordingState_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ChoreographyStopRecordingState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChoreographyStopRecordingState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStopRecordingState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStopRecordingState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStopRecordingState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChoreographyStopRecordingState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChoreographyStopRecordingState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ChoreographyStopRecordingState_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyStopRecordingState_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyStopRecordingState_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ChoreographyStopRecordingState_Response__init(msg: *mut ChoreographyStopRecordingState_Response) -> bool;
    fn spot_msgs__srv__ChoreographyStopRecordingState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStopRecordingState_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ChoreographyStopRecordingState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStopRecordingState_Response>);
    fn spot_msgs__srv__ChoreographyStopRecordingState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChoreographyStopRecordingState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ChoreographyStopRecordingState_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ChoreographyStopRecordingState_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyStopRecordingState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for ChoreographyStopRecordingState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ChoreographyStopRecordingState_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ChoreographyStopRecordingState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChoreographyStopRecordingState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStopRecordingState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStopRecordingState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ChoreographyStopRecordingState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChoreographyStopRecordingState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChoreographyStopRecordingState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ChoreographyStopRecordingState_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ChoreographyStopRecordingState_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetChoreographyStatus_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetChoreographyStatus_Request__init(msg: *mut GetChoreographyStatus_Request) -> bool;
    fn spot_msgs__srv__GetChoreographyStatus_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetChoreographyStatus_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GetChoreographyStatus_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetChoreographyStatus_Request>);
    fn spot_msgs__srv__GetChoreographyStatus_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetChoreographyStatus_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetChoreographyStatus_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GetChoreographyStatus_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetChoreographyStatus_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}

impl GetChoreographyStatus_Request {
    /// Start Replicated Enum
    pub const STATUS_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_DANCING: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_COMPLETED_SEQUENCE: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_PREPPING: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_WAITING_FOR_START_TIME: u8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_VALIDATING: u8 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_INTERRUPTED: u8 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_FALLEN: u8 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_POWERED_OFF: u8 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_OTHER: u8 = 9;

}


impl Default for GetChoreographyStatus_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetChoreographyStatus_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetChoreographyStatus_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetChoreographyStatus_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetChoreographyStatus_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetChoreographyStatus_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetChoreographyStatus_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetChoreographyStatus_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetChoreographyStatus_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetChoreographyStatus_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetChoreographyStatus_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetChoreographyStatus_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetChoreographyStatus_Response__init(msg: *mut GetChoreographyStatus_Response) -> bool;
    fn spot_msgs__srv__GetChoreographyStatus_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetChoreographyStatus_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GetChoreographyStatus_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetChoreographyStatus_Response>);
    fn spot_msgs__srv__GetChoreographyStatus_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetChoreographyStatus_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetChoreographyStatus_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GetChoreographyStatus_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetChoreographyStatus_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub execution_id: i32,

}



impl Default for GetChoreographyStatus_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetChoreographyStatus_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetChoreographyStatus_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetChoreographyStatus_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetChoreographyStatus_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetChoreographyStatus_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetChoreographyStatus_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetChoreographyStatus_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetChoreographyStatus_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetChoreographyStatus_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetChoreographyStatus_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetInverseKinematicSolutions_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetInverseKinematicSolutions_Request__init(msg: *mut GetInverseKinematicSolutions_Request) -> bool;
    fn spot_msgs__srv__GetInverseKinematicSolutions_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInverseKinematicSolutions_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GetInverseKinematicSolutions_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInverseKinematicSolutions_Request>);
    fn spot_msgs__srv__GetInverseKinematicSolutions_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInverseKinematicSolutions_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInverseKinematicSolutions_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GetInverseKinematicSolutions_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInverseKinematicSolutions_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bosdyn_spot_api_msgs::msg::rmw::InverseKinematicsRequest,

}



impl Default for GetInverseKinematicSolutions_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetInverseKinematicSolutions_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetInverseKinematicSolutions_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInverseKinematicSolutions_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetInverseKinematicSolutions_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetInverseKinematicSolutions_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetInverseKinematicSolutions_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInverseKinematicSolutions_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInverseKinematicSolutions_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetInverseKinematicSolutions_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetInverseKinematicSolutions_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetInverseKinematicSolutions_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetInverseKinematicSolutions_Response__init(msg: *mut GetInverseKinematicSolutions_Response) -> bool;
    fn spot_msgs__srv__GetInverseKinematicSolutions_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInverseKinematicSolutions_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GetInverseKinematicSolutions_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInverseKinematicSolutions_Response>);
    fn spot_msgs__srv__GetInverseKinematicSolutions_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInverseKinematicSolutions_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInverseKinematicSolutions_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GetInverseKinematicSolutions_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInverseKinematicSolutions_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: bosdyn_spot_api_msgs::msg::rmw::InverseKinematicsResponse,

}



impl Default for GetInverseKinematicSolutions_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetInverseKinematicSolutions_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetInverseKinematicSolutions_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInverseKinematicSolutions_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetInverseKinematicSolutions_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetInverseKinematicSolutions_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetInverseKinematicSolutions_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInverseKinematicSolutions_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInverseKinematicSolutions_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetInverseKinematicSolutions_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetInverseKinematicSolutions_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListGraph_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListGraph_Request__init(msg: *mut ListGraph_Request) -> bool;
    fn spot_msgs__srv__ListGraph_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListGraph_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ListGraph_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListGraph_Request>);
    fn spot_msgs__srv__ListGraph_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListGraph_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListGraph_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ListGraph_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListGraph_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub upload_filepath: rosidl_runtime_rs::String,

}



impl Default for ListGraph_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListGraph_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListGraph_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListGraph_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListGraph_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListGraph_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListGraph_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListGraph_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListGraph_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListGraph_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListGraph_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListGraph_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListGraph_Response__init(msg: *mut ListGraph_Response) -> bool;
    fn spot_msgs__srv__ListGraph_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListGraph_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ListGraph_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListGraph_Response>);
    fn spot_msgs__srv__ListGraph_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListGraph_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListGraph_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ListGraph_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListGraph_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub waypoint_ids: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for ListGraph_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListGraph_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListGraph_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListGraph_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListGraph_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListGraph_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListGraph_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListGraph_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListGraph_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListGraph_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListGraph_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListWorldObjects_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListWorldObjects_Request__init(msg: *mut ListWorldObjects_Request) -> bool;
    fn spot_msgs__srv__ListWorldObjects_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListWorldObjects_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ListWorldObjects_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListWorldObjects_Request>);
    fn spot_msgs__srv__ListWorldObjects_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListWorldObjects_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListWorldObjects_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ListWorldObjects_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListWorldObjects_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bosdyn_api_msgs::msg::rmw::ListWorldObjectRequest,

}



impl Default for ListWorldObjects_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListWorldObjects_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListWorldObjects_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListWorldObjects_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListWorldObjects_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListWorldObjects_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListWorldObjects_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListWorldObjects_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListWorldObjects_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListWorldObjects_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListWorldObjects_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListWorldObjects_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListWorldObjects_Response__init(msg: *mut ListWorldObjects_Response) -> bool;
    fn spot_msgs__srv__ListWorldObjects_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListWorldObjects_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ListWorldObjects_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListWorldObjects_Response>);
    fn spot_msgs__srv__ListWorldObjects_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListWorldObjects_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListWorldObjects_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ListWorldObjects_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListWorldObjects_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: bosdyn_api_msgs::msg::rmw::ListWorldObjectResponse,

}



impl Default for ListWorldObjects_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListWorldObjects_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListWorldObjects_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListWorldObjects_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListWorldObjects_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListWorldObjects_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListWorldObjects_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListWorldObjects_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListWorldObjects_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListWorldObjects_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListWorldObjects_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ReturnLease_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ReturnLease_Request__init(msg: *mut ReturnLease_Request) -> bool;
    fn spot_msgs__srv__ReturnLease_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ReturnLease_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ReturnLease_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ReturnLease_Request>);
    fn spot_msgs__srv__ReturnLease_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ReturnLease_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ReturnLease_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ReturnLease_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReturnLease_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub lease: bosdyn_api_msgs::msg::rmw::Lease,

}



impl Default for ReturnLease_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ReturnLease_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ReturnLease_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ReturnLease_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ReturnLease_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ReturnLease_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ReturnLease_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ReturnLease_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ReturnLease_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ReturnLease_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ReturnLease_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ReturnLease_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ReturnLease_Response__init(msg: *mut ReturnLease_Response) -> bool;
    fn spot_msgs__srv__ReturnLease_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ReturnLease_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ReturnLease_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ReturnLease_Response>);
    fn spot_msgs__srv__ReturnLease_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ReturnLease_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ReturnLease_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ReturnLease_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReturnLease_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for ReturnLease_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ReturnLease_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ReturnLease_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ReturnLease_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ReturnLease_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ReturnLease_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ReturnLease_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ReturnLease_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ReturnLease_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ReturnLease_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ReturnLease_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetLocomotion_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetLocomotion_Request__init(msg: *mut SetLocomotion_Request) -> bool;
    fn spot_msgs__srv__SetLocomotion_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLocomotion_Request>, size: usize) -> bool;
    fn spot_msgs__srv__SetLocomotion_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLocomotion_Request>);
    fn spot_msgs__srv__SetLocomotion_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLocomotion_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLocomotion_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__SetLocomotion_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLocomotion_Request {
    /// See https://dev.bostondynamics.com/protos/bosdyn/api/proto_reference.html?highlight=mobilityparams#locomotionhint for details
    pub locomotion_mode: u32,

}



impl Default for SetLocomotion_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetLocomotion_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetLocomotion_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLocomotion_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLocomotion_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLocomotion_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLocomotion_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLocomotion_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLocomotion_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetLocomotion_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetLocomotion_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetLocomotion_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetLocomotion_Response__init(msg: *mut SetLocomotion_Response) -> bool;
    fn spot_msgs__srv__SetLocomotion_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLocomotion_Response>, size: usize) -> bool;
    fn spot_msgs__srv__SetLocomotion_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLocomotion_Response>);
    fn spot_msgs__srv__SetLocomotion_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLocomotion_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLocomotion_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__SetLocomotion_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLocomotion_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetLocomotion_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetLocomotion_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetLocomotion_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLocomotion_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLocomotion_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLocomotion_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLocomotion_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLocomotion_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLocomotion_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetLocomotion_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetLocomotion_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetVelocity_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetVelocity_Request__init(msg: *mut SetVelocity_Request) -> bool;
    fn spot_msgs__srv__SetVelocity_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetVelocity_Request>, size: usize) -> bool;
    fn spot_msgs__srv__SetVelocity_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetVelocity_Request>);
    fn spot_msgs__srv__SetVelocity_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetVelocity_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetVelocity_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__SetVelocity_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetVelocity_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity_limit: geometry_msgs::msg::rmw::Twist,

}



impl Default for SetVelocity_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetVelocity_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetVelocity_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetVelocity_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVelocity_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVelocity_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVelocity_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetVelocity_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetVelocity_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetVelocity_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetVelocity_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetVelocity_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetVelocity_Response__init(msg: *mut SetVelocity_Response) -> bool;
    fn spot_msgs__srv__SetVelocity_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetVelocity_Response>, size: usize) -> bool;
    fn spot_msgs__srv__SetVelocity_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetVelocity_Response>);
    fn spot_msgs__srv__SetVelocity_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetVelocity_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetVelocity_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__SetVelocity_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetVelocity_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetVelocity_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetVelocity_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetVelocity_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetVelocity_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVelocity_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVelocity_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVelocity_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetVelocity_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetVelocity_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetVelocity_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetVelocity_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListAllDances_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListAllDances_Request__init(msg: *mut ListAllDances_Request) -> bool;
    fn spot_msgs__srv__ListAllDances_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListAllDances_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ListAllDances_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListAllDances_Request>);
    fn spot_msgs__srv__ListAllDances_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListAllDances_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListAllDances_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ListAllDances_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAllDances_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListAllDances_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListAllDances_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListAllDances_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListAllDances_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllDances_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllDances_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllDances_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListAllDances_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListAllDances_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListAllDances_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListAllDances_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListAllDances_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListAllDances_Response__init(msg: *mut ListAllDances_Response) -> bool;
    fn spot_msgs__srv__ListAllDances_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListAllDances_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ListAllDances_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListAllDances_Response>);
    fn spot_msgs__srv__ListAllDances_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListAllDances_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListAllDances_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ListAllDances_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAllDances_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dances: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for ListAllDances_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListAllDances_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListAllDances_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListAllDances_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllDances_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllDances_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllDances_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListAllDances_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListAllDances_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListAllDances_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListAllDances_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListAllMoves_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListAllMoves_Request__init(msg: *mut ListAllMoves_Request) -> bool;
    fn spot_msgs__srv__ListAllMoves_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListAllMoves_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ListAllMoves_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListAllMoves_Request>);
    fn spot_msgs__srv__ListAllMoves_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListAllMoves_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListAllMoves_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ListAllMoves_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAllMoves_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListAllMoves_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListAllMoves_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListAllMoves_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListAllMoves_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllMoves_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllMoves_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllMoves_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListAllMoves_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListAllMoves_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListAllMoves_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListAllMoves_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListAllMoves_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListAllMoves_Response__init(msg: *mut ListAllMoves_Response) -> bool;
    fn spot_msgs__srv__ListAllMoves_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListAllMoves_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ListAllMoves_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListAllMoves_Response>);
    fn spot_msgs__srv__ListAllMoves_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListAllMoves_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListAllMoves_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ListAllMoves_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAllMoves_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub moves: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for ListAllMoves_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListAllMoves_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListAllMoves_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListAllMoves_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllMoves_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllMoves_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListAllMoves_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListAllMoves_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListAllMoves_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListAllMoves_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListAllMoves_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__UploadAnimation_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__UploadAnimation_Request__init(msg: *mut UploadAnimation_Request) -> bool;
    fn spot_msgs__srv__UploadAnimation_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UploadAnimation_Request>, size: usize) -> bool;
    fn spot_msgs__srv__UploadAnimation_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UploadAnimation_Request>);
    fn spot_msgs__srv__UploadAnimation_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UploadAnimation_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<UploadAnimation_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__UploadAnimation_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadAnimation_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub animation_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub animation_file_content: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub animation_proto_serialized: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for UploadAnimation_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__UploadAnimation_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__UploadAnimation_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UploadAnimation_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadAnimation_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadAnimation_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadAnimation_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UploadAnimation_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UploadAnimation_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/UploadAnimation_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__UploadAnimation_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__UploadAnimation_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__UploadAnimation_Response__init(msg: *mut UploadAnimation_Response) -> bool;
    fn spot_msgs__srv__UploadAnimation_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UploadAnimation_Response>, size: usize) -> bool;
    fn spot_msgs__srv__UploadAnimation_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UploadAnimation_Response>);
    fn spot_msgs__srv__UploadAnimation_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UploadAnimation_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<UploadAnimation_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__UploadAnimation_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadAnimation_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for UploadAnimation_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__UploadAnimation_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__UploadAnimation_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UploadAnimation_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadAnimation_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadAnimation_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadAnimation_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UploadAnimation_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UploadAnimation_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/UploadAnimation_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__UploadAnimation_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__UploadSequence_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__UploadSequence_Request__init(msg: *mut UploadSequence_Request) -> bool;
    fn spot_msgs__srv__UploadSequence_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UploadSequence_Request>, size: usize) -> bool;
    fn spot_msgs__srv__UploadSequence_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UploadSequence_Request>);
    fn spot_msgs__srv__UploadSequence_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UploadSequence_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<UploadSequence_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__UploadSequence_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadSequence_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_proto_serialized: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for UploadSequence_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__UploadSequence_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__UploadSequence_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UploadSequence_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadSequence_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadSequence_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadSequence_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UploadSequence_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UploadSequence_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/UploadSequence_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__UploadSequence_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__UploadSequence_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__UploadSequence_Response__init(msg: *mut UploadSequence_Response) -> bool;
    fn spot_msgs__srv__UploadSequence_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UploadSequence_Response>, size: usize) -> bool;
    fn spot_msgs__srv__UploadSequence_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UploadSequence_Response>);
    fn spot_msgs__srv__UploadSequence_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UploadSequence_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<UploadSequence_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__UploadSequence_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadSequence_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for UploadSequence_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__UploadSequence_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__UploadSequence_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UploadSequence_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadSequence_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadSequence_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__UploadSequence_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UploadSequence_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UploadSequence_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/UploadSequence_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__UploadSequence_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ClearBehaviorFault_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ClearBehaviorFault_Request__init(msg: *mut ClearBehaviorFault_Request) -> bool;
    fn spot_msgs__srv__ClearBehaviorFault_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ClearBehaviorFault_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ClearBehaviorFault_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ClearBehaviorFault_Request>);
    fn spot_msgs__srv__ClearBehaviorFault_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ClearBehaviorFault_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ClearBehaviorFault_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ClearBehaviorFault_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearBehaviorFault_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u32,

}



impl Default for ClearBehaviorFault_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ClearBehaviorFault_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ClearBehaviorFault_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ClearBehaviorFault_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ClearBehaviorFault_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ClearBehaviorFault_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ClearBehaviorFault_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ClearBehaviorFault_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ClearBehaviorFault_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ClearBehaviorFault_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ClearBehaviorFault_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ClearBehaviorFault_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ClearBehaviorFault_Response__init(msg: *mut ClearBehaviorFault_Response) -> bool;
    fn spot_msgs__srv__ClearBehaviorFault_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ClearBehaviorFault_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ClearBehaviorFault_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ClearBehaviorFault_Response>);
    fn spot_msgs__srv__ClearBehaviorFault_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ClearBehaviorFault_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ClearBehaviorFault_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ClearBehaviorFault_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearBehaviorFault_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for ClearBehaviorFault_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ClearBehaviorFault_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ClearBehaviorFault_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ClearBehaviorFault_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ClearBehaviorFault_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ClearBehaviorFault_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ClearBehaviorFault_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ClearBehaviorFault_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ClearBehaviorFault_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ClearBehaviorFault_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ClearBehaviorFault_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListSounds_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListSounds_Request__init(msg: *mut ListSounds_Request) -> bool;
    fn spot_msgs__srv__ListSounds_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListSounds_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ListSounds_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListSounds_Request>);
    fn spot_msgs__srv__ListSounds_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListSounds_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListSounds_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ListSounds_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListSounds_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListSounds_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListSounds_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListSounds_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListSounds_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListSounds_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListSounds_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListSounds_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListSounds_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListSounds_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListSounds_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListSounds_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListSounds_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListSounds_Response__init(msg: *mut ListSounds_Response) -> bool;
    fn spot_msgs__srv__ListSounds_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListSounds_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ListSounds_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListSounds_Response>);
    fn spot_msgs__srv__ListSounds_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListSounds_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListSounds_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ListSounds_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListSounds_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for ListSounds_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListSounds_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListSounds_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListSounds_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListSounds_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListSounds_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListSounds_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListSounds_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListSounds_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListSounds_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListSounds_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__LoadSound_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__LoadSound_Request__init(msg: *mut LoadSound_Request) -> bool;
    fn spot_msgs__srv__LoadSound_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadSound_Request>, size: usize) -> bool;
    fn spot_msgs__srv__LoadSound_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadSound_Request>);
    fn spot_msgs__srv__LoadSound_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadSound_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadSound_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__LoadSound_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadSound_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

    /// Path to wav file to upload
    pub wav_path: rosidl_runtime_rs::String,

}



impl Default for LoadSound_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__LoadSound_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__LoadSound_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadSound_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__LoadSound_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__LoadSound_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__LoadSound_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadSound_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadSound_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/LoadSound_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__LoadSound_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__LoadSound_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__LoadSound_Response__init(msg: *mut LoadSound_Response) -> bool;
    fn spot_msgs__srv__LoadSound_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadSound_Response>, size: usize) -> bool;
    fn spot_msgs__srv__LoadSound_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadSound_Response>);
    fn spot_msgs__srv__LoadSound_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadSound_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadSound_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__LoadSound_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadSound_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for LoadSound_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__LoadSound_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__LoadSound_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadSound_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__LoadSound_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__LoadSound_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__LoadSound_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadSound_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadSound_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/LoadSound_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__LoadSound_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__PlaySound_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__PlaySound_Request__init(msg: *mut PlaySound_Request) -> bool;
    fn spot_msgs__srv__PlaySound_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Request>, size: usize) -> bool;
    fn spot_msgs__srv__PlaySound_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Request>);
    fn spot_msgs__srv__PlaySound_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaySound_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__PlaySound_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub volume_multiplier: f32,

}



impl Default for PlaySound_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__PlaySound_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__PlaySound_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaySound_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__PlaySound_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__PlaySound_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__PlaySound_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaySound_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaySound_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/PlaySound_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__PlaySound_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__PlaySound_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__PlaySound_Response__init(msg: *mut PlaySound_Response) -> bool;
    fn spot_msgs__srv__PlaySound_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Response>, size: usize) -> bool;
    fn spot_msgs__srv__PlaySound_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Response>);
    fn spot_msgs__srv__PlaySound_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaySound_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__PlaySound_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for PlaySound_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__PlaySound_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__PlaySound_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaySound_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__PlaySound_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__PlaySound_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__PlaySound_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaySound_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaySound_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/PlaySound_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__PlaySound_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__DeleteSound_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__DeleteSound_Request__init(msg: *mut DeleteSound_Request) -> bool;
    fn spot_msgs__srv__DeleteSound_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteSound_Request>, size: usize) -> bool;
    fn spot_msgs__srv__DeleteSound_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteSound_Request>);
    fn spot_msgs__srv__DeleteSound_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteSound_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteSound_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__DeleteSound_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteSound_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

}



impl Default for DeleteSound_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__DeleteSound_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__DeleteSound_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteSound_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteSound_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteSound_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteSound_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteSound_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteSound_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/DeleteSound_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__DeleteSound_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__DeleteSound_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__DeleteSound_Response__init(msg: *mut DeleteSound_Response) -> bool;
    fn spot_msgs__srv__DeleteSound_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteSound_Response>, size: usize) -> bool;
    fn spot_msgs__srv__DeleteSound_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteSound_Response>);
    fn spot_msgs__srv__DeleteSound_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteSound_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteSound_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__DeleteSound_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteSound_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for DeleteSound_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__DeleteSound_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__DeleteSound_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteSound_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteSound_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteSound_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteSound_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteSound_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteSound_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/DeleteSound_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__DeleteSound_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetVolume_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetVolume_Request__init(msg: *mut GetVolume_Request) -> bool;
    fn spot_msgs__srv__GetVolume_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetVolume_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GetVolume_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetVolume_Request>);
    fn spot_msgs__srv__GetVolume_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetVolume_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetVolume_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GetVolume_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetVolume_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetVolume_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetVolume_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetVolume_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetVolume_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetVolume_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetVolume_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetVolume_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetVolume_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetVolume_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetVolume_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetVolume_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetVolume_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetVolume_Response__init(msg: *mut GetVolume_Response) -> bool;
    fn spot_msgs__srv__GetVolume_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetVolume_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GetVolume_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetVolume_Response>);
    fn spot_msgs__srv__GetVolume_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetVolume_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetVolume_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GetVolume_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetVolume_Response {
    /// From 0 to 100
    pub volume: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for GetVolume_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetVolume_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetVolume_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetVolume_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetVolume_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetVolume_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetVolume_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetVolume_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetVolume_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetVolume_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetVolume_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetVolume_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetVolume_Request__init(msg: *mut SetVolume_Request) -> bool;
    fn spot_msgs__srv__SetVolume_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetVolume_Request>, size: usize) -> bool;
    fn spot_msgs__srv__SetVolume_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetVolume_Request>);
    fn spot_msgs__srv__SetVolume_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetVolume_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetVolume_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__SetVolume_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetVolume_Request {
    /// From 0 to 100
    pub volume: f32,

}



impl Default for SetVolume_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetVolume_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetVolume_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetVolume_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVolume_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVolume_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVolume_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetVolume_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetVolume_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetVolume_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetVolume_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetVolume_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetVolume_Response__init(msg: *mut SetVolume_Response) -> bool;
    fn spot_msgs__srv__SetVolume_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetVolume_Response>, size: usize) -> bool;
    fn spot_msgs__srv__SetVolume_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetVolume_Response>);
    fn spot_msgs__srv__SetVolume_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetVolume_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetVolume_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__SetVolume_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetVolume_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetVolume_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetVolume_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetVolume_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetVolume_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVolume_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVolume_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetVolume_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetVolume_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetVolume_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetVolume_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetVolume_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListPtz_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListPtz_Request__init(msg: *mut ListPtz_Request) -> bool;
    fn spot_msgs__srv__ListPtz_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListPtz_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ListPtz_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListPtz_Request>);
    fn spot_msgs__srv__ListPtz_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListPtz_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListPtz_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ListPtz_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListPtz_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListPtz_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListPtz_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListPtz_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListPtz_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListPtz_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListPtz_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListPtz_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListPtz_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListPtz_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListPtz_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListPtz_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListPtz_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListPtz_Response__init(msg: *mut ListPtz_Response) -> bool;
    fn spot_msgs__srv__ListPtz_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListPtz_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ListPtz_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListPtz_Response>);
    fn spot_msgs__srv__ListPtz_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListPtz_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListPtz_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ListPtz_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListPtz_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub descriptions: rosidl_runtime_rs::Sequence<bosdyn_spot_cam_api_msgs::msg::rmw::PtzDescription>,

}



impl Default for ListPtz_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListPtz_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListPtz_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListPtz_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListPtz_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListPtz_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListPtz_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListPtz_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListPtz_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListPtz_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListPtz_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetPtzPosition_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetPtzPosition_Request__init(msg: *mut GetPtzPosition_Request) -> bool;
    fn spot_msgs__srv__GetPtzPosition_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPtzPosition_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GetPtzPosition_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPtzPosition_Request>);
    fn spot_msgs__srv__GetPtzPosition_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPtzPosition_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPtzPosition_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GetPtzPosition_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPtzPosition_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

}



impl Default for GetPtzPosition_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetPtzPosition_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetPtzPosition_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPtzPosition_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetPtzPosition_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetPtzPosition_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetPtzPosition_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPtzPosition_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPtzPosition_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetPtzPosition_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetPtzPosition_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetPtzPosition_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetPtzPosition_Response__init(msg: *mut GetPtzPosition_Response) -> bool;
    fn spot_msgs__srv__GetPtzPosition_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPtzPosition_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GetPtzPosition_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPtzPosition_Response>);
    fn spot_msgs__srv__GetPtzPosition_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPtzPosition_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPtzPosition_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GetPtzPosition_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPtzPosition_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: bosdyn_spot_cam_api_msgs::msg::rmw::PtzPosition,

}



impl Default for GetPtzPosition_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetPtzPosition_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetPtzPosition_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPtzPosition_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetPtzPosition_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetPtzPosition_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetPtzPosition_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPtzPosition_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPtzPosition_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetPtzPosition_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetPtzPosition_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetPtzPosition_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetPtzPosition_Request__init(msg: *mut SetPtzPosition_Request) -> bool;
    fn spot_msgs__srv__SetPtzPosition_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPtzPosition_Request>, size: usize) -> bool;
    fn spot_msgs__srv__SetPtzPosition_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPtzPosition_Request>);
    fn spot_msgs__srv__SetPtzPosition_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPtzPosition_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPtzPosition_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__SetPtzPosition_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPtzPosition_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pan: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tilt: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub zoom: f32,

}



impl Default for SetPtzPosition_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetPtzPosition_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetPtzPosition_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPtzPosition_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetPtzPosition_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetPtzPosition_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetPtzPosition_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPtzPosition_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPtzPosition_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetPtzPosition_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetPtzPosition_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetPtzPosition_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetPtzPosition_Response__init(msg: *mut SetPtzPosition_Response) -> bool;
    fn spot_msgs__srv__SetPtzPosition_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPtzPosition_Response>, size: usize) -> bool;
    fn spot_msgs__srv__SetPtzPosition_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPtzPosition_Response>);
    fn spot_msgs__srv__SetPtzPosition_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPtzPosition_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPtzPosition_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__SetPtzPosition_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPtzPosition_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetPtzPosition_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetPtzPosition_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetPtzPosition_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPtzPosition_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetPtzPosition_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetPtzPosition_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetPtzPosition_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPtzPosition_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPtzPosition_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetPtzPosition_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetPtzPosition_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__InitializeLens_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__InitializeLens_Request__init(msg: *mut InitializeLens_Request) -> bool;
    fn spot_msgs__srv__InitializeLens_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InitializeLens_Request>, size: usize) -> bool;
    fn spot_msgs__srv__InitializeLens_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InitializeLens_Request>);
    fn spot_msgs__srv__InitializeLens_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InitializeLens_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<InitializeLens_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__InitializeLens_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InitializeLens_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for InitializeLens_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__InitializeLens_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__InitializeLens_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InitializeLens_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__InitializeLens_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__InitializeLens_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__InitializeLens_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InitializeLens_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InitializeLens_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/InitializeLens_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__InitializeLens_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__InitializeLens_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__InitializeLens_Response__init(msg: *mut InitializeLens_Response) -> bool;
    fn spot_msgs__srv__InitializeLens_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InitializeLens_Response>, size: usize) -> bool;
    fn spot_msgs__srv__InitializeLens_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InitializeLens_Response>);
    fn spot_msgs__srv__InitializeLens_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InitializeLens_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<InitializeLens_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__InitializeLens_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InitializeLens_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for InitializeLens_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__InitializeLens_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__InitializeLens_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InitializeLens_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__InitializeLens_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__InitializeLens_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__InitializeLens_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InitializeLens_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InitializeLens_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/InitializeLens_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__InitializeLens_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__DeleteLogpoint_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__DeleteLogpoint_Request__init(msg: *mut DeleteLogpoint_Request) -> bool;
    fn spot_msgs__srv__DeleteLogpoint_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteLogpoint_Request>, size: usize) -> bool;
    fn spot_msgs__srv__DeleteLogpoint_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteLogpoint_Request>);
    fn spot_msgs__srv__DeleteLogpoint_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteLogpoint_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteLogpoint_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__DeleteLogpoint_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteLogpoint_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

}



impl Default for DeleteLogpoint_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__DeleteLogpoint_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__DeleteLogpoint_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteLogpoint_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteLogpoint_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteLogpoint_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteLogpoint_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteLogpoint_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteLogpoint_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/DeleteLogpoint_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__DeleteLogpoint_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__DeleteLogpoint_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__DeleteLogpoint_Response__init(msg: *mut DeleteLogpoint_Response) -> bool;
    fn spot_msgs__srv__DeleteLogpoint_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteLogpoint_Response>, size: usize) -> bool;
    fn spot_msgs__srv__DeleteLogpoint_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteLogpoint_Response>);
    fn spot_msgs__srv__DeleteLogpoint_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteLogpoint_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteLogpoint_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__DeleteLogpoint_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteLogpoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for DeleteLogpoint_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__DeleteLogpoint_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__DeleteLogpoint_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteLogpoint_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteLogpoint_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteLogpoint_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__DeleteLogpoint_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteLogpoint_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteLogpoint_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/DeleteLogpoint_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__DeleteLogpoint_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetLogpointStatus_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetLogpointStatus_Request__init(msg: *mut GetLogpointStatus_Request) -> bool;
    fn spot_msgs__srv__GetLogpointStatus_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLogpointStatus_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GetLogpointStatus_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLogpointStatus_Request>);
    fn spot_msgs__srv__GetLogpointStatus_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLogpointStatus_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLogpointStatus_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GetLogpointStatus_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLogpointStatus_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

}



impl Default for GetLogpointStatus_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetLogpointStatus_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetLogpointStatus_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLogpointStatus_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLogpointStatus_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLogpointStatus_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLogpointStatus_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLogpointStatus_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLogpointStatus_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetLogpointStatus_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetLogpointStatus_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetLogpointStatus_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetLogpointStatus_Response__init(msg: *mut GetLogpointStatus_Response) -> bool;
    fn spot_msgs__srv__GetLogpointStatus_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLogpointStatus_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GetLogpointStatus_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLogpointStatus_Response>);
    fn spot_msgs__srv__GetLogpointStatus_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLogpointStatus_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLogpointStatus_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GetLogpointStatus_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLogpointStatus_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bosdyn_spot_cam_api_msgs::msg::rmw::LogpointLogStatus,

}



impl Default for GetLogpointStatus_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetLogpointStatus_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetLogpointStatus_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLogpointStatus_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLogpointStatus_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLogpointStatus_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLogpointStatus_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLogpointStatus_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLogpointStatus_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetLogpointStatus_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetLogpointStatus_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListCameras_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListCameras_Request__init(msg: *mut ListCameras_Request) -> bool;
    fn spot_msgs__srv__ListCameras_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListCameras_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ListCameras_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListCameras_Request>);
    fn spot_msgs__srv__ListCameras_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListCameras_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListCameras_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ListCameras_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListCameras_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListCameras_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListCameras_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListCameras_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListCameras_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListCameras_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListCameras_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListCameras_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListCameras_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListCameras_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListCameras_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListCameras_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListCameras_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListCameras_Response__init(msg: *mut ListCameras_Response) -> bool;
    fn spot_msgs__srv__ListCameras_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListCameras_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ListCameras_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListCameras_Response>);
    fn spot_msgs__srv__ListCameras_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListCameras_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListCameras_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ListCameras_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListCameras_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cameras: rosidl_runtime_rs::Sequence<bosdyn_spot_cam_api_msgs::msg::rmw::Camera>,

}



impl Default for ListCameras_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListCameras_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListCameras_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListCameras_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListCameras_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListCameras_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListCameras_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListCameras_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListCameras_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListCameras_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListCameras_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListLogpoints_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListLogpoints_Request__init(msg: *mut ListLogpoints_Request) -> bool;
    fn spot_msgs__srv__ListLogpoints_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListLogpoints_Request>, size: usize) -> bool;
    fn spot_msgs__srv__ListLogpoints_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListLogpoints_Request>);
    fn spot_msgs__srv__ListLogpoints_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListLogpoints_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListLogpoints_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__ListLogpoints_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListLogpoints_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListLogpoints_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListLogpoints_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListLogpoints_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListLogpoints_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListLogpoints_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListLogpoints_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListLogpoints_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListLogpoints_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListLogpoints_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListLogpoints_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListLogpoints_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListLogpoints_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__ListLogpoints_Response__init(msg: *mut ListLogpoints_Response) -> bool;
    fn spot_msgs__srv__ListLogpoints_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListLogpoints_Response>, size: usize) -> bool;
    fn spot_msgs__srv__ListLogpoints_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListLogpoints_Response>);
    fn spot_msgs__srv__ListLogpoints_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListLogpoints_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListLogpoints_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__ListLogpoints_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListLogpoints_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub logpoints: rosidl_runtime_rs::Sequence<bosdyn_spot_cam_api_msgs::msg::rmw::Logpoint>,

}



impl Default for ListLogpoints_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__ListLogpoints_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__ListLogpoints_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListLogpoints_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListLogpoints_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListLogpoints_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__ListLogpoints_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListLogpoints_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListLogpoints_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/ListLogpoints_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__ListLogpoints_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__RetrieveLogpoint_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__RetrieveLogpoint_Request__init(msg: *mut RetrieveLogpoint_Request) -> bool;
    fn spot_msgs__srv__RetrieveLogpoint_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RetrieveLogpoint_Request>, size: usize) -> bool;
    fn spot_msgs__srv__RetrieveLogpoint_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RetrieveLogpoint_Request>);
    fn spot_msgs__srv__RetrieveLogpoint_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RetrieveLogpoint_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RetrieveLogpoint_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__RetrieveLogpoint_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RetrieveLogpoint_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

    /// True to get raw data
    pub raw: bool,

}



impl Default for RetrieveLogpoint_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__RetrieveLogpoint_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__RetrieveLogpoint_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RetrieveLogpoint_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RetrieveLogpoint_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RetrieveLogpoint_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RetrieveLogpoint_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RetrieveLogpoint_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RetrieveLogpoint_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/RetrieveLogpoint_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__RetrieveLogpoint_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__RetrieveLogpoint_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__RetrieveLogpoint_Response__init(msg: *mut RetrieveLogpoint_Response) -> bool;
    fn spot_msgs__srv__RetrieveLogpoint_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RetrieveLogpoint_Response>, size: usize) -> bool;
    fn spot_msgs__srv__RetrieveLogpoint_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RetrieveLogpoint_Response>);
    fn spot_msgs__srv__RetrieveLogpoint_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RetrieveLogpoint_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RetrieveLogpoint_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__RetrieveLogpoint_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RetrieveLogpoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub logpoint: bosdyn_spot_cam_api_msgs::msg::rmw::Logpoint,

    /// Data comes in as byte buffer.
    pub data: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for RetrieveLogpoint_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__RetrieveLogpoint_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__RetrieveLogpoint_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RetrieveLogpoint_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RetrieveLogpoint_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RetrieveLogpoint_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RetrieveLogpoint_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RetrieveLogpoint_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RetrieveLogpoint_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/RetrieveLogpoint_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__RetrieveLogpoint_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__RobotCommand_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__RobotCommand_Request__init(msg: *mut RobotCommand_Request) -> bool;
    fn spot_msgs__srv__RobotCommand_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Request>, size: usize) -> bool;
    fn spot_msgs__srv__RobotCommand_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Request>);
    fn spot_msgs__srv__RobotCommand_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotCommand_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__RobotCommand_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: bosdyn_api_msgs::msg::rmw::RobotCommand,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::rmw::Duration,

}



impl Default for RobotCommand_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__RobotCommand_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__RobotCommand_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotCommand_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RobotCommand_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RobotCommand_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RobotCommand_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotCommand_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/RobotCommand_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__RobotCommand_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__RobotCommand_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__RobotCommand_Response__init(msg: *mut RobotCommand_Response) -> bool;
    fn spot_msgs__srv__RobotCommand_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Response>, size: usize) -> bool;
    fn spot_msgs__srv__RobotCommand_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Response>);
    fn spot_msgs__srv__RobotCommand_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotCommand_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotCommand_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__RobotCommand_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_command_id: u32,

}



impl Default for RobotCommand_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__RobotCommand_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__RobotCommand_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotCommand_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RobotCommand_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RobotCommand_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__RobotCommand_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotCommand_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/RobotCommand_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__RobotCommand_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetGripperAngle_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetGripperAngle_Request__init(msg: *mut SetGripperAngle_Request) -> bool;
    fn spot_msgs__srv__SetGripperAngle_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetGripperAngle_Request>, size: usize) -> bool;
    fn spot_msgs__srv__SetGripperAngle_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetGripperAngle_Request>);
    fn spot_msgs__srv__SetGripperAngle_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetGripperAngle_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetGripperAngle_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__SetGripperAngle_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGripperAngle_Request {
    /// In range [0, 90]
    pub gripper_angle: f32,

}



impl Default for SetGripperAngle_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetGripperAngle_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetGripperAngle_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetGripperAngle_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperAngle_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperAngle_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperAngle_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetGripperAngle_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetGripperAngle_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetGripperAngle_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetGripperAngle_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetGripperAngle_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetGripperAngle_Response__init(msg: *mut SetGripperAngle_Response) -> bool;
    fn spot_msgs__srv__SetGripperAngle_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetGripperAngle_Response>, size: usize) -> bool;
    fn spot_msgs__srv__SetGripperAngle_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetGripperAngle_Response>);
    fn spot_msgs__srv__SetGripperAngle_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetGripperAngle_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetGripperAngle_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__SetGripperAngle_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGripperAngle_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetGripperAngle_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetGripperAngle_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetGripperAngle_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetGripperAngle_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperAngle_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperAngle_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperAngle_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetGripperAngle_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetGripperAngle_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetGripperAngle_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetGripperAngle_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__StoreLogpoint_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__StoreLogpoint_Request__init(msg: *mut StoreLogpoint_Request) -> bool;
    fn spot_msgs__srv__StoreLogpoint_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StoreLogpoint_Request>, size: usize) -> bool;
    fn spot_msgs__srv__StoreLogpoint_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StoreLogpoint_Request>);
    fn spot_msgs__srv__StoreLogpoint_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StoreLogpoint_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<StoreLogpoint_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__StoreLogpoint_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StoreLogpoint_Request {
    /// Can take values: pano, ptz, ir (if ir cam attached), c0, c1, c2, c3, c4
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tag: rosidl_runtime_rs::String,

}



impl Default for StoreLogpoint_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__StoreLogpoint_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__StoreLogpoint_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StoreLogpoint_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__StoreLogpoint_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__StoreLogpoint_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__StoreLogpoint_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StoreLogpoint_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StoreLogpoint_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/StoreLogpoint_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__StoreLogpoint_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__StoreLogpoint_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__StoreLogpoint_Response__init(msg: *mut StoreLogpoint_Response) -> bool;
    fn spot_msgs__srv__StoreLogpoint_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StoreLogpoint_Response>, size: usize) -> bool;
    fn spot_msgs__srv__StoreLogpoint_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StoreLogpoint_Response>);
    fn spot_msgs__srv__StoreLogpoint_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StoreLogpoint_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<StoreLogpoint_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__StoreLogpoint_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StoreLogpoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub logpoint: bosdyn_spot_cam_api_msgs::msg::rmw::Logpoint,

}



impl Default for StoreLogpoint_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__StoreLogpoint_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__StoreLogpoint_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StoreLogpoint_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__StoreLogpoint_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__StoreLogpoint_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__StoreLogpoint_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StoreLogpoint_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StoreLogpoint_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/StoreLogpoint_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__StoreLogpoint_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__TagLogpoint_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__TagLogpoint_Request__init(msg: *mut TagLogpoint_Request) -> bool;
    fn spot_msgs__srv__TagLogpoint_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TagLogpoint_Request>, size: usize) -> bool;
    fn spot_msgs__srv__TagLogpoint_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TagLogpoint_Request>);
    fn spot_msgs__srv__TagLogpoint_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TagLogpoint_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TagLogpoint_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__TagLogpoint_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TagLogpoint_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tag: rosidl_runtime_rs::String,

}



impl Default for TagLogpoint_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__TagLogpoint_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__TagLogpoint_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TagLogpoint_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__TagLogpoint_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__TagLogpoint_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__TagLogpoint_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TagLogpoint_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TagLogpoint_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/TagLogpoint_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__TagLogpoint_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__TagLogpoint_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__TagLogpoint_Response__init(msg: *mut TagLogpoint_Response) -> bool;
    fn spot_msgs__srv__TagLogpoint_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TagLogpoint_Response>, size: usize) -> bool;
    fn spot_msgs__srv__TagLogpoint_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TagLogpoint_Response>);
    fn spot_msgs__srv__TagLogpoint_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TagLogpoint_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TagLogpoint_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__TagLogpoint_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TagLogpoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for TagLogpoint_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__TagLogpoint_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__TagLogpoint_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TagLogpoint_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__TagLogpoint_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__TagLogpoint_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__TagLogpoint_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TagLogpoint_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TagLogpoint_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/TagLogpoint_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__TagLogpoint_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetLEDBrightness_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetLEDBrightness_Request__init(msg: *mut GetLEDBrightness_Request) -> bool;
    fn spot_msgs__srv__GetLEDBrightness_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLEDBrightness_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GetLEDBrightness_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLEDBrightness_Request>);
    fn spot_msgs__srv__GetLEDBrightness_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLEDBrightness_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLEDBrightness_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GetLEDBrightness_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLEDBrightness_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetLEDBrightness_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetLEDBrightness_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetLEDBrightness_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLEDBrightness_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLEDBrightness_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLEDBrightness_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLEDBrightness_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLEDBrightness_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLEDBrightness_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetLEDBrightness_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetLEDBrightness_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetLEDBrightness_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetLEDBrightness_Response__init(msg: *mut GetLEDBrightness_Response) -> bool;
    fn spot_msgs__srv__GetLEDBrightness_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLEDBrightness_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GetLEDBrightness_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLEDBrightness_Response>);
    fn spot_msgs__srv__GetLEDBrightness_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLEDBrightness_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLEDBrightness_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GetLEDBrightness_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLEDBrightness_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

    /// In order REAR_LEFT, FRONT_LEFT, FRONT_RIGHT, REAR_RIGHT
    pub brightness: rosidl_runtime_rs::Sequence<f32>,

}



impl Default for GetLEDBrightness_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetLEDBrightness_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetLEDBrightness_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLEDBrightness_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLEDBrightness_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLEDBrightness_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetLEDBrightness_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLEDBrightness_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLEDBrightness_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetLEDBrightness_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetLEDBrightness_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetLEDBrightness_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetLEDBrightness_Request__init(msg: *mut SetLEDBrightness_Request) -> bool;
    fn spot_msgs__srv__SetLEDBrightness_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLEDBrightness_Request>, size: usize) -> bool;
    fn spot_msgs__srv__SetLEDBrightness_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLEDBrightness_Request>);
    fn spot_msgs__srv__SetLEDBrightness_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLEDBrightness_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLEDBrightness_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__SetLEDBrightness_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLEDBrightness_Request {
    /// In range [0, 1]
    pub brightness: f32,

}



impl Default for SetLEDBrightness_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetLEDBrightness_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetLEDBrightness_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLEDBrightness_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLEDBrightness_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLEDBrightness_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLEDBrightness_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLEDBrightness_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLEDBrightness_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetLEDBrightness_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetLEDBrightness_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetLEDBrightness_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetLEDBrightness_Response__init(msg: *mut SetLEDBrightness_Response) -> bool;
    fn spot_msgs__srv__SetLEDBrightness_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetLEDBrightness_Response>, size: usize) -> bool;
    fn spot_msgs__srv__SetLEDBrightness_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetLEDBrightness_Response>);
    fn spot_msgs__srv__SetLEDBrightness_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetLEDBrightness_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetLEDBrightness_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__SetLEDBrightness_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLEDBrightness_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetLEDBrightness_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetLEDBrightness_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetLEDBrightness_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetLEDBrightness_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLEDBrightness_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLEDBrightness_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetLEDBrightness_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetLEDBrightness_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetLEDBrightness_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetLEDBrightness_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetLEDBrightness_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavUploadGraph_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GraphNavUploadGraph_Request__init(msg: *mut GraphNavUploadGraph_Request) -> bool;
    fn spot_msgs__srv__GraphNavUploadGraph_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GraphNavUploadGraph_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GraphNavUploadGraph_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GraphNavUploadGraph_Request>);
    fn spot_msgs__srv__GraphNavUploadGraph_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GraphNavUploadGraph_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GraphNavUploadGraph_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GraphNavUploadGraph_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavUploadGraph_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub upload_filepath: rosidl_runtime_rs::String,

}



impl Default for GraphNavUploadGraph_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GraphNavUploadGraph_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GraphNavUploadGraph_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GraphNavUploadGraph_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavUploadGraph_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavUploadGraph_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavUploadGraph_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GraphNavUploadGraph_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GraphNavUploadGraph_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GraphNavUploadGraph_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavUploadGraph_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavUploadGraph_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GraphNavUploadGraph_Response__init(msg: *mut GraphNavUploadGraph_Response) -> bool;
    fn spot_msgs__srv__GraphNavUploadGraph_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GraphNavUploadGraph_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GraphNavUploadGraph_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GraphNavUploadGraph_Response>);
    fn spot_msgs__srv__GraphNavUploadGraph_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GraphNavUploadGraph_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GraphNavUploadGraph_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GraphNavUploadGraph_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavUploadGraph_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for GraphNavUploadGraph_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GraphNavUploadGraph_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GraphNavUploadGraph_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GraphNavUploadGraph_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavUploadGraph_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavUploadGraph_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavUploadGraph_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GraphNavUploadGraph_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GraphNavUploadGraph_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GraphNavUploadGraph_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavUploadGraph_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavClearGraph_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GraphNavClearGraph_Request__init(msg: *mut GraphNavClearGraph_Request) -> bool;
    fn spot_msgs__srv__GraphNavClearGraph_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GraphNavClearGraph_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GraphNavClearGraph_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GraphNavClearGraph_Request>);
    fn spot_msgs__srv__GraphNavClearGraph_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GraphNavClearGraph_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GraphNavClearGraph_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GraphNavClearGraph_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavClearGraph_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GraphNavClearGraph_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GraphNavClearGraph_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GraphNavClearGraph_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GraphNavClearGraph_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavClearGraph_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavClearGraph_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavClearGraph_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GraphNavClearGraph_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GraphNavClearGraph_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GraphNavClearGraph_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavClearGraph_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavClearGraph_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GraphNavClearGraph_Response__init(msg: *mut GraphNavClearGraph_Response) -> bool;
    fn spot_msgs__srv__GraphNavClearGraph_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GraphNavClearGraph_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GraphNavClearGraph_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GraphNavClearGraph_Response>);
    fn spot_msgs__srv__GraphNavClearGraph_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GraphNavClearGraph_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GraphNavClearGraph_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GraphNavClearGraph_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavClearGraph_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for GraphNavClearGraph_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GraphNavClearGraph_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GraphNavClearGraph_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GraphNavClearGraph_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavClearGraph_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavClearGraph_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavClearGraph_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GraphNavClearGraph_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GraphNavClearGraph_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GraphNavClearGraph_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavClearGraph_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavSetLocalization_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GraphNavSetLocalization_Request__init(msg: *mut GraphNavSetLocalization_Request) -> bool;
    fn spot_msgs__srv__GraphNavSetLocalization_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GraphNavSetLocalization_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GraphNavSetLocalization_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GraphNavSetLocalization_Request>);
    fn spot_msgs__srv__GraphNavSetLocalization_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GraphNavSetLocalization_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GraphNavSetLocalization_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GraphNavSetLocalization_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavSetLocalization_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub method: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub waypoint_id: rosidl_runtime_rs::String,

}



impl Default for GraphNavSetLocalization_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GraphNavSetLocalization_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GraphNavSetLocalization_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GraphNavSetLocalization_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavSetLocalization_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavSetLocalization_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavSetLocalization_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GraphNavSetLocalization_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GraphNavSetLocalization_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GraphNavSetLocalization_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavSetLocalization_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavSetLocalization_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GraphNavSetLocalization_Response__init(msg: *mut GraphNavSetLocalization_Response) -> bool;
    fn spot_msgs__srv__GraphNavSetLocalization_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GraphNavSetLocalization_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GraphNavSetLocalization_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GraphNavSetLocalization_Response>);
    fn spot_msgs__srv__GraphNavSetLocalization_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GraphNavSetLocalization_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GraphNavSetLocalization_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GraphNavSetLocalization_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavSetLocalization_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for GraphNavSetLocalization_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GraphNavSetLocalization_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GraphNavSetLocalization_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GraphNavSetLocalization_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavSetLocalization_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavSetLocalization_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavSetLocalization_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GraphNavSetLocalization_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GraphNavSetLocalization_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GraphNavSetLocalization_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavSetLocalization_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavGetLocalizationPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GraphNavGetLocalizationPose_Request__init(msg: *mut GraphNavGetLocalizationPose_Request) -> bool;
    fn spot_msgs__srv__GraphNavGetLocalizationPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GraphNavGetLocalizationPose_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GraphNavGetLocalizationPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GraphNavGetLocalizationPose_Request>);
    fn spot_msgs__srv__GraphNavGetLocalizationPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GraphNavGetLocalizationPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GraphNavGetLocalizationPose_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GraphNavGetLocalizationPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavGetLocalizationPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GraphNavGetLocalizationPose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GraphNavGetLocalizationPose_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GraphNavGetLocalizationPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GraphNavGetLocalizationPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavGetLocalizationPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavGetLocalizationPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavGetLocalizationPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GraphNavGetLocalizationPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GraphNavGetLocalizationPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GraphNavGetLocalizationPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavGetLocalizationPose_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavGetLocalizationPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GraphNavGetLocalizationPose_Response__init(msg: *mut GraphNavGetLocalizationPose_Response) -> bool;
    fn spot_msgs__srv__GraphNavGetLocalizationPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GraphNavGetLocalizationPose_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GraphNavGetLocalizationPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GraphNavGetLocalizationPose_Response>);
    fn spot_msgs__srv__GraphNavGetLocalizationPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GraphNavGetLocalizationPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GraphNavGetLocalizationPose_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GraphNavGetLocalizationPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavGetLocalizationPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for GraphNavGetLocalizationPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GraphNavGetLocalizationPose_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GraphNavGetLocalizationPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GraphNavGetLocalizationPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavGetLocalizationPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavGetLocalizationPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GraphNavGetLocalizationPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GraphNavGetLocalizationPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GraphNavGetLocalizationPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GraphNavGetLocalizationPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GraphNavGetLocalizationPose_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__Dock_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__Dock_Request__init(msg: *mut Dock_Request) -> bool;
    fn spot_msgs__srv__Dock_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dock_Request>, size: usize) -> bool;
    fn spot_msgs__srv__Dock_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dock_Request>);
    fn spot_msgs__srv__Dock_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dock_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Dock_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__Dock_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub dock_id: i16,

}



impl Default for Dock_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__Dock_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__Dock_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dock_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__Dock_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__Dock_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__Dock_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dock_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dock_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/Dock_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__Dock_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__Dock_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__Dock_Response__init(msg: *mut Dock_Response) -> bool;
    fn spot_msgs__srv__Dock_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dock_Response>, size: usize) -> bool;
    fn spot_msgs__srv__Dock_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dock_Response>);
    fn spot_msgs__srv__Dock_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dock_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Dock_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__Dock_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for Dock_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__Dock_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__Dock_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dock_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__Dock_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__Dock_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__Dock_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dock_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dock_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/Dock_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__Dock_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetGripperCameraParameters_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetGripperCameraParameters_Request__init(msg: *mut GetGripperCameraParameters_Request) -> bool;
    fn spot_msgs__srv__GetGripperCameraParameters_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetGripperCameraParameters_Request>, size: usize) -> bool;
    fn spot_msgs__srv__GetGripperCameraParameters_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetGripperCameraParameters_Request>);
    fn spot_msgs__srv__GetGripperCameraParameters_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetGripperCameraParameters_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetGripperCameraParameters_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__GetGripperCameraParameters_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGripperCameraParameters_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bosdyn_api_msgs::msg::rmw::GripperCameraGetParamRequest,

}



impl Default for GetGripperCameraParameters_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetGripperCameraParameters_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetGripperCameraParameters_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetGripperCameraParameters_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetGripperCameraParameters_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetGripperCameraParameters_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetGripperCameraParameters_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetGripperCameraParameters_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetGripperCameraParameters_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetGripperCameraParameters_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetGripperCameraParameters_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetGripperCameraParameters_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__GetGripperCameraParameters_Response__init(msg: *mut GetGripperCameraParameters_Response) -> bool;
    fn spot_msgs__srv__GetGripperCameraParameters_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetGripperCameraParameters_Response>, size: usize) -> bool;
    fn spot_msgs__srv__GetGripperCameraParameters_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetGripperCameraParameters_Response>);
    fn spot_msgs__srv__GetGripperCameraParameters_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetGripperCameraParameters_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetGripperCameraParameters_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__GetGripperCameraParameters_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGripperCameraParameters_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: bosdyn_api_msgs::msg::rmw::GripperCameraGetParamResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for GetGripperCameraParameters_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__GetGripperCameraParameters_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__GetGripperCameraParameters_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetGripperCameraParameters_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetGripperCameraParameters_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetGripperCameraParameters_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__GetGripperCameraParameters_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetGripperCameraParameters_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetGripperCameraParameters_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/GetGripperCameraParameters_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__GetGripperCameraParameters_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetGripperCameraParameters_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetGripperCameraParameters_Request__init(msg: *mut SetGripperCameraParameters_Request) -> bool;
    fn spot_msgs__srv__SetGripperCameraParameters_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetGripperCameraParameters_Request>, size: usize) -> bool;
    fn spot_msgs__srv__SetGripperCameraParameters_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetGripperCameraParameters_Request>);
    fn spot_msgs__srv__SetGripperCameraParameters_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetGripperCameraParameters_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetGripperCameraParameters_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__SetGripperCameraParameters_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGripperCameraParameters_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bosdyn_api_msgs::msg::rmw::GripperCameraParamRequest,

}



impl Default for SetGripperCameraParameters_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetGripperCameraParameters_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetGripperCameraParameters_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetGripperCameraParameters_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperCameraParameters_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperCameraParameters_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperCameraParameters_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetGripperCameraParameters_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetGripperCameraParameters_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetGripperCameraParameters_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetGripperCameraParameters_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetGripperCameraParameters_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetGripperCameraParameters_Response__init(msg: *mut SetGripperCameraParameters_Response) -> bool;
    fn spot_msgs__srv__SetGripperCameraParameters_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetGripperCameraParameters_Response>, size: usize) -> bool;
    fn spot_msgs__srv__SetGripperCameraParameters_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetGripperCameraParameters_Response>);
    fn spot_msgs__srv__SetGripperCameraParameters_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetGripperCameraParameters_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetGripperCameraParameters_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__SetGripperCameraParameters_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGripperCameraParameters_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: bosdyn_api_msgs::msg::rmw::GripperCameraParamResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetGripperCameraParameters_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetGripperCameraParameters_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetGripperCameraParameters_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetGripperCameraParameters_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperCameraParameters_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperCameraParameters_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetGripperCameraParameters_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetGripperCameraParameters_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetGripperCameraParameters_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetGripperCameraParameters_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetGripperCameraParameters_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__OverrideGraspOrCarry_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__OverrideGraspOrCarry_Request__init(msg: *mut OverrideGraspOrCarry_Request) -> bool;
    fn spot_msgs__srv__OverrideGraspOrCarry_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<OverrideGraspOrCarry_Request>, size: usize) -> bool;
    fn spot_msgs__srv__OverrideGraspOrCarry_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<OverrideGraspOrCarry_Request>);
    fn spot_msgs__srv__OverrideGraspOrCarry_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<OverrideGraspOrCarry_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<OverrideGraspOrCarry_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__OverrideGraspOrCarry_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OverrideGraspOrCarry_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub grasp_override: bosdyn_api_msgs::msg::rmw::ApiGraspOverrideOverride,


    // This member is not documented.
    #[allow(missing_docs)]
    pub carry_override: bosdyn_api_msgs::msg::rmw::ManipulatorStateCarryState,

}



impl Default for OverrideGraspOrCarry_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__OverrideGraspOrCarry_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__OverrideGraspOrCarry_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for OverrideGraspOrCarry_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__OverrideGraspOrCarry_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__OverrideGraspOrCarry_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__OverrideGraspOrCarry_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for OverrideGraspOrCarry_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for OverrideGraspOrCarry_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/OverrideGraspOrCarry_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__OverrideGraspOrCarry_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__OverrideGraspOrCarry_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__OverrideGraspOrCarry_Response__init(msg: *mut OverrideGraspOrCarry_Response) -> bool;
    fn spot_msgs__srv__OverrideGraspOrCarry_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<OverrideGraspOrCarry_Response>, size: usize) -> bool;
    fn spot_msgs__srv__OverrideGraspOrCarry_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<OverrideGraspOrCarry_Response>);
    fn spot_msgs__srv__OverrideGraspOrCarry_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<OverrideGraspOrCarry_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<OverrideGraspOrCarry_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__OverrideGraspOrCarry_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OverrideGraspOrCarry_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for OverrideGraspOrCarry_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__OverrideGraspOrCarry_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__OverrideGraspOrCarry_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for OverrideGraspOrCarry_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__OverrideGraspOrCarry_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__OverrideGraspOrCarry_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__OverrideGraspOrCarry_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for OverrideGraspOrCarry_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for OverrideGraspOrCarry_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/OverrideGraspOrCarry_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__OverrideGraspOrCarry_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetStandHeight_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetStandHeight_Request__init(msg: *mut SetStandHeight_Request) -> bool;
    fn spot_msgs__srv__SetStandHeight_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetStandHeight_Request>, size: usize) -> bool;
    fn spot_msgs__srv__SetStandHeight_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetStandHeight_Request>);
    fn spot_msgs__srv__SetStandHeight_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetStandHeight_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetStandHeight_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__SetStandHeight_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetStandHeight_Request {
    /// In range [-0.15, 0.15]
    pub height: f32,

}



impl Default for SetStandHeight_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetStandHeight_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetStandHeight_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetStandHeight_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStandHeight_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStandHeight_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStandHeight_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetStandHeight_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetStandHeight_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetStandHeight_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetStandHeight_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetStandHeight_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetStandHeight_Response__init(msg: *mut SetStandHeight_Response) -> bool;
    fn spot_msgs__srv__SetStandHeight_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetStandHeight_Response>, size: usize) -> bool;
    fn spot_msgs__srv__SetStandHeight_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetStandHeight_Response>);
    fn spot_msgs__srv__SetStandHeight_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetStandHeight_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetStandHeight_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__SetStandHeight_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetStandHeight_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetStandHeight_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetStandHeight_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetStandHeight_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetStandHeight_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStandHeight_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStandHeight_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStandHeight_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetStandHeight_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetStandHeight_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetStandHeight_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetStandHeight_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetStairsMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetStairsMode_Request__init(msg: *mut SetStairsMode_Request) -> bool;
    fn spot_msgs__srv__SetStairsMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetStairsMode_Request>, size: usize) -> bool;
    fn spot_msgs__srv__SetStairsMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetStairsMode_Request>);
    fn spot_msgs__srv__SetStairsMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetStairsMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetStairsMode_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__SetStairsMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetStairsMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stairs_mode: bosdyn_spot_api_msgs::msg::rmw::MobilityParamsStairsMode,

}



impl Default for SetStairsMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetStairsMode_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetStairsMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetStairsMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStairsMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStairsMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStairsMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetStairsMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetStairsMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetStairsMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetStairsMode_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetStairsMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__SetStairsMode_Response__init(msg: *mut SetStairsMode_Response) -> bool;
    fn spot_msgs__srv__SetStairsMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetStairsMode_Response>, size: usize) -> bool;
    fn spot_msgs__srv__SetStairsMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetStairsMode_Response>);
    fn spot_msgs__srv__SetStairsMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetStairsMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetStairsMode_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__SetStairsMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetStairsMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetStairsMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__SetStairsMode_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__SetStairsMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetStairsMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStairsMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStairsMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__SetStairsMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetStairsMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetStairsMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/SetStairsMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__SetStairsMode_Response() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__MutateWorldObject_Request() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__MutateWorldObject_Request__init(msg: *mut MutateWorldObject_Request) -> bool;
    fn spot_msgs__srv__MutateWorldObject_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MutateWorldObject_Request>, size: usize) -> bool;
    fn spot_msgs__srv__MutateWorldObject_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MutateWorldObject_Request>);
    fn spot_msgs__srv__MutateWorldObject_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MutateWorldObject_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MutateWorldObject_Request>) -> bool;
}

// Corresponds to spot_msgs__srv__MutateWorldObject_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MutateWorldObject_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bosdyn_api_msgs::msg::rmw::MutateWorldObjectRequest,

}



impl Default for MutateWorldObject_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__MutateWorldObject_Request__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__MutateWorldObject_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MutateWorldObject_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__MutateWorldObject_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__MutateWorldObject_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__MutateWorldObject_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MutateWorldObject_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MutateWorldObject_Request where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/MutateWorldObject_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__MutateWorldObject_Request() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__MutateWorldObject_Response() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__srv__MutateWorldObject_Response__init(msg: *mut MutateWorldObject_Response) -> bool;
    fn spot_msgs__srv__MutateWorldObject_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MutateWorldObject_Response>, size: usize) -> bool;
    fn spot_msgs__srv__MutateWorldObject_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MutateWorldObject_Response>);
    fn spot_msgs__srv__MutateWorldObject_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MutateWorldObject_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MutateWorldObject_Response>) -> bool;
}

// Corresponds to spot_msgs__srv__MutateWorldObject_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MutateWorldObject_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: bosdyn_api_msgs::msg::rmw::MutateWorldObjectResponse,

}



impl Default for MutateWorldObject_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__srv__MutateWorldObject_Response__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__srv__MutateWorldObject_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MutateWorldObject_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__MutateWorldObject_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__MutateWorldObject_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__srv__MutateWorldObject_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MutateWorldObject_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MutateWorldObject_Response where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/srv/MutateWorldObject_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__srv__MutateWorldObject_Response() }
  }
}






#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__AcquireLease() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__AcquireLease
#[allow(missing_docs, non_camel_case_types)]
pub struct AcquireLease;

impl rosidl_runtime_rs::Service for AcquireLease {
    type Request = AcquireLease_Request;
    type Response = AcquireLease_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__AcquireLease() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ChoreographyRecordedStateToAnimation() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ChoreographyRecordedStateToAnimation
#[allow(missing_docs, non_camel_case_types)]
pub struct ChoreographyRecordedStateToAnimation;

impl rosidl_runtime_rs::Service for ChoreographyRecordedStateToAnimation {
    type Request = ChoreographyRecordedStateToAnimation_Request;
    type Response = ChoreographyRecordedStateToAnimation_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ChoreographyRecordedStateToAnimation() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ChoreographyStartRecordingState() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ChoreographyStartRecordingState
#[allow(missing_docs, non_camel_case_types)]
pub struct ChoreographyStartRecordingState;

impl rosidl_runtime_rs::Service for ChoreographyStartRecordingState {
    type Request = ChoreographyStartRecordingState_Request;
    type Response = ChoreographyStartRecordingState_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ChoreographyStartRecordingState() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ChoreographyStopRecordingState() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ChoreographyStopRecordingState
#[allow(missing_docs, non_camel_case_types)]
pub struct ChoreographyStopRecordingState;

impl rosidl_runtime_rs::Service for ChoreographyStopRecordingState {
    type Request = ChoreographyStopRecordingState_Request;
    type Response = ChoreographyStopRecordingState_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ChoreographyStopRecordingState() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetChoreographyStatus() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GetChoreographyStatus
#[allow(missing_docs, non_camel_case_types)]
pub struct GetChoreographyStatus;

impl rosidl_runtime_rs::Service for GetChoreographyStatus {
    type Request = GetChoreographyStatus_Request;
    type Response = GetChoreographyStatus_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetChoreographyStatus() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetInverseKinematicSolutions() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GetInverseKinematicSolutions
#[allow(missing_docs, non_camel_case_types)]
pub struct GetInverseKinematicSolutions;

impl rosidl_runtime_rs::Service for GetInverseKinematicSolutions {
    type Request = GetInverseKinematicSolutions_Request;
    type Response = GetInverseKinematicSolutions_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetInverseKinematicSolutions() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListGraph() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ListGraph
#[allow(missing_docs, non_camel_case_types)]
pub struct ListGraph;

impl rosidl_runtime_rs::Service for ListGraph {
    type Request = ListGraph_Request;
    type Response = ListGraph_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListGraph() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListWorldObjects() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ListWorldObjects
#[allow(missing_docs, non_camel_case_types)]
pub struct ListWorldObjects;

impl rosidl_runtime_rs::Service for ListWorldObjects {
    type Request = ListWorldObjects_Request;
    type Response = ListWorldObjects_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListWorldObjects() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ReturnLease() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ReturnLease
#[allow(missing_docs, non_camel_case_types)]
pub struct ReturnLease;

impl rosidl_runtime_rs::Service for ReturnLease {
    type Request = ReturnLease_Request;
    type Response = ReturnLease_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ReturnLease() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetLocomotion() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__SetLocomotion
#[allow(missing_docs, non_camel_case_types)]
pub struct SetLocomotion;

impl rosidl_runtime_rs::Service for SetLocomotion {
    type Request = SetLocomotion_Request;
    type Response = SetLocomotion_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetLocomotion() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetVelocity() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__SetVelocity
#[allow(missing_docs, non_camel_case_types)]
pub struct SetVelocity;

impl rosidl_runtime_rs::Service for SetVelocity {
    type Request = SetVelocity_Request;
    type Response = SetVelocity_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetVelocity() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListAllDances() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ListAllDances
#[allow(missing_docs, non_camel_case_types)]
pub struct ListAllDances;

impl rosidl_runtime_rs::Service for ListAllDances {
    type Request = ListAllDances_Request;
    type Response = ListAllDances_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListAllDances() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListAllMoves() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ListAllMoves
#[allow(missing_docs, non_camel_case_types)]
pub struct ListAllMoves;

impl rosidl_runtime_rs::Service for ListAllMoves {
    type Request = ListAllMoves_Request;
    type Response = ListAllMoves_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListAllMoves() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__UploadAnimation() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__UploadAnimation
#[allow(missing_docs, non_camel_case_types)]
pub struct UploadAnimation;

impl rosidl_runtime_rs::Service for UploadAnimation {
    type Request = UploadAnimation_Request;
    type Response = UploadAnimation_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__UploadAnimation() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__UploadSequence() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__UploadSequence
#[allow(missing_docs, non_camel_case_types)]
pub struct UploadSequence;

impl rosidl_runtime_rs::Service for UploadSequence {
    type Request = UploadSequence_Request;
    type Response = UploadSequence_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__UploadSequence() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ClearBehaviorFault() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ClearBehaviorFault
#[allow(missing_docs, non_camel_case_types)]
pub struct ClearBehaviorFault;

impl rosidl_runtime_rs::Service for ClearBehaviorFault {
    type Request = ClearBehaviorFault_Request;
    type Response = ClearBehaviorFault_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ClearBehaviorFault() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListSounds() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ListSounds
#[allow(missing_docs, non_camel_case_types)]
pub struct ListSounds;

impl rosidl_runtime_rs::Service for ListSounds {
    type Request = ListSounds_Request;
    type Response = ListSounds_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListSounds() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__LoadSound() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__LoadSound
#[allow(missing_docs, non_camel_case_types)]
pub struct LoadSound;

impl rosidl_runtime_rs::Service for LoadSound {
    type Request = LoadSound_Request;
    type Response = LoadSound_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__LoadSound() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__PlaySound() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__PlaySound
#[allow(missing_docs, non_camel_case_types)]
pub struct PlaySound;

impl rosidl_runtime_rs::Service for PlaySound {
    type Request = PlaySound_Request;
    type Response = PlaySound_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__PlaySound() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__DeleteSound() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__DeleteSound
#[allow(missing_docs, non_camel_case_types)]
pub struct DeleteSound;

impl rosidl_runtime_rs::Service for DeleteSound {
    type Request = DeleteSound_Request;
    type Response = DeleteSound_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__DeleteSound() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetVolume() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GetVolume
#[allow(missing_docs, non_camel_case_types)]
pub struct GetVolume;

impl rosidl_runtime_rs::Service for GetVolume {
    type Request = GetVolume_Request;
    type Response = GetVolume_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetVolume() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetVolume() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__SetVolume
#[allow(missing_docs, non_camel_case_types)]
pub struct SetVolume;

impl rosidl_runtime_rs::Service for SetVolume {
    type Request = SetVolume_Request;
    type Response = SetVolume_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetVolume() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListPtz() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ListPtz
#[allow(missing_docs, non_camel_case_types)]
pub struct ListPtz;

impl rosidl_runtime_rs::Service for ListPtz {
    type Request = ListPtz_Request;
    type Response = ListPtz_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListPtz() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetPtzPosition() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GetPtzPosition
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPtzPosition;

impl rosidl_runtime_rs::Service for GetPtzPosition {
    type Request = GetPtzPosition_Request;
    type Response = GetPtzPosition_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetPtzPosition() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetPtzPosition() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__SetPtzPosition
#[allow(missing_docs, non_camel_case_types)]
pub struct SetPtzPosition;

impl rosidl_runtime_rs::Service for SetPtzPosition {
    type Request = SetPtzPosition_Request;
    type Response = SetPtzPosition_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetPtzPosition() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__InitializeLens() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__InitializeLens
#[allow(missing_docs, non_camel_case_types)]
pub struct InitializeLens;

impl rosidl_runtime_rs::Service for InitializeLens {
    type Request = InitializeLens_Request;
    type Response = InitializeLens_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__InitializeLens() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__DeleteLogpoint() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__DeleteLogpoint
#[allow(missing_docs, non_camel_case_types)]
pub struct DeleteLogpoint;

impl rosidl_runtime_rs::Service for DeleteLogpoint {
    type Request = DeleteLogpoint_Request;
    type Response = DeleteLogpoint_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__DeleteLogpoint() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetLogpointStatus() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GetLogpointStatus
#[allow(missing_docs, non_camel_case_types)]
pub struct GetLogpointStatus;

impl rosidl_runtime_rs::Service for GetLogpointStatus {
    type Request = GetLogpointStatus_Request;
    type Response = GetLogpointStatus_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetLogpointStatus() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListCameras() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ListCameras
#[allow(missing_docs, non_camel_case_types)]
pub struct ListCameras;

impl rosidl_runtime_rs::Service for ListCameras {
    type Request = ListCameras_Request;
    type Response = ListCameras_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListCameras() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListLogpoints() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__ListLogpoints
#[allow(missing_docs, non_camel_case_types)]
pub struct ListLogpoints;

impl rosidl_runtime_rs::Service for ListLogpoints {
    type Request = ListLogpoints_Request;
    type Response = ListLogpoints_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__ListLogpoints() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__RetrieveLogpoint() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__RetrieveLogpoint
#[allow(missing_docs, non_camel_case_types)]
pub struct RetrieveLogpoint;

impl rosidl_runtime_rs::Service for RetrieveLogpoint {
    type Request = RetrieveLogpoint_Request;
    type Response = RetrieveLogpoint_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__RetrieveLogpoint() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__RobotCommand() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__RobotCommand
#[allow(missing_docs, non_camel_case_types)]
pub struct RobotCommand;

impl rosidl_runtime_rs::Service for RobotCommand {
    type Request = RobotCommand_Request;
    type Response = RobotCommand_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__RobotCommand() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetGripperAngle() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__SetGripperAngle
#[allow(missing_docs, non_camel_case_types)]
pub struct SetGripperAngle;

impl rosidl_runtime_rs::Service for SetGripperAngle {
    type Request = SetGripperAngle_Request;
    type Response = SetGripperAngle_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetGripperAngle() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__StoreLogpoint() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__StoreLogpoint
#[allow(missing_docs, non_camel_case_types)]
pub struct StoreLogpoint;

impl rosidl_runtime_rs::Service for StoreLogpoint {
    type Request = StoreLogpoint_Request;
    type Response = StoreLogpoint_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__StoreLogpoint() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__TagLogpoint() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__TagLogpoint
#[allow(missing_docs, non_camel_case_types)]
pub struct TagLogpoint;

impl rosidl_runtime_rs::Service for TagLogpoint {
    type Request = TagLogpoint_Request;
    type Response = TagLogpoint_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__TagLogpoint() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetLEDBrightness() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GetLEDBrightness
#[allow(missing_docs, non_camel_case_types)]
pub struct GetLEDBrightness;

impl rosidl_runtime_rs::Service for GetLEDBrightness {
    type Request = GetLEDBrightness_Request;
    type Response = GetLEDBrightness_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetLEDBrightness() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetLEDBrightness() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__SetLEDBrightness
#[allow(missing_docs, non_camel_case_types)]
pub struct SetLEDBrightness;

impl rosidl_runtime_rs::Service for SetLEDBrightness {
    type Request = SetLEDBrightness_Request;
    type Response = SetLEDBrightness_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetLEDBrightness() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GraphNavUploadGraph() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GraphNavUploadGraph
#[allow(missing_docs, non_camel_case_types)]
pub struct GraphNavUploadGraph;

impl rosidl_runtime_rs::Service for GraphNavUploadGraph {
    type Request = GraphNavUploadGraph_Request;
    type Response = GraphNavUploadGraph_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GraphNavUploadGraph() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GraphNavClearGraph() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GraphNavClearGraph
#[allow(missing_docs, non_camel_case_types)]
pub struct GraphNavClearGraph;

impl rosidl_runtime_rs::Service for GraphNavClearGraph {
    type Request = GraphNavClearGraph_Request;
    type Response = GraphNavClearGraph_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GraphNavClearGraph() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GraphNavSetLocalization() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GraphNavSetLocalization
#[allow(missing_docs, non_camel_case_types)]
pub struct GraphNavSetLocalization;

impl rosidl_runtime_rs::Service for GraphNavSetLocalization {
    type Request = GraphNavSetLocalization_Request;
    type Response = GraphNavSetLocalization_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GraphNavSetLocalization() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GraphNavGetLocalizationPose() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GraphNavGetLocalizationPose
#[allow(missing_docs, non_camel_case_types)]
pub struct GraphNavGetLocalizationPose;

impl rosidl_runtime_rs::Service for GraphNavGetLocalizationPose {
    type Request = GraphNavGetLocalizationPose_Request;
    type Response = GraphNavGetLocalizationPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GraphNavGetLocalizationPose() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__Dock() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__Dock
#[allow(missing_docs, non_camel_case_types)]
pub struct Dock;

impl rosidl_runtime_rs::Service for Dock {
    type Request = Dock_Request;
    type Response = Dock_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__Dock() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetGripperCameraParameters() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__GetGripperCameraParameters
#[allow(missing_docs, non_camel_case_types)]
pub struct GetGripperCameraParameters;

impl rosidl_runtime_rs::Service for GetGripperCameraParameters {
    type Request = GetGripperCameraParameters_Request;
    type Response = GetGripperCameraParameters_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__GetGripperCameraParameters() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetGripperCameraParameters() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__SetGripperCameraParameters
#[allow(missing_docs, non_camel_case_types)]
pub struct SetGripperCameraParameters;

impl rosidl_runtime_rs::Service for SetGripperCameraParameters {
    type Request = SetGripperCameraParameters_Request;
    type Response = SetGripperCameraParameters_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetGripperCameraParameters() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__OverrideGraspOrCarry() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__OverrideGraspOrCarry
#[allow(missing_docs, non_camel_case_types)]
pub struct OverrideGraspOrCarry;

impl rosidl_runtime_rs::Service for OverrideGraspOrCarry {
    type Request = OverrideGraspOrCarry_Request;
    type Response = OverrideGraspOrCarry_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__OverrideGraspOrCarry() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetStandHeight() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__SetStandHeight
#[allow(missing_docs, non_camel_case_types)]
pub struct SetStandHeight;

impl rosidl_runtime_rs::Service for SetStandHeight {
    type Request = SetStandHeight_Request;
    type Response = SetStandHeight_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetStandHeight() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetStairsMode() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__SetStairsMode
#[allow(missing_docs, non_camel_case_types)]
pub struct SetStairsMode;

impl rosidl_runtime_rs::Service for SetStairsMode {
    type Request = SetStairsMode_Request;
    type Response = SetStairsMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__SetStairsMode() }
    }
}




#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__MutateWorldObject() -> *const std::ffi::c_void;
}

// Corresponds to spot_msgs__srv__MutateWorldObject
#[allow(missing_docs, non_camel_case_types)]
pub struct MutateWorldObject;

impl rosidl_runtime_rs::Service for MutateWorldObject {
    type Request = MutateWorldObject_Request;
    type Response = MutateWorldObject_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__spot_msgs__srv__MutateWorldObject() }
    }
}


