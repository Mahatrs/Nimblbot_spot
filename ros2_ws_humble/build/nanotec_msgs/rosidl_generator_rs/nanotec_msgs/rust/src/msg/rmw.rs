#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "nanotec_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__nanotec_msgs__msg__DriverStatus() -> *const std::ffi::c_void;
}

#[link(name = "nanotec_msgs__rosidl_generator_c")]
extern "C" {
    fn nanotec_msgs__msg__DriverStatus__init(msg: *mut DriverStatus) -> bool;
    fn nanotec_msgs__msg__DriverStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriverStatus>, size: usize) -> bool;
    fn nanotec_msgs__msg__DriverStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriverStatus>);
    fn nanotec_msgs__msg__DriverStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriverStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<DriverStatus>) -> bool;
}

// Corresponds to nanotec_msgs__msg__DriverStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriverStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub devices: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DeviceStatus>,

}



impl Default for DriverStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !nanotec_msgs__msg__DriverStatus__init(&mut msg as *mut _) {
        panic!("Call to nanotec_msgs__msg__DriverStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriverStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { nanotec_msgs__msg__DriverStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { nanotec_msgs__msg__DriverStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { nanotec_msgs__msg__DriverStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriverStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriverStatus where Self: Sized {
  const TYPE_NAME: &'static str = "nanotec_msgs/msg/DriverStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__nanotec_msgs__msg__DriverStatus() }
  }
}


#[link(name = "nanotec_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__nanotec_msgs__msg__DeviceStatus() -> *const std::ffi::c_void;
}

#[link(name = "nanotec_msgs__rosidl_generator_c")]
extern "C" {
    fn nanotec_msgs__msg__DeviceStatus__init(msg: *mut DeviceStatus) -> bool;
    fn nanotec_msgs__msg__DeviceStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeviceStatus>, size: usize) -> bool;
    fn nanotec_msgs__msg__DeviceStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeviceStatus>);
    fn nanotec_msgs__msg__DeviceStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeviceStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<DeviceStatus>) -> bool;
}

// Corresponds to nanotec_msgs__msg__DeviceStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeviceStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub voltage_power: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub voltage_logic: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature_motor: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature_micro_chip: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ready_to_switch_on: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub switched_on: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub operation_enabled: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub fault: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub voltage_enabled: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub quick_stop: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub switch_on_disabled: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub warning: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_reached: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub internal_limit_active: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub operation_mode_specific: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub homing_status: i32,

}



impl Default for DeviceStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !nanotec_msgs__msg__DeviceStatus__init(&mut msg as *mut _) {
        panic!("Call to nanotec_msgs__msg__DeviceStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeviceStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { nanotec_msgs__msg__DeviceStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { nanotec_msgs__msg__DeviceStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { nanotec_msgs__msg__DeviceStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeviceStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeviceStatus where Self: Sized {
  const TYPE_NAME: &'static str = "nanotec_msgs/msg/DeviceStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__nanotec_msgs__msg__DeviceStatus() }
  }
}


