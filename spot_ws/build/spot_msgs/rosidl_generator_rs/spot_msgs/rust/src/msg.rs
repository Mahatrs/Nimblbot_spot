#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to spot_msgs__msg__BatteryStateArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BatteryStateArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub battery_states: Vec<super::msg::BatteryState>,

}



impl Default for BatteryStateArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BatteryStateArray::default())
  }
}

impl rosidl_runtime_rs::Message for BatteryStateArray {
  type RmwMsg = super::msg::rmw::BatteryStateArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        battery_states: msg.battery_states
          .into_iter()
          .map(|elem| super::msg::BatteryState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        battery_states: msg.battery_states
          .iter()
          .map(|elem| super::msg::BatteryState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      battery_states: msg.battery_states
          .into_iter()
          .map(super::msg::BatteryState::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__msg__BehaviorFault
/// Cause

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BehaviorFault {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BehaviorFault::default())
  }
}

impl rosidl_runtime_rs::Message for BehaviorFault {
  type RmwMsg = super::msg::rmw::BehaviorFault;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        behavior_fault_id: msg.behavior_fault_id,
        cause: msg.cause,
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      behavior_fault_id: msg.behavior_fault_id,
      cause: msg.cause,
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      behavior_fault_id: msg.behavior_fault_id,
      cause: msg.cause,
      status: msg.status,
    }
  }
}


// Corresponds to spot_msgs__msg__EStopStateArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EStopStateArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub estop_states: Vec<super::msg::EStopState>,

}



impl Default for EStopStateArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::EStopStateArray::default())
  }
}

impl rosidl_runtime_rs::Message for EStopStateArray {
  type RmwMsg = super::msg::rmw::EStopStateArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        estop_states: msg.estop_states
          .into_iter()
          .map(|elem| super::msg::EStopState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        estop_states: msg.estop_states
          .iter()
          .map(|elem| super::msg::EStopState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      estop_states: msg.estop_states
          .into_iter()
          .map(super::msg::EStopState::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__msg__FootStateArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FootStateArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub states: Vec<super::msg::FootState>,

}



impl Default for FootStateArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FootStateArray::default())
  }
}

impl rosidl_runtime_rs::Message for FootStateArray {
  type RmwMsg = super::msg::rmw::FootStateArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        states: msg.states
          .into_iter()
          .map(|elem| super::msg::FootState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        states: msg.states
          .iter()
          .map(|elem| super::msg::FootState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      states: msg.states
          .into_iter()
          .map(super::msg::FootState::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__msg__LeaseArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LeaseArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub resources: Vec<super::msg::LeaseResource>,

}



impl Default for LeaseArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LeaseArray::default())
  }
}

impl rosidl_runtime_rs::Message for LeaseArray {
  type RmwMsg = super::msg::rmw::LeaseArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        resources: msg.resources
          .into_iter()
          .map(|elem| super::msg::LeaseResource::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        resources: msg.resources
          .iter()
          .map(|elem| super::msg::LeaseResource::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      resources: msg.resources
          .into_iter()
          .map(super::msg::LeaseResource::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__msg__LeaseOwner

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LeaseOwner {

    // This member is not documented.
    #[allow(missing_docs)]
    pub client_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub user_name: std::string::String,

}



impl Default for LeaseOwner {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LeaseOwner::default())
  }
}

impl rosidl_runtime_rs::Message for LeaseOwner {
  type RmwMsg = super::msg::rmw::LeaseOwner;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        client_name: msg.client_name.as_str().into(),
        user_name: msg.user_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        client_name: msg.client_name.as_str().into(),
        user_name: msg.user_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      client_name: msg.client_name.to_string(),
      user_name: msg.user_name.to_string(),
    }
  }
}


// Corresponds to spot_msgs__msg__Metrics

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Metrics {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gait_cycles: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub time_moving: builtin_interfaces::msg::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub electric_power: builtin_interfaces::msg::Duration,

}



impl Default for Metrics {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Metrics::default())
  }
}

