#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__msg__TestArray() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__msg__TestArray__init(msg: *mut TestArray) -> bool;
    fn ros_babel_fish_test_msgs__msg__TestArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TestArray>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__msg__TestArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TestArray>);
    fn ros_babel_fish_test_msgs__msg__TestArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TestArray>, out_seq: *mut rosidl_runtime_rs::Sequence<TestArray>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__msg__TestArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TestArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub bools: rosidl_runtime_rs::Sequence<bool>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub uint8s: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub uint16s: [u16; 32],


    // This member is not documented.
    #[allow(missing_docs)]
    pub uint32s: rosidl_runtime_rs::Sequence<u32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub uint64s: rosidl_runtime_rs::Sequence<u64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub int8s: rosidl_runtime_rs::Sequence<i8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub int16s: rosidl_runtime_rs::Sequence<i16>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub int32s: rosidl_runtime_rs::Sequence<i32>,

    /// Comment
    pub int64s: [i64; 32],


    // This member is not documented.
    #[allow(missing_docs)]
    pub float32s: rosidl_runtime_rs::Sequence<f32>,

    /// Bounded array
    pub float64s: rosidl_runtime_rs::BoundedSequence<f64, 16>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub times: rosidl_runtime_rs::Sequence<builtin_interfaces::msg::rmw::Time>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub durations: [builtin_interfaces::msg::rmw::Duration; 12],


    // This member is not documented.
    #[allow(missing_docs)]
    pub strings: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub subarrays_fixed: [super::super::msg::rmw::TestSubArray; 10],


    // This member is not documented.
    #[allow(missing_docs)]
    pub subarrays: rosidl_runtime_rs::Sequence<super::super::msg::rmw::TestSubArray>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub subarray: super::super::msg::rmw::TestSubArray,

}



impl Default for TestArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__msg__TestArray__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__msg__TestArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TestArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__msg__TestArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__msg__TestArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__msg__TestArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TestArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TestArray where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/msg/TestArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__msg__TestArray() }
  }
}


#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__msg__TestMessage() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__msg__TestMessage__init(msg: *mut TestMessage) -> bool;
    fn ros_babel_fish_test_msgs__msg__TestMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TestMessage>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__msg__TestMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TestMessage>);
    fn ros_babel_fish_test_msgs__msg__TestMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TestMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<TestMessage>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__msg__TestMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TestMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ui8: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ui16: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ui32: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ui64: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i8: i8,

    /// With default value
    pub i16: i16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i32: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i64: i64,

    /// Comment
    pub f32: f32,

    /// Also a comment but closer
    pub f64: f64,

    /// Two comment signs # and a third
    pub str: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bounded_str: rosidl_runtime_rs::BoundedString<12>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub t: builtin_interfaces::msg::rmw::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: builtin_interfaces::msg::rmw::Duration,

    /// more comment
    pub point_arr: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Point>,

}



impl Default for TestMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__msg__TestMessage__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__msg__TestMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TestMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__msg__TestMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__msg__TestMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__msg__TestMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TestMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TestMessage where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/msg/TestMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__msg__TestMessage() }
  }
}


#[link(name = "ros_babel_fish_test_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__msg__TestSubArray() -> *const std::ffi::c_void;
}

#[link(name = "ros_babel_fish_test_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_babel_fish_test_msgs__msg__TestSubArray__init(msg: *mut TestSubArray) -> bool;
    fn ros_babel_fish_test_msgs__msg__TestSubArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TestSubArray>, size: usize) -> bool;
    fn ros_babel_fish_test_msgs__msg__TestSubArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TestSubArray>);
    fn ros_babel_fish_test_msgs__msg__TestSubArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TestSubArray>, out_seq: *mut rosidl_runtime_rs::Sequence<TestSubArray>) -> bool;
}

// Corresponds to ros_babel_fish_test_msgs__msg__TestSubArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TestSubArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ints: rosidl_runtime_rs::Sequence<i32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub strings: rosidl_runtime_rs::BoundedSequence<rosidl_runtime_rs::String, 10>,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub times: [builtin_interfaces::msg::rmw::Time; 42],


    // This member is not documented.
    #[allow(missing_docs)]
    pub floats: [f64; 12],

}



impl Default for TestSubArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_babel_fish_test_msgs__msg__TestSubArray__init(&mut msg as *mut _) {
        panic!("Call to ros_babel_fish_test_msgs__msg__TestSubArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TestSubArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__msg__TestSubArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__msg__TestSubArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_babel_fish_test_msgs__msg__TestSubArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TestSubArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TestSubArray where Self: Sized {
  const TYPE_NAME: &'static str = "ros_babel_fish_test_msgs/msg/TestSubArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_babel_fish_test_msgs__msg__TestSubArray() }
  }
}


