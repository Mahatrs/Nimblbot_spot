#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__BatteryStateArray() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__BatteryStateArray__init(msg: *mut BatteryStateArray) -> bool;
    fn spot_msgs__msg__BatteryStateArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BatteryStateArray>, size: usize) -> bool;
    fn spot_msgs__msg__BatteryStateArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BatteryStateArray>);
    fn spot_msgs__msg__BatteryStateArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BatteryStateArray>, out_seq: *mut rosidl_runtime_rs::Sequence<BatteryStateArray>) -> bool;
}

// Corresponds to spot_msgs__msg__BatteryStateArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BatteryStateArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub battery_states: rosidl_runtime_rs::Sequence<super::super::msg::rmw::BatteryState>,

}



impl Default for BatteryStateArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__BatteryStateArray__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__BatteryStateArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BatteryStateArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BatteryStateArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BatteryStateArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BatteryStateArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BatteryStateArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BatteryStateArray where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/BatteryStateArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__BatteryStateArray() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__BehaviorFault() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__BehaviorFault__init(msg: *mut BehaviorFault) -> bool;
    fn spot_msgs__msg__BehaviorFault__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BehaviorFault>, size: usize) -> bool;
    fn spot_msgs__msg__BehaviorFault__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BehaviorFault>);
    fn spot_msgs__msg__BehaviorFault__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BehaviorFault>, out_seq: *mut rosidl_runtime_rs::Sequence<BehaviorFault>) -> bool;
}

// Corresponds to spot_msgs__msg__BehaviorFault
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Cause

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BehaviorFault {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub behavior_fault_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cause: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u8,

}

impl BehaviorFault {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CAUSE_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CAUSE_FALL: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CAUSE_HARDWARE: u8 = 2;

    /// Status
    pub const STATUS_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_CLEARABLE: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_UNCLEARABLE: u8 = 2;

}


impl Default for BehaviorFault {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__BehaviorFault__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__BehaviorFault__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BehaviorFault {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BehaviorFault__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BehaviorFault__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BehaviorFault__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BehaviorFault {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BehaviorFault where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/BehaviorFault";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__BehaviorFault() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__EStopStateArray() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__EStopStateArray__init(msg: *mut EStopStateArray) -> bool;
    fn spot_msgs__msg__EStopStateArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EStopStateArray>, size: usize) -> bool;
    fn spot_msgs__msg__EStopStateArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EStopStateArray>);
    fn spot_msgs__msg__EStopStateArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EStopStateArray>, out_seq: *mut rosidl_runtime_rs::Sequence<EStopStateArray>) -> bool;
}

// Corresponds to spot_msgs__msg__EStopStateArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EStopStateArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub estop_states: rosidl_runtime_rs::Sequence<super::super::msg::rmw::EStopState>,

}



impl Default for EStopStateArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__EStopStateArray__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__EStopStateArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EStopStateArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__EStopStateArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__EStopStateArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__EStopStateArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EStopStateArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EStopStateArray where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/EStopStateArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__EStopStateArray() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__FootStateArray() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__FootStateArray__init(msg: *mut FootStateArray) -> bool;
    fn spot_msgs__msg__FootStateArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FootStateArray>, size: usize) -> bool;
    fn spot_msgs__msg__FootStateArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FootStateArray>);
    fn spot_msgs__msg__FootStateArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FootStateArray>, out_seq: *mut rosidl_runtime_rs::Sequence<FootStateArray>) -> bool;
}

// Corresponds to spot_msgs__msg__FootStateArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FootStateArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub states: rosidl_runtime_rs::Sequence<super::super::msg::rmw::FootState>,

}



impl Default for FootStateArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__FootStateArray__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__FootStateArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FootStateArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__FootStateArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__FootStateArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__FootStateArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FootStateArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FootStateArray where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/FootStateArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__FootStateArray() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__LeaseArray() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__LeaseArray__init(msg: *mut LeaseArray) -> bool;
    fn spot_msgs__msg__LeaseArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LeaseArray>, size: usize) -> bool;
    fn spot_msgs__msg__LeaseArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LeaseArray>);
    fn spot_msgs__msg__LeaseArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LeaseArray>, out_seq: *mut rosidl_runtime_rs::Sequence<LeaseArray>) -> bool;
}

// Corresponds to spot_msgs__msg__LeaseArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LeaseArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub resources: rosidl_runtime_rs::Sequence<super::super::msg::rmw::LeaseResource>,

}