impl rosidl_runtime_rs::Message for Metrics {
  type RmwMsg = super::msg::rmw::Metrics;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        distance: msg.distance,
        gait_cycles: msg.gait_cycles,
        time_moving: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.time_moving)).into_owned(),
        electric_power: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.electric_power)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      distance: msg.distance,
      gait_cycles: msg.gait_cycles,
        time_moving: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.time_moving)).into_owned(),
        electric_power: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.electric_power)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      distance: msg.distance,
      gait_cycles: msg.gait_cycles,
      time_moving: builtin_interfaces::msg::Duration::from_rmw_message(msg.time_moving),
      electric_power: builtin_interfaces::msg::Duration::from_rmw_message(msg.electric_power),
    }
  }
}


// Corresponds to spot_msgs__msg__MobilityParams

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MobilityParams {

    // This member is not documented.
    #[allow(missing_docs)]
    pub body_control: geometry_msgs::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub locomotion_hint: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stairs_mode: u32,

}



impl Default for MobilityParams {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MobilityParams::default())
  }
}

impl rosidl_runtime_rs::Message for MobilityParams {
  type RmwMsg = super::msg::rmw::MobilityParams;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        body_control: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.body_control)).into_owned(),
        locomotion_hint: msg.locomotion_hint,
        stairs_mode: msg.stairs_mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        body_control: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.body_control)).into_owned(),
      locomotion_hint: msg.locomotion_hint,
      stairs_mode: msg.stairs_mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      body_control: geometry_msgs::msg::Pose::from_rmw_message(msg.body_control),
      locomotion_hint: msg.locomotion_hint,
      stairs_mode: msg.stairs_mode,
    }
  }
}


// Corresponds to spot_msgs__msg__SystemFault
/// Severity

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SystemFault {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub code: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub attributes: Vec<std::string::String>,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SystemFault::default())
  }
}

impl rosidl_runtime_rs::Message for SystemFault {
  type RmwMsg = super::msg::rmw::SystemFault;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        name: msg.name.as_str().into(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.duration)).into_owned(),
        code: msg.code,
        uuid: msg.uuid.as_str().into(),
        error_message: msg.error_message.as_str().into(),
        attributes: msg.attributes
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        severity: msg.severity,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        name: msg.name.as_str().into(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.duration)).into_owned(),
      code: msg.code,
        uuid: msg.uuid.as_str().into(),
        error_message: msg.error_message.as_str().into(),
        attributes: msg.attributes
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      severity: msg.severity,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      name: msg.name.to_string(),
      duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.duration),
      code: msg.code,
      uuid: msg.uuid.to_string(),
      error_message: msg.error_message.to_string(),
      attributes: msg.attributes
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      severity: msg.severity,
    }
  }
}


// Corresponds to spot_msgs__msg__WiFiState
/// Mode

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WiFiState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub essid: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WiFiState::default())
  }
}

impl rosidl_runtime_rs::Message for WiFiState {
  type RmwMsg = super::msg::rmw::WiFiState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_mode: msg.current_mode,
        essid: msg.essid.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      current_mode: msg.current_mode,
        essid: msg.essid.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_mode: msg.current_mode,
      essid: msg.essid.to_string(),
    }
  }
}


// Corresponds to spot_msgs__msg__BatteryState
/// Status

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BatteryState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub identifier: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub charge_percentage: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub estimated_runtime: builtin_interfaces::msg::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub voltage: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperatures: Vec<f64>,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BatteryState::default())
  }
}

impl rosidl_runtime_rs::Message for BatteryState {
  type RmwMsg = super::msg::rmw::BatteryState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        identifier: msg.identifier.as_str().into(),
        charge_percentage: msg.charge_percentage,
        estimated_runtime: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.estimated_runtime)).into_owned(),
        current: msg.current,
        voltage: msg.voltage,
        temperatures: msg.temperatures.into(),
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        identifier: msg.identifier.as_str().into(),
      charge_percentage: msg.charge_percentage,
        estimated_runtime: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.estimated_runtime)).into_owned(),
      current: msg.current,
      voltage: msg.voltage,
        temperatures: msg.temperatures.as_slice().into(),
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      identifier: msg.identifier.to_string(),
      charge_percentage: msg.charge_percentage,
      estimated_runtime: builtin_interfaces::msg::Duration::from_rmw_message(msg.estimated_runtime),
      current: msg.current,
      voltage: msg.voltage,
      temperatures: msg.temperatures
          .into_iter()
          .collect(),
      status: msg.status,
    }
  }
}


// Corresponds to spot_msgs__msg__BehaviorFaultState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BehaviorFaultState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub faults: Vec<super::msg::BehaviorFault>,

}



