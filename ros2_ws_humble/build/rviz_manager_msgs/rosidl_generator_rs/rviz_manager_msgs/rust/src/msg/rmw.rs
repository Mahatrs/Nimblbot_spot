#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "rviz_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rviz_manager_msgs__msg__ManagerLaunch() -> *const std::ffi::c_void;
}

#[link(name = "rviz_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn rviz_manager_msgs__msg__ManagerLaunch__init(msg: *mut ManagerLaunch) -> bool;
    fn rviz_manager_msgs__msg__ManagerLaunch__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ManagerLaunch>, size: usize) -> bool;
    fn rviz_manager_msgs__msg__ManagerLaunch__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ManagerLaunch>);
    fn rviz_manager_msgs__msg__ManagerLaunch__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ManagerLaunch>, out_seq: *mut rosidl_runtime_rs::Sequence<ManagerLaunch>) -> bool;
}

// Corresponds to rviz_manager_msgs__msg__ManagerLaunch
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ManagerLaunch {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// unique identifier
    pub id: i32,

    /// "start", "stop", "restart", "status"
    pub action: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ns: rosidl_runtime_rs::String,

    /// true = session ros, false = custom cmd
    pub bash_session: bool,

    /// true = launch file, false = node
    pub is_launch_file: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub package: rosidl_runtime_rs::String,

    /// node or launch file
    pub executable: rosidl_runtime_rs::String,

    /// CLI args
    pub arguments: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// --ros-args ...
    pub ros_arguments: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// optional
    pub working_dir: rosidl_runtime_rs::String,

    /// tmux session name (optional override)
    pub session_name: rosidl_runtime_rs::String,

    /// optional flag
    pub use_sim_time: bool,

    /// optional (sec)
    pub timeout: i32,

}



impl Default for ManagerLaunch {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !rviz_manager_msgs__msg__ManagerLaunch__init(&mut msg as *mut _) {
        panic!("Call to rviz_manager_msgs__msg__ManagerLaunch__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ManagerLaunch {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { rviz_manager_msgs__msg__ManagerLaunch__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { rviz_manager_msgs__msg__ManagerLaunch__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { rviz_manager_msgs__msg__ManagerLaunch__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ManagerLaunch {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ManagerLaunch where Self: Sized {
  const TYPE_NAME: &'static str = "rviz_manager_msgs/msg/ManagerLaunch";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__rviz_manager_msgs__msg__ManagerLaunch() }
  }
}


#[link(name = "rviz_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rviz_manager_msgs__msg__ManagerStatus() -> *const std::ffi::c_void;
}

#[link(name = "rviz_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn rviz_manager_msgs__msg__ManagerStatus__init(msg: *mut ManagerStatus) -> bool;
    fn rviz_manager_msgs__msg__ManagerStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ManagerStatus>, size: usize) -> bool;
    fn rviz_manager_msgs__msg__ManagerStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ManagerStatus>);
    fn rviz_manager_msgs__msg__ManagerStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ManagerStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<ManagerStatus>) -> bool;
}

// Corresponds to rviz_manager_msgs__msg__ManagerStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ManagerStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i32,

    /// "running", "stopped", "error"
    pub status: rosidl_runtime_rs::String,

    /// logs or error
    pub message: rosidl_runtime_rs::String,

}



impl Default for ManagerStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !rviz_manager_msgs__msg__ManagerStatus__init(&mut msg as *mut _) {
        panic!("Call to rviz_manager_msgs__msg__ManagerStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ManagerStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { rviz_manager_msgs__msg__ManagerStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { rviz_manager_msgs__msg__ManagerStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { rviz_manager_msgs__msg__ManagerStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ManagerStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ManagerStatus where Self: Sized {
  const TYPE_NAME: &'static str = "rviz_manager_msgs/msg/ManagerStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__rviz_manager_msgs__msg__ManagerStatus() }
  }
}


