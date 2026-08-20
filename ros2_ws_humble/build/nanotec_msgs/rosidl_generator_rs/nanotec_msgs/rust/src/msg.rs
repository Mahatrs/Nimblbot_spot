#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to nanotec_msgs__msg__DriverStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriverStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub devices: Vec<super::msg::DeviceStatus>,

}



impl Default for DriverStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DriverStatus::default())
  }
}

impl rosidl_runtime_rs::Message for DriverStatus {
  type RmwMsg = super::msg::rmw::DriverStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        devices: msg.devices
          .into_iter()
          .map(|elem| super::msg::DeviceStatus::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        devices: msg.devices
          .iter()
          .map(|elem| super::msg::DeviceStatus::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      devices: msg.devices
          .into_iter()
          .map(super::msg::DeviceStatus::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to nanotec_msgs__msg__DeviceStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeviceStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DeviceStatus::default())
  }
}

impl rosidl_runtime_rs::Message for DeviceStatus {
  type RmwMsg = super::msg::rmw::DeviceStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        voltage_power: msg.voltage_power,
        voltage_logic: msg.voltage_logic,
        temperature_motor: msg.temperature_motor,
        temperature_micro_chip: msg.temperature_micro_chip,
        ready_to_switch_on: msg.ready_to_switch_on,
        switched_on: msg.switched_on,
        operation_enabled: msg.operation_enabled,
        fault: msg.fault,
        voltage_enabled: msg.voltage_enabled,
        quick_stop: msg.quick_stop,
        switch_on_disabled: msg.switch_on_disabled,
        warning: msg.warning,
        target_reached: msg.target_reached,
        internal_limit_active: msg.internal_limit_active,
        operation_mode_specific: msg.operation_mode_specific,
        homing_status: msg.homing_status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      voltage_power: msg.voltage_power,
      voltage_logic: msg.voltage_logic,
      temperature_motor: msg.temperature_motor,
      temperature_micro_chip: msg.temperature_micro_chip,
      ready_to_switch_on: msg.ready_to_switch_on,
      switched_on: msg.switched_on,
      operation_enabled: msg.operation_enabled,
      fault: msg.fault,
      voltage_enabled: msg.voltage_enabled,
      quick_stop: msg.quick_stop,
      switch_on_disabled: msg.switch_on_disabled,
      warning: msg.warning,
      target_reached: msg.target_reached,
      internal_limit_active: msg.internal_limit_active,
      operation_mode_specific: msg.operation_mode_specific,
      homing_status: msg.homing_status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      voltage_power: msg.voltage_power,
      voltage_logic: msg.voltage_logic,
      temperature_motor: msg.temperature_motor,
      temperature_micro_chip: msg.temperature_micro_chip,
      ready_to_switch_on: msg.ready_to_switch_on,
      switched_on: msg.switched_on,
      operation_enabled: msg.operation_enabled,
      fault: msg.fault,
      voltage_enabled: msg.voltage_enabled,
      quick_stop: msg.quick_stop,
      switch_on_disabled: msg.switch_on_disabled,
      warning: msg.warning,
      target_reached: msg.target_reached,
      internal_limit_active: msg.internal_limit_active,
      operation_mode_specific: msg.operation_mode_specific,
      homing_status: msg.homing_status,
    }
  }
}