impl Default for BehaviorFaultState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BehaviorFaultState::default())
  }
}

impl rosidl_runtime_rs::Message for BehaviorFaultState {
  type RmwMsg = super::msg::rmw::BehaviorFaultState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        faults: msg.faults
          .into_iter()
          .map(|elem| super::msg::BehaviorFault::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        faults: msg.faults
          .iter()
          .map(|elem| super::msg::BehaviorFault::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      faults: msg.faults
          .into_iter()
          .map(super::msg::BehaviorFault::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__msg__EStopState
/// Type

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EStopState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state_description: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::EStopState::default())
  }
}

impl rosidl_runtime_rs::Message for EStopState {
  type RmwMsg = super::msg::rmw::EStopState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        name: msg.name.as_str().into(),
        type_: msg.type_,
        state: msg.state,
        state_description: msg.state_description.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        name: msg.name.as_str().into(),
      type_: msg.type_,
      state: msg.state,
        state_description: msg.state_description.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      name: msg.name.to_string(),
      type_: msg.type_,
      state: msg.state,
      state_description: msg.state_description.to_string(),
    }
  }
}


// Corresponds to spot_msgs__msg__Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub serial_number: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub species: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub version: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub nickname: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub computer_serial_number: std::string::String,

}



impl Default for Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for Feedback {
  type RmwMsg = super::msg::rmw::Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        standing: msg.standing,
        sitting: msg.sitting,
        moving: msg.moving,
        serial_number: msg.serial_number.as_str().into(),
        species: msg.species.as_str().into(),
        version: msg.version.as_str().into(),
        nickname: msg.nickname.as_str().into(),
        computer_serial_number: msg.computer_serial_number.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      standing: msg.standing,
      sitting: msg.sitting,
      moving: msg.moving,
        serial_number: msg.serial_number.as_str().into(),
        species: msg.species.as_str().into(),
        version: msg.version.as_str().into(),
        nickname: msg.nickname.as_str().into(),
        computer_serial_number: msg.computer_serial_number.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      standing: msg.standing,
      sitting: msg.sitting,
      moving: msg.moving,
      serial_number: msg.serial_number.to_string(),
      species: msg.species.to_string(),
      version: msg.version.to_string(),
      nickname: msg.nickname.to_string(),
      computer_serial_number: msg.computer_serial_number.to_string(),
    }
  }
}


// Corresponds to spot_msgs__msg__FootState
/// Contact

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FootState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_position_rt_body: geometry_msgs::msg::Point,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FootState::default())
  }
}

impl rosidl_runtime_rs::Message for FootState {
  type RmwMsg = super::msg::rmw::FootState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        foot_position_rt_body: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.foot_position_rt_body)).into_owned(),
        contact: msg.contact,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        foot_position_rt_body: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.foot_position_rt_body)).into_owned(),
      contact: msg.contact,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      foot_position_rt_body: geometry_msgs::msg::Point::from_rmw_message(msg.foot_position_rt_body),
      contact: msg.contact,
    }
  }
}


// Corresponds to spot_msgs__msg__JointCommand
/// list of the joint names to control

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: Vec<std::string::String>,

    /// desired position commands for each joint in rad
    pub position: Vec<f64>,

    /// desired velocity commands for each joint in rad/s
    pub velocity: Vec<f64>,

    /// desired effort commands for each joint in Nm
    pub effort: Vec<f64>,

    /// desired k_q_p commands for each joint in Nm/rad
    pub k_q_p: Vec<f64>,

    /// desired k_qd_p command for each joint in Nms/rad
    pub k_qd_p: Vec<f64>,

}



impl Default for JointCommand {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointCommand::default())
  }
}

impl rosidl_runtime_rs::Message for JointCommand {
  type RmwMsg = super::msg::rmw::JointCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        position: msg.position.into(),
        velocity: msg.velocity.into(),
        effort: msg.effort.into(),
        k_q_p: msg.k_q_p.into(),
        k_qd_p: msg.k_qd_p.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        position: msg.position.as_slice().into(),
        velocity: msg.velocity.as_slice().into(),
        effort: msg.effort.as_slice().into(),
        k_q_p: msg.k_q_p.as_slice().into(),
        k_qd_p: msg.k_qd_p.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      position: msg.position
          .into_iter()
          .collect(),
      velocity: msg.velocity
          .into_iter()
          .collect(),
      effort: msg.effort
          .into_iter()
          .collect(),
      k_q_p: msg.k_q_p
          .into_iter()
          .collect(),
      k_qd_p: msg.k_qd_p
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__msg__Lease

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Lease {

    // This member is not documented.
    #[allow(missing_docs)]
    pub resource: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub epoch: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence: Vec<u32>,

}



impl Default for Lease {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Lease::default())
  }
}

