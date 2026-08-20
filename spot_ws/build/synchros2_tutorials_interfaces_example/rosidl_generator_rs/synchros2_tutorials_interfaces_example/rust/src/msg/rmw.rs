#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "synchros2_tutorials_interfaces_example__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__msg__String() -> *const std::ffi::c_void;
}

#[link(name = "synchros2_tutorials_interfaces_example__rosidl_generator_c")]
extern "C" {
    fn synchros2_tutorials_interfaces_example__msg__String__init(msg: *mut String) -> bool;
    fn synchros2_tutorials_interfaces_example__msg__String__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<String>, size: usize) -> bool;
    fn synchros2_tutorials_interfaces_example__msg__String__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<String>);
    fn synchros2_tutorials_interfaces_example__msg__String__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<String>, out_seq: *mut rosidl_runtime_rs::Sequence<String>) -> bool;
}

// Corresponds to synchros2_tutorials_interfaces_example__msg__String
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct String {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::String,

}



impl Default for String {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !synchros2_tutorials_interfaces_example__msg__String__init(&mut msg as *mut _) {
        panic!("Call to synchros2_tutorials_interfaces_example__msg__String__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for String {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__msg__String__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__msg__String__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { synchros2_tutorials_interfaces_example__msg__String__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for String {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for String where Self: Sized {
  const TYPE_NAME: &'static str = "synchros2_tutorials_interfaces_example/msg/String";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__synchros2_tutorials_interfaces_example__msg__String() }
  }
}