impl Default for LeaseArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__LeaseArray__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__LeaseArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LeaseArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__LeaseArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__LeaseArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__LeaseArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LeaseArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LeaseArray where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/LeaseArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__LeaseArray() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__LeaseOwner() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__LeaseOwner__init(msg: *mut LeaseOwner) -> bool;
    fn spot_msgs__msg__LeaseOwner__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LeaseOwner>, size: usize) -> bool;
    fn spot_msgs__msg__LeaseOwner__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LeaseOwner>);
    fn spot_msgs__msg__LeaseOwner__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LeaseOwner>, out_seq: *mut rosidl_runtime_rs::Sequence<LeaseOwner>) -> bool;
}

// Corresponds to spot_msgs__msg__LeaseOwner
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LeaseOwner {

    // This member is not documented.
    #[allow(missing_docs)]
    pub client_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub user_name: rosidl_runtime_rs::String,

}



impl Default for LeaseOwner {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__LeaseOwner__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__LeaseOwner__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LeaseOwner {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__LeaseOwner__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__LeaseOwner__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__LeaseOwner__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LeaseOwner {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LeaseOwner where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/LeaseOwner";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__LeaseOwner() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__Metrics() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__Metrics__init(msg: *mut Metrics) -> bool;
    fn spot_msgs__msg__Metrics__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Metrics>, size: usize) -> bool;
    fn spot_msgs__msg__Metrics__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Metrics>);
    fn spot_msgs__msg__Metrics__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Metrics>, out_seq: *mut rosidl_runtime_rs::Sequence<Metrics>) -> bool;
}

// Corresponds to spot_msgs__msg__Metrics
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Metrics {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gait_cycles: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub time_moving: builtin_interfaces::msg::rmw::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub electric_power: builtin_interfaces::msg::rmw::Duration,

}



impl Default for Metrics {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__Metrics__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__Metrics__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Metrics {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__Metrics__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__Metrics__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__Metrics__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Metrics {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Metrics where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/Metrics";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__Metrics() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__MobilityParams() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__MobilityParams__init(msg: *mut MobilityParams) -> bool;
    fn spot_msgs__msg__MobilityParams__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MobilityParams>, size: usize) -> bool;
    fn spot_msgs__msg__MobilityParams__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MobilityParams>);
    fn spot_msgs__msg__MobilityParams__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MobilityParams>, out_seq: *mut rosidl_runtime_rs::Sequence<MobilityParams>) -> bool;
}

// Corresponds to spot_msgs__msg__MobilityParams
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MobilityParams {

    // This member is not documented.
    #[allow(missing_docs)]
    pub body_control: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub locomotion_hint: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stairs_mode: u32,

}



impl Default for MobilityParams {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__MobilityParams__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__MobilityParams__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MobilityParams {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__MobilityParams__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__MobilityParams__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__MobilityParams__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MobilityParams {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MobilityParams where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/MobilityParams";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__MobilityParams() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__SystemFault() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__SystemFault__init(msg: *mut SystemFault) -> bool;
    fn spot_msgs__msg__SystemFault__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SystemFault>, size: usize) -> bool;
    fn spot_msgs__msg__SystemFault__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SystemFault>);
    fn spot_msgs__msg__SystemFault__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SystemFault>, out_seq: *mut rosidl_runtime_rs::Sequence<SystemFault>) -> bool;
}

// Corresponds to spot_msgs__msg__SystemFault
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Severity

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SystemFault {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::rmw::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub code: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub attributes: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub severity: u8,

}

impl SystemFault {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SEVERITY_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SEVERITY_INFO: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SEVERITY_WARN: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SEVERITY_CRITICAL: u8 = 3;

}


impl Default for SystemFault {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__SystemFault__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__SystemFault__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SystemFault {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__SystemFault__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__SystemFault__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__SystemFault__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SystemFault {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SystemFault where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/SystemFault";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__SystemFault() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__WiFiState() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__WiFiState__init(msg: *mut WiFiState) -> bool;
    fn spot_msgs__msg__WiFiState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WiFiState>, size: usize) -> bool;
    fn spot_msgs__msg__WiFiState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WiFiState>);
    fn spot_msgs__msg__WiFiState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WiFiState>, out_seq: *mut rosidl_runtime_rs::Sequence<WiFiState>) -> bool;
}

// Corresponds to spot_msgs__msg__WiFiState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Mode

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WiFiState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub essid: rosidl_runtime_rs::String,

}

impl WiFiState {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MODE_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MODE_ACCESS_POINT: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MODE_CLIENT: u8 = 2;

}


impl Default for WiFiState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__WiFiState__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__WiFiState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WiFiState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__WiFiState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__WiFiState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__WiFiState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WiFiState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WiFiState where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/WiFiState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__WiFiState() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__BatteryState() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__BatteryState__init(msg: *mut BatteryState) -> bool;
    fn spot_msgs__msg__BatteryState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BatteryState>, size: usize) -> bool;
    fn spot_msgs__msg__BatteryState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BatteryState>);
    fn spot_msgs__msg__BatteryState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BatteryState>, out_seq: *mut rosidl_runtime_rs::Sequence<BatteryState>) -> bool;
}