impl rosidl_runtime_rs::Message for Lease {
  type RmwMsg = super::msg::rmw::Lease;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        resource: msg.resource.as_str().into(),
        epoch: msg.epoch.as_str().into(),
        sequence: msg.sequence.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        resource: msg.resource.as_str().into(),
        epoch: msg.epoch.as_str().into(),
        sequence: msg.sequence.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      resource: msg.resource.to_string(),
      epoch: msg.epoch.to_string(),
      sequence: msg.sequence
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__msg__LeaseResource

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LeaseResource {

    // This member is not documented.
    #[allow(missing_docs)]
    pub resource: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lease: super::msg::Lease,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lease_owner: super::msg::LeaseOwner,

}



impl Default for LeaseResource {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LeaseResource::default())
  }
}

impl rosidl_runtime_rs::Message for LeaseResource {
  type RmwMsg = super::msg::rmw::LeaseResource;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        resource: msg.resource.as_str().into(),
        lease: super::msg::Lease::into_rmw_message(std::borrow::Cow::Owned(msg.lease)).into_owned(),
        lease_owner: super::msg::LeaseOwner::into_rmw_message(std::borrow::Cow::Owned(msg.lease_owner)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        resource: msg.resource.as_str().into(),
        lease: super::msg::Lease::into_rmw_message(std::borrow::Cow::Borrowed(&msg.lease)).into_owned(),
        lease_owner: super::msg::LeaseOwner::into_rmw_message(std::borrow::Cow::Borrowed(&msg.lease_owner)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      resource: msg.resource.to_string(),
      lease: super::msg::Lease::from_rmw_message(msg.lease),
      lease_owner: super::msg::LeaseOwner::from_rmw_message(msg.lease_owner),
    }
  }
}


// Corresponds to spot_msgs__msg__PowerState
/// MotorPowerState

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PowerState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    pub locomotion_estimated_runtime: builtin_interfaces::msg::Duration,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PowerState::default())
  }
}

impl rosidl_runtime_rs::Message for PowerState {
  type RmwMsg = super::msg::rmw::PowerState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        motor_power_state: msg.motor_power_state,
        shore_power_state: msg.shore_power_state,
        locomotion_charge_percentage: msg.locomotion_charge_percentage,
        locomotion_estimated_runtime: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.locomotion_estimated_runtime)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      motor_power_state: msg.motor_power_state,
      shore_power_state: msg.shore_power_state,
      locomotion_charge_percentage: msg.locomotion_charge_percentage,
        locomotion_estimated_runtime: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.locomotion_estimated_runtime)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      motor_power_state: msg.motor_power_state,
      shore_power_state: msg.shore_power_state,
      locomotion_charge_percentage: msg.locomotion_charge_percentage,
      locomotion_estimated_runtime: builtin_interfaces::msg::Duration::from_rmw_message(msg.locomotion_estimated_runtime),
    }
  }
}


// Corresponds to spot_msgs__msg__SystemFaultState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SystemFaultState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub faults: Vec<super::msg::SystemFault>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub historical_faults: Vec<super::msg::SystemFault>,

}



impl Default for SystemFaultState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SystemFaultState::default())
  }
}

impl rosidl_runtime_rs::Message for SystemFaultState {
  type RmwMsg = super::msg::rmw::SystemFaultState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        faults: msg.faults
          .into_iter()
          .map(|elem| super::msg::SystemFault::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        historical_faults: msg.historical_faults
          .into_iter()
          .map(|elem| super::msg::SystemFault::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        faults: msg.faults
          .iter()
          .map(|elem| super::msg::SystemFault::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        historical_faults: msg.historical_faults
          .iter()
          .map(|elem| super::msg::SystemFault::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      faults: msg.faults
          .into_iter()
          .map(super::msg::SystemFault::from_rmw_message)
          .collect(),
      historical_faults: msg.historical_faults
          .into_iter()
          .map(super::msg::SystemFault::from_rmw_message)
          .collect(),
    }
  }
}