// Corresponds to spot_msgs__msg__BatteryState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Status

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BatteryState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub identifier: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub charge_percentage: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub estimated_runtime: builtin_interfaces::msg::rmw::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub voltage: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperatures: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u8,

}

impl BatteryState {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_MISSING: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_CHARGING: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_DISCHARGING: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_BOOTING: u8 = 4;

}


impl Default for BatteryState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__BatteryState__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__BatteryState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BatteryState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BatteryState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BatteryState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BatteryState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BatteryState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BatteryState where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/BatteryState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__BatteryState() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__BehaviorFaultState() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__BehaviorFaultState__init(msg: *mut BehaviorFaultState) -> bool;
    fn spot_msgs__msg__BehaviorFaultState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BehaviorFaultState>, size: usize) -> bool;
    fn spot_msgs__msg__BehaviorFaultState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BehaviorFaultState>);
    fn spot_msgs__msg__BehaviorFaultState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BehaviorFaultState>, out_seq: *mut rosidl_runtime_rs::Sequence<BehaviorFaultState>) -> bool;
}

// Corresponds to spot_msgs__msg__BehaviorFaultState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BehaviorFaultState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub faults: rosidl_runtime_rs::Sequence<super::super::msg::rmw::BehaviorFault>,

}



impl Default for BehaviorFaultState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__BehaviorFaultState__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__BehaviorFaultState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BehaviorFaultState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BehaviorFaultState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BehaviorFaultState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__BehaviorFaultState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BehaviorFaultState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BehaviorFaultState where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/BehaviorFaultState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__BehaviorFaultState() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__EStopState() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__EStopState__init(msg: *mut EStopState) -> bool;
    fn spot_msgs__msg__EStopState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EStopState>, size: usize) -> bool;
    fn spot_msgs__msg__EStopState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EStopState>);
    fn spot_msgs__msg__EStopState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EStopState>, out_seq: *mut rosidl_runtime_rs::Sequence<EStopState>) -> bool;
}

// Corresponds to spot_msgs__msg__EStopState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Type

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EStopState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state_description: rosidl_runtime_rs::String,

}

impl EStopState {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TYPE_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TYPE_HARDWARE: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TYPE_SOFTWARE: u8 = 2;

    /// State
    pub const STATE_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATE_ESTOPPED: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATE_NOT_ESTOPPED: u8 = 2;

}


impl Default for EStopState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__EStopState__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__EStopState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EStopState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__EStopState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__EStopState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__EStopState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EStopState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EStopState where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/EStopState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__EStopState() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__Feedback() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__Feedback__init(msg: *mut Feedback) -> bool;
    fn spot_msgs__msg__Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Feedback>, size: usize) -> bool;
    fn spot_msgs__msg__Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Feedback>);
    fn spot_msgs__msg__Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Feedback>) -> bool;
}

// Corresponds to spot_msgs__msg__Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub standing: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sitting: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub moving: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub serial_number: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub species: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub version: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub nickname: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub computer_serial_number: rosidl_runtime_rs::String,

}



impl Default for Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__Feedback__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__Feedback() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__FootState() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__FootState__init(msg: *mut FootState) -> bool;
    fn spot_msgs__msg__FootState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FootState>, size: usize) -> bool;
    fn spot_msgs__msg__FootState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FootState>);
    fn spot_msgs__msg__FootState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FootState>, out_seq: *mut rosidl_runtime_rs::Sequence<FootState>) -> bool;
}

// Corresponds to spot_msgs__msg__FootState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Contact

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FootState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_position_rt_body: geometry_msgs::msg::rmw::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub contact: u8,

}

impl FootState {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CONTACT_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CONTACT_MADE: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CONTACT_LOST: u8 = 2;

}


impl Default for FootState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__FootState__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__FootState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FootState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__FootState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__FootState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__FootState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FootState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FootState where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/FootState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__FootState() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__JointCommand() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__JointCommand__init(msg: *mut JointCommand) -> bool;
    fn spot_msgs__msg__JointCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointCommand>, size: usize) -> bool;
    fn spot_msgs__msg__JointCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointCommand>);
    fn spot_msgs__msg__JointCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<JointCommand>) -> bool;
}

// Corresponds to spot_msgs__msg__JointCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// list of the joint names to control

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// desired position commands for each joint in rad
    pub position: rosidl_runtime_rs::Sequence<f64>,

    /// desired velocity commands for each joint in rad/s
    pub velocity: rosidl_runtime_rs::Sequence<f64>,

    /// desired effort commands for each joint in Nm
    pub effort: rosidl_runtime_rs::Sequence<f64>,

    /// desired k_q_p commands for each joint in Nm/rad
    pub k_q_p: rosidl_runtime_rs::Sequence<f64>,

    /// desired k_qd_p command for each joint in Nms/rad
    pub k_qd_p: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for JointCommand {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__JointCommand__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__JointCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__JointCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__JointCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__JointCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointCommand where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/JointCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__JointCommand() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__Lease() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__Lease__init(msg: *mut Lease) -> bool;
    fn spot_msgs__msg__Lease__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Lease>, size: usize) -> bool;
    fn spot_msgs__msg__Lease__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Lease>);
    fn spot_msgs__msg__Lease__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Lease>, out_seq: *mut rosidl_runtime_rs::Sequence<Lease>) -> bool;
}

// Corresponds to spot_msgs__msg__Lease
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Lease {

    // This member is not documented.
    #[allow(missing_docs)]
    pub resource: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub epoch: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence: rosidl_runtime_rs::Sequence<u32>,

}



impl Default for Lease {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__Lease__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__Lease__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Lease {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__Lease__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__Lease__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__Lease__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Lease {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Lease where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/Lease";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__Lease() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__LeaseResource() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__LeaseResource__init(msg: *mut LeaseResource) -> bool;
    fn spot_msgs__msg__LeaseResource__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LeaseResource>, size: usize) -> bool;
    fn spot_msgs__msg__LeaseResource__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LeaseResource>);
    fn spot_msgs__msg__LeaseResource__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LeaseResource>, out_seq: *mut rosidl_runtime_rs::Sequence<LeaseResource>) -> bool;
}

// Corresponds to spot_msgs__msg__LeaseResource
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LeaseResource {

    // This member is not documented.
    #[allow(missing_docs)]
    pub resource: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lease: super::super::msg::rmw::Lease,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lease_owner: super::super::msg::rmw::LeaseOwner,

}



impl Default for LeaseResource {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__LeaseResource__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__LeaseResource__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LeaseResource {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__LeaseResource__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__LeaseResource__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__LeaseResource__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LeaseResource {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LeaseResource where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/LeaseResource";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__LeaseResource() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__PowerState() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__PowerState__init(msg: *mut PowerState) -> bool;
    fn spot_msgs__msg__PowerState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PowerState>, size: usize) -> bool;
    fn spot_msgs__msg__PowerState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PowerState>);
    fn spot_msgs__msg__PowerState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PowerState>, out_seq: *mut rosidl_runtime_rs::Sequence<PowerState>) -> bool;
}

// Corresponds to spot_msgs__msg__PowerState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// MotorPowerState

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PowerState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_power_state: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub shore_power_state: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub locomotion_charge_percentage: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub locomotion_estimated_runtime: builtin_interfaces::msg::rmw::Duration,

}

impl PowerState {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATE_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATE_OFF: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATE_ON: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATE_POWERING_ON: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATE_POWERING_OFF: u8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATE_ERROR: u8 = 5;

    /// ShorePowerState
    pub const STATE_UNKNOWN_SHORE_POWER: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATE_ON_SHORE_POWER: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATE_OFF_SHORE_POWER: u8 = 2;

}


impl Default for PowerState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__PowerState__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__PowerState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PowerState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__PowerState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__PowerState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__PowerState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PowerState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PowerState where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/PowerState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__PowerState() }
  }
}


#[link(name = "spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__SystemFaultState() -> *const std::ffi::c_void;
}

#[link(name = "spot_msgs__rosidl_generator_c")]
extern "C" {
    fn spot_msgs__msg__SystemFaultState__init(msg: *mut SystemFaultState) -> bool;
    fn spot_msgs__msg__SystemFaultState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SystemFaultState>, size: usize) -> bool;
    fn spot_msgs__msg__SystemFaultState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SystemFaultState>);
    fn spot_msgs__msg__SystemFaultState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SystemFaultState>, out_seq: *mut rosidl_runtime_rs::Sequence<SystemFaultState>) -> bool;
}

// Corresponds to spot_msgs__msg__SystemFaultState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SystemFaultState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub faults: rosidl_runtime_rs::Sequence<super::super::msg::rmw::SystemFault>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub historical_faults: rosidl_runtime_rs::Sequence<super::super::msg::rmw::SystemFault>,

}



impl Default for SystemFaultState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !spot_msgs__msg__SystemFaultState__init(&mut msg as *mut _) {
        panic!("Call to spot_msgs__msg__SystemFaultState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SystemFaultState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__SystemFaultState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__SystemFaultState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { spot_msgs__msg__SystemFaultState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SystemFaultState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SystemFaultState where Self: Sized {
  const TYPE_NAME: &'static str = "spot_msgs/msg/SystemFaultState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__spot_msgs__msg__SystemFaultState() }
  }
}


