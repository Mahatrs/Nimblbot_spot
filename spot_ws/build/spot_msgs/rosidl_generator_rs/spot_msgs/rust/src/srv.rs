#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to spot_msgs__srv__AcquireLease_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AcquireLease_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub client_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub resource_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub force: bool,

}



impl Default for AcquireLease_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AcquireLease_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AcquireLease_Request {
  type RmwMsg = super::srv::rmw::AcquireLease_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        client_name: msg.client_name.as_str().into(),
        resource_name: msg.resource_name.as_str().into(),
        force: msg.force,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        client_name: msg.client_name.as_str().into(),
        resource_name: msg.resource_name.as_str().into(),
      force: msg.force,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      client_name: msg.client_name.to_string(),
      resource_name: msg.resource_name.to_string(),
      force: msg.force,
    }
  }
}


// Corresponds to spot_msgs__srv__AcquireLease_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AcquireLease_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lease: bosdyn_api_msgs::msg::Lease,

}



impl Default for AcquireLease_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AcquireLease_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AcquireLease_Response {
  type RmwMsg = super::srv::rmw::AcquireLease_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        lease: bosdyn_api_msgs::msg::Lease::into_rmw_message(std::borrow::Cow::Owned(msg.lease)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        lease: bosdyn_api_msgs::msg::Lease::into_rmw_message(std::borrow::Cow::Borrowed(&msg.lease)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      lease: bosdyn_api_msgs::msg::Lease::from_rmw_message(msg.lease),
    }
  }
}


// Corresponds to spot_msgs__srv__ChoreographyRecordedStateToAnimation_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyRecordedStateToAnimation_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub has_arm: bool,

}



impl Default for ChoreographyRecordedStateToAnimation_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ChoreographyRecordedStateToAnimation_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ChoreographyRecordedStateToAnimation_Request {
  type RmwMsg = super::srv::rmw::ChoreographyRecordedStateToAnimation_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        has_arm: msg.has_arm,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      has_arm: msg.has_arm,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      has_arm: msg.has_arm,
    }
  }
}


// Corresponds to spot_msgs__srv__ChoreographyRecordedStateToAnimation_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyRecordedStateToAnimation_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub animation_file_contents: std::string::String,

}



impl Default for ChoreographyRecordedStateToAnimation_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ChoreographyRecordedStateToAnimation_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ChoreographyRecordedStateToAnimation_Response {
  type RmwMsg = super::srv::rmw::ChoreographyRecordedStateToAnimation_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        animation_file_contents: msg.animation_file_contents.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        animation_file_contents: msg.animation_file_contents.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      animation_file_contents: msg.animation_file_contents.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__ChoreographyStartRecordingState_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ChoreographyStartRecordingState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ChoreographyStartRecordingState_Request {
  type RmwMsg = super::srv::rmw::ChoreographyStartRecordingState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        duration_seconds: msg.duration_seconds,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      duration_seconds: msg.duration_seconds,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      duration_seconds: msg.duration_seconds,
    }
  }
}


// Corresponds to spot_msgs__srv__ChoreographyStartRecordingState_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyStartRecordingState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub recording_session_id: u64,

}



impl Default for ChoreographyStartRecordingState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ChoreographyStartRecordingState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ChoreographyStartRecordingState_Response {
  type RmwMsg = super::srv::rmw::ChoreographyStartRecordingState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        status: msg.status,
        recording_session_id: msg.recording_session_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      status: msg.status,
      recording_session_id: msg.recording_session_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      status: msg.status,
      recording_session_id: msg.recording_session_id,
    }
  }
}


// Corresponds to spot_msgs__srv__ChoreographyStopRecordingState_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyStopRecordingState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ChoreographyStopRecordingState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ChoreographyStopRecordingState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ChoreographyStopRecordingState_Request {
  type RmwMsg = super::srv::rmw::ChoreographyStopRecordingState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__ChoreographyStopRecordingState_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChoreographyStopRecordingState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for ChoreographyStopRecordingState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ChoreographyStopRecordingState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ChoreographyStopRecordingState_Response {
  type RmwMsg = super::srv::rmw::ChoreographyStopRecordingState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GetChoreographyStatus_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetChoreographyStatus_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetChoreographyStatus_Request {
  type RmwMsg = super::srv::rmw::GetChoreographyStatus_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__GetChoreographyStatus_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetChoreographyStatus_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub execution_id: i32,

}



impl Default for GetChoreographyStatus_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetChoreographyStatus_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetChoreographyStatus_Response {
  type RmwMsg = super::srv::rmw::GetChoreographyStatus_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        status: msg.status,
        execution_id: msg.execution_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      status: msg.status,
      execution_id: msg.execution_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      status: msg.status,
      execution_id: msg.execution_id,
    }
  }
}


// Corresponds to spot_msgs__srv__GetInverseKinematicSolutions_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInverseKinematicSolutions_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bosdyn_spot_api_msgs::msg::InverseKinematicsRequest,

}



impl Default for GetInverseKinematicSolutions_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInverseKinematicSolutions_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetInverseKinematicSolutions_Request {
  type RmwMsg = super::srv::rmw::GetInverseKinematicSolutions_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: bosdyn_spot_api_msgs::msg::InverseKinematicsRequest::into_rmw_message(std::borrow::Cow::Owned(msg.request)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: bosdyn_spot_api_msgs::msg::InverseKinematicsRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.request)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      request: bosdyn_spot_api_msgs::msg::InverseKinematicsRequest::from_rmw_message(msg.request),
    }
  }
}


// Corresponds to spot_msgs__srv__GetInverseKinematicSolutions_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInverseKinematicSolutions_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: bosdyn_spot_api_msgs::msg::InverseKinematicsResponse,

}



impl Default for GetInverseKinematicSolutions_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInverseKinematicSolutions_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetInverseKinematicSolutions_Response {
  type RmwMsg = super::srv::rmw::GetInverseKinematicSolutions_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: bosdyn_spot_api_msgs::msg::InverseKinematicsResponse::into_rmw_message(std::borrow::Cow::Owned(msg.response)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: bosdyn_spot_api_msgs::msg::InverseKinematicsResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.response)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      response: bosdyn_spot_api_msgs::msg::InverseKinematicsResponse::from_rmw_message(msg.response),
    }
  }
}


// Corresponds to spot_msgs__srv__ListGraph_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListGraph_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub upload_filepath: std::string::String,

}



impl Default for ListGraph_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListGraph_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListGraph_Request {
  type RmwMsg = super::srv::rmw::ListGraph_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        upload_filepath: msg.upload_filepath.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        upload_filepath: msg.upload_filepath.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      upload_filepath: msg.upload_filepath.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__ListGraph_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListGraph_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub waypoint_ids: Vec<std::string::String>,

}



impl Default for ListGraph_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListGraph_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListGraph_Response {
  type RmwMsg = super::srv::rmw::ListGraph_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        waypoint_ids: msg.waypoint_ids
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        waypoint_ids: msg.waypoint_ids
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      waypoint_ids: msg.waypoint_ids
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__srv__ListWorldObjects_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListWorldObjects_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bosdyn_api_msgs::msg::ListWorldObjectRequest,

}



impl Default for ListWorldObjects_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListWorldObjects_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListWorldObjects_Request {
  type RmwMsg = super::srv::rmw::ListWorldObjects_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: bosdyn_api_msgs::msg::ListWorldObjectRequest::into_rmw_message(std::borrow::Cow::Owned(msg.request)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: bosdyn_api_msgs::msg::ListWorldObjectRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.request)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      request: bosdyn_api_msgs::msg::ListWorldObjectRequest::from_rmw_message(msg.request),
    }
  }
}


// Corresponds to spot_msgs__srv__ListWorldObjects_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListWorldObjects_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: bosdyn_api_msgs::msg::ListWorldObjectResponse,

}



impl Default for ListWorldObjects_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListWorldObjects_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListWorldObjects_Response {
  type RmwMsg = super::srv::rmw::ListWorldObjects_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: bosdyn_api_msgs::msg::ListWorldObjectResponse::into_rmw_message(std::borrow::Cow::Owned(msg.response)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: bosdyn_api_msgs::msg::ListWorldObjectResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.response)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      response: bosdyn_api_msgs::msg::ListWorldObjectResponse::from_rmw_message(msg.response),
    }
  }
}


// Corresponds to spot_msgs__srv__ReturnLease_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReturnLease_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub lease: bosdyn_api_msgs::msg::Lease,

}



impl Default for ReturnLease_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ReturnLease_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ReturnLease_Request {
  type RmwMsg = super::srv::rmw::ReturnLease_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        lease: bosdyn_api_msgs::msg::Lease::into_rmw_message(std::borrow::Cow::Owned(msg.lease)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        lease: bosdyn_api_msgs::msg::Lease::into_rmw_message(std::borrow::Cow::Borrowed(&msg.lease)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      lease: bosdyn_api_msgs::msg::Lease::from_rmw_message(msg.lease),
    }
  }
}


// Corresponds to spot_msgs__srv__ReturnLease_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReturnLease_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for ReturnLease_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ReturnLease_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ReturnLease_Response {
  type RmwMsg = super::srv::rmw::ReturnLease_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__SetLocomotion_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLocomotion_Request {
    /// See https://dev.bostondynamics.com/protos/bosdyn/api/proto_reference.html?highlight=mobilityparams#locomotionhint for details
    pub locomotion_mode: u32,

}



impl Default for SetLocomotion_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetLocomotion_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetLocomotion_Request {
  type RmwMsg = super::srv::rmw::SetLocomotion_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        locomotion_mode: msg.locomotion_mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      locomotion_mode: msg.locomotion_mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      locomotion_mode: msg.locomotion_mode,
    }
  }
}


// Corresponds to spot_msgs__srv__SetLocomotion_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLocomotion_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetLocomotion_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetLocomotion_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetLocomotion_Response {
  type RmwMsg = super::srv::rmw::SetLocomotion_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__SetVelocity_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetVelocity_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity_limit: geometry_msgs::msg::Twist,

}



impl Default for SetVelocity_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetVelocity_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetVelocity_Request {
  type RmwMsg = super::srv::rmw::SetVelocity_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        velocity_limit: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.velocity_limit)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        velocity_limit: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.velocity_limit)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      velocity_limit: geometry_msgs::msg::Twist::from_rmw_message(msg.velocity_limit),
    }
  }
}


// Corresponds to spot_msgs__srv__SetVelocity_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetVelocity_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetVelocity_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetVelocity_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetVelocity_Response {
  type RmwMsg = super::srv::rmw::SetVelocity_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__ListAllDances_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAllDances_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListAllDances_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListAllDances_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListAllDances_Request {
  type RmwMsg = super::srv::rmw::ListAllDances_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__ListAllDances_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAllDances_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dances: Vec<std::string::String>,

}



impl Default for ListAllDances_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListAllDances_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListAllDances_Response {
  type RmwMsg = super::srv::rmw::ListAllDances_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        dances: msg.dances
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        dances: msg.dances
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      dances: msg.dances
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__srv__ListAllMoves_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAllMoves_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListAllMoves_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListAllMoves_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListAllMoves_Request {
  type RmwMsg = super::srv::rmw::ListAllMoves_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__ListAllMoves_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAllMoves_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub moves: Vec<std::string::String>,

}



impl Default for ListAllMoves_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListAllMoves_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListAllMoves_Response {
  type RmwMsg = super::srv::rmw::ListAllMoves_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        moves: msg.moves
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        moves: msg.moves
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      moves: msg.moves
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__srv__UploadAnimation_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadAnimation_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub animation_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub animation_file_content: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub animation_proto_serialized: Vec<u8>,

}



impl Default for UploadAnimation_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UploadAnimation_Request::default())
  }
}

impl rosidl_runtime_rs::Message for UploadAnimation_Request {
  type RmwMsg = super::srv::rmw::UploadAnimation_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        animation_name: msg.animation_name.as_str().into(),
        animation_file_content: msg.animation_file_content.as_str().into(),
        animation_proto_serialized: msg.animation_proto_serialized.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        animation_name: msg.animation_name.as_str().into(),
        animation_file_content: msg.animation_file_content.as_str().into(),
        animation_proto_serialized: msg.animation_proto_serialized.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      animation_name: msg.animation_name.to_string(),
      animation_file_content: msg.animation_file_content.to_string(),
      animation_proto_serialized: msg.animation_proto_serialized
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__srv__UploadAnimation_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadAnimation_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for UploadAnimation_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UploadAnimation_Response::default())
  }
}

impl rosidl_runtime_rs::Message for UploadAnimation_Response {
  type RmwMsg = super::srv::rmw::UploadAnimation_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__UploadSequence_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadSequence_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_proto_serialized: Vec<u8>,

}



impl Default for UploadSequence_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UploadSequence_Request::default())
  }
}

impl rosidl_runtime_rs::Message for UploadSequence_Request {
  type RmwMsg = super::srv::rmw::UploadSequence_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        sequence_proto_serialized: msg.sequence_proto_serialized.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        sequence_proto_serialized: msg.sequence_proto_serialized.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      sequence_proto_serialized: msg.sequence_proto_serialized
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__srv__UploadSequence_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadSequence_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for UploadSequence_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UploadSequence_Response::default())
  }
}

impl rosidl_runtime_rs::Message for UploadSequence_Response {
  type RmwMsg = super::srv::rmw::UploadSequence_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__ClearBehaviorFault_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearBehaviorFault_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u32,

}



impl Default for ClearBehaviorFault_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ClearBehaviorFault_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ClearBehaviorFault_Request {
  type RmwMsg = super::srv::rmw::ClearBehaviorFault_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
    }
  }
}


// Corresponds to spot_msgs__srv__ClearBehaviorFault_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearBehaviorFault_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for ClearBehaviorFault_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ClearBehaviorFault_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ClearBehaviorFault_Response {
  type RmwMsg = super::srv::rmw::ClearBehaviorFault_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__ListSounds_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListSounds_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListSounds_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListSounds_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListSounds_Request {
  type RmwMsg = super::srv::rmw::ListSounds_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__ListSounds_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListSounds_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub names: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for ListSounds_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListSounds_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListSounds_Response {
  type RmwMsg = super::srv::rmw::ListSounds_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        names: msg.names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        names: msg.names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      names: msg.names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__LoadSound_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadSound_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

    /// Path to wav file to upload
    pub wav_path: std::string::String,

}



impl Default for LoadSound_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadSound_Request::default())
  }
}

impl rosidl_runtime_rs::Message for LoadSound_Request {
  type RmwMsg = super::srv::rmw::LoadSound_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        wav_path: msg.wav_path.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        wav_path: msg.wav_path.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      wav_path: msg.wav_path.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__LoadSound_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadSound_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for LoadSound_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadSound_Response::default())
  }
}

impl rosidl_runtime_rs::Message for LoadSound_Response {
  type RmwMsg = super::srv::rmw::LoadSound_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__PlaySound_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub volume_multiplier: f32,

}



impl Default for PlaySound_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PlaySound_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PlaySound_Request {
  type RmwMsg = super::srv::rmw::PlaySound_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        volume_multiplier: msg.volume_multiplier,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      volume_multiplier: msg.volume_multiplier,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      volume_multiplier: msg.volume_multiplier,
    }
  }
}


// Corresponds to spot_msgs__srv__PlaySound_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for PlaySound_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PlaySound_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PlaySound_Response {
  type RmwMsg = super::srv::rmw::PlaySound_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__DeleteSound_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteSound_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

}



impl Default for DeleteSound_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeleteSound_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteSound_Request {
  type RmwMsg = super::srv::rmw::DeleteSound_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__DeleteSound_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteSound_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for DeleteSound_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeleteSound_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteSound_Response {
  type RmwMsg = super::srv::rmw::DeleteSound_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GetVolume_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetVolume_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetVolume_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetVolume_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetVolume_Request {
  type RmwMsg = super::srv::rmw::GetVolume_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__GetVolume_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetVolume_Response {
    /// From 0 to 100
    pub volume: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for GetVolume_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetVolume_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetVolume_Response {
  type RmwMsg = super::srv::rmw::GetVolume_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        volume: msg.volume,
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      volume: msg.volume,
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      volume: msg.volume,
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__SetVolume_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetVolume_Request {
    /// From 0 to 100
    pub volume: f32,

}



impl Default for SetVolume_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetVolume_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetVolume_Request {
  type RmwMsg = super::srv::rmw::SetVolume_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        volume: msg.volume,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      volume: msg.volume,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      volume: msg.volume,
    }
  }
}


// Corresponds to spot_msgs__srv__SetVolume_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetVolume_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetVolume_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetVolume_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetVolume_Response {
  type RmwMsg = super::srv::rmw::SetVolume_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__ListPtz_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListPtz_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListPtz_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListPtz_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListPtz_Request {
  type RmwMsg = super::srv::rmw::ListPtz_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__ListPtz_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListPtz_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub descriptions: Vec<bosdyn_spot_cam_api_msgs::msg::PtzDescription>,

}



impl Default for ListPtz_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListPtz_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListPtz_Response {
  type RmwMsg = super::srv::rmw::ListPtz_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        descriptions: msg.descriptions
          .into_iter()
          .map(|elem| bosdyn_spot_cam_api_msgs::msg::PtzDescription::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        descriptions: msg.descriptions
          .iter()
          .map(|elem| bosdyn_spot_cam_api_msgs::msg::PtzDescription::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      descriptions: msg.descriptions
          .into_iter()
          .map(bosdyn_spot_cam_api_msgs::msg::PtzDescription::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__srv__GetPtzPosition_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPtzPosition_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

}



impl Default for GetPtzPosition_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPtzPosition_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPtzPosition_Request {
  type RmwMsg = super::srv::rmw::GetPtzPosition_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GetPtzPosition_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPtzPosition_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: bosdyn_spot_cam_api_msgs::msg::PtzPosition,

}



impl Default for GetPtzPosition_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPtzPosition_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPtzPosition_Response {
  type RmwMsg = super::srv::rmw::GetPtzPosition_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        position: bosdyn_spot_cam_api_msgs::msg::PtzPosition::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        position: bosdyn_spot_cam_api_msgs::msg::PtzPosition::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      position: bosdyn_spot_cam_api_msgs::msg::PtzPosition::from_rmw_message(msg.position),
    }
  }
}


// Corresponds to spot_msgs__srv__SetPtzPosition_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPtzPosition_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetPtzPosition_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetPtzPosition_Request {
  type RmwMsg = super::srv::rmw::SetPtzPosition_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        pan: msg.pan,
        tilt: msg.tilt,
        zoom: msg.zoom,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      pan: msg.pan,
      tilt: msg.tilt,
      zoom: msg.zoom,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      pan: msg.pan,
      tilt: msg.tilt,
      zoom: msg.zoom,
    }
  }
}


// Corresponds to spot_msgs__srv__SetPtzPosition_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPtzPosition_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetPtzPosition_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetPtzPosition_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetPtzPosition_Response {
  type RmwMsg = super::srv::rmw::SetPtzPosition_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__InitializeLens_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InitializeLens_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for InitializeLens_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::InitializeLens_Request::default())
  }
}

impl rosidl_runtime_rs::Message for InitializeLens_Request {
  type RmwMsg = super::srv::rmw::InitializeLens_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__InitializeLens_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InitializeLens_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for InitializeLens_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::InitializeLens_Response::default())
  }
}

impl rosidl_runtime_rs::Message for InitializeLens_Response {
  type RmwMsg = super::srv::rmw::InitializeLens_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__DeleteLogpoint_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteLogpoint_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

}



impl Default for DeleteLogpoint_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeleteLogpoint_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteLogpoint_Request {
  type RmwMsg = super::srv::rmw::DeleteLogpoint_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__DeleteLogpoint_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteLogpoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for DeleteLogpoint_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeleteLogpoint_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteLogpoint_Response {
  type RmwMsg = super::srv::rmw::DeleteLogpoint_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GetLogpointStatus_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLogpointStatus_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

}



impl Default for GetLogpointStatus_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetLogpointStatus_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetLogpointStatus_Request {
  type RmwMsg = super::srv::rmw::GetLogpointStatus_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GetLogpointStatus_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLogpointStatus_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bosdyn_spot_cam_api_msgs::msg::LogpointLogStatus,

}



impl Default for GetLogpointStatus_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetLogpointStatus_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetLogpointStatus_Response {
  type RmwMsg = super::srv::rmw::GetLogpointStatus_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        status: bosdyn_spot_cam_api_msgs::msg::LogpointLogStatus::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        status: bosdyn_spot_cam_api_msgs::msg::LogpointLogStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      status: bosdyn_spot_cam_api_msgs::msg::LogpointLogStatus::from_rmw_message(msg.status),
    }
  }
}


// Corresponds to spot_msgs__srv__ListCameras_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListCameras_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListCameras_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListCameras_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListCameras_Request {
  type RmwMsg = super::srv::rmw::ListCameras_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__ListCameras_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListCameras_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cameras: Vec<bosdyn_spot_cam_api_msgs::msg::Camera>,

}



impl Default for ListCameras_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListCameras_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListCameras_Response {
  type RmwMsg = super::srv::rmw::ListCameras_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        cameras: msg.cameras
          .into_iter()
          .map(|elem| bosdyn_spot_cam_api_msgs::msg::Camera::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        cameras: msg.cameras
          .iter()
          .map(|elem| bosdyn_spot_cam_api_msgs::msg::Camera::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      cameras: msg.cameras
          .into_iter()
          .map(bosdyn_spot_cam_api_msgs::msg::Camera::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__srv__ListLogpoints_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListLogpoints_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListLogpoints_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListLogpoints_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListLogpoints_Request {
  type RmwMsg = super::srv::rmw::ListLogpoints_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__ListLogpoints_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListLogpoints_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub logpoints: Vec<bosdyn_spot_cam_api_msgs::msg::Logpoint>,

}



impl Default for ListLogpoints_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListLogpoints_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListLogpoints_Response {
  type RmwMsg = super::srv::rmw::ListLogpoints_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        logpoints: msg.logpoints
          .into_iter()
          .map(|elem| bosdyn_spot_cam_api_msgs::msg::Logpoint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        logpoints: msg.logpoints
          .iter()
          .map(|elem| bosdyn_spot_cam_api_msgs::msg::Logpoint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      logpoints: msg.logpoints
          .into_iter()
          .map(bosdyn_spot_cam_api_msgs::msg::Logpoint::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__srv__RetrieveLogpoint_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RetrieveLogpoint_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

    /// True to get raw data
    pub raw: bool,

}



impl Default for RetrieveLogpoint_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RetrieveLogpoint_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RetrieveLogpoint_Request {
  type RmwMsg = super::srv::rmw::RetrieveLogpoint_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        raw: msg.raw,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      raw: msg.raw,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      raw: msg.raw,
    }
  }
}


// Corresponds to spot_msgs__srv__RetrieveLogpoint_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RetrieveLogpoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub logpoint: bosdyn_spot_cam_api_msgs::msg::Logpoint,

    /// Data comes in as byte buffer.
    pub data: Vec<u8>,

}



impl Default for RetrieveLogpoint_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RetrieveLogpoint_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RetrieveLogpoint_Response {
  type RmwMsg = super::srv::rmw::RetrieveLogpoint_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        logpoint: bosdyn_spot_cam_api_msgs::msg::Logpoint::into_rmw_message(std::borrow::Cow::Owned(msg.logpoint)).into_owned(),
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        logpoint: bosdyn_spot_cam_api_msgs::msg::Logpoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.logpoint)).into_owned(),
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      logpoint: bosdyn_spot_cam_api_msgs::msg::Logpoint::from_rmw_message(msg.logpoint),
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__srv__RobotCommand_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: bosdyn_api_msgs::msg::RobotCommand,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::Duration,

}



impl Default for RobotCommand_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RobotCommand_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_Request {
  type RmwMsg = super::srv::rmw::RobotCommand_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: bosdyn_api_msgs::msg::RobotCommand::into_rmw_message(std::borrow::Cow::Owned(msg.command)).into_owned(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.duration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: bosdyn_api_msgs::msg::RobotCommand::into_rmw_message(std::borrow::Cow::Borrowed(&msg.command)).into_owned(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.duration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      command: bosdyn_api_msgs::msg::RobotCommand::from_rmw_message(msg.command),
      duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.duration),
    }
  }
}


// Corresponds to spot_msgs__srv__RobotCommand_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotCommand_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_command_id: u32,

}



impl Default for RobotCommand_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RobotCommand_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RobotCommand_Response {
  type RmwMsg = super::srv::rmw::RobotCommand_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        robot_command_id: msg.robot_command_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      robot_command_id: msg.robot_command_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      robot_command_id: msg.robot_command_id,
    }
  }
}


// Corresponds to spot_msgs__srv__SetGripperAngle_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGripperAngle_Request {
    /// In range [0, 90]
    pub gripper_angle: f32,

}



impl Default for SetGripperAngle_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetGripperAngle_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetGripperAngle_Request {
  type RmwMsg = super::srv::rmw::SetGripperAngle_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        gripper_angle: msg.gripper_angle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      gripper_angle: msg.gripper_angle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      gripper_angle: msg.gripper_angle,
    }
  }
}


// Corresponds to spot_msgs__srv__SetGripperAngle_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGripperAngle_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetGripperAngle_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetGripperAngle_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetGripperAngle_Response {
  type RmwMsg = super::srv::rmw::SetGripperAngle_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__StoreLogpoint_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StoreLogpoint_Request {
    /// Can take values: pano, ptz, ir (if ir cam attached), c0, c1, c2, c3, c4
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tag: std::string::String,

}



impl Default for StoreLogpoint_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StoreLogpoint_Request::default())
  }
}

impl rosidl_runtime_rs::Message for StoreLogpoint_Request {
  type RmwMsg = super::srv::rmw::StoreLogpoint_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        tag: msg.tag.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        tag: msg.tag.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      tag: msg.tag.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__StoreLogpoint_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StoreLogpoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub logpoint: bosdyn_spot_cam_api_msgs::msg::Logpoint,

}



impl Default for StoreLogpoint_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StoreLogpoint_Response::default())
  }
}

impl rosidl_runtime_rs::Message for StoreLogpoint_Response {
  type RmwMsg = super::srv::rmw::StoreLogpoint_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        logpoint: bosdyn_spot_cam_api_msgs::msg::Logpoint::into_rmw_message(std::borrow::Cow::Owned(msg.logpoint)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        logpoint: bosdyn_spot_cam_api_msgs::msg::Logpoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.logpoint)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      logpoint: bosdyn_spot_cam_api_msgs::msg::Logpoint::from_rmw_message(msg.logpoint),
    }
  }
}


// Corresponds to spot_msgs__srv__TagLogpoint_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TagLogpoint_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tag: std::string::String,

}



impl Default for TagLogpoint_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TagLogpoint_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TagLogpoint_Request {
  type RmwMsg = super::srv::rmw::TagLogpoint_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        tag: msg.tag.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        tag: msg.tag.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      tag: msg.tag.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__TagLogpoint_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TagLogpoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for TagLogpoint_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TagLogpoint_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TagLogpoint_Response {
  type RmwMsg = super::srv::rmw::TagLogpoint_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GetLEDBrightness_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLEDBrightness_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetLEDBrightness_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetLEDBrightness_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetLEDBrightness_Request {
  type RmwMsg = super::srv::rmw::GetLEDBrightness_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__GetLEDBrightness_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLEDBrightness_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

    /// In order REAR_LEFT, FRONT_LEFT, FRONT_RIGHT, REAR_RIGHT
    pub brightness: Vec<f32>,

}



impl Default for GetLEDBrightness_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetLEDBrightness_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetLEDBrightness_Response {
  type RmwMsg = super::srv::rmw::GetLEDBrightness_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        brightness: msg.brightness.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        brightness: msg.brightness.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      brightness: msg.brightness
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to spot_msgs__srv__SetLEDBrightness_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLEDBrightness_Request {
    /// In range [0, 1]
    pub brightness: f32,

}



impl Default for SetLEDBrightness_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetLEDBrightness_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetLEDBrightness_Request {
  type RmwMsg = super::srv::rmw::SetLEDBrightness_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        brightness: msg.brightness,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      brightness: msg.brightness,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      brightness: msg.brightness,
    }
  }
}


// Corresponds to spot_msgs__srv__SetLEDBrightness_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetLEDBrightness_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetLEDBrightness_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetLEDBrightness_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetLEDBrightness_Response {
  type RmwMsg = super::srv::rmw::SetLEDBrightness_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GraphNavUploadGraph_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavUploadGraph_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub upload_filepath: std::string::String,

}



impl Default for GraphNavUploadGraph_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GraphNavUploadGraph_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GraphNavUploadGraph_Request {
  type RmwMsg = super::srv::rmw::GraphNavUploadGraph_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        upload_filepath: msg.upload_filepath.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        upload_filepath: msg.upload_filepath.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      upload_filepath: msg.upload_filepath.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GraphNavUploadGraph_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavUploadGraph_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for GraphNavUploadGraph_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GraphNavUploadGraph_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GraphNavUploadGraph_Response {
  type RmwMsg = super::srv::rmw::GraphNavUploadGraph_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GraphNavClearGraph_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavClearGraph_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GraphNavClearGraph_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GraphNavClearGraph_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GraphNavClearGraph_Request {
  type RmwMsg = super::srv::rmw::GraphNavClearGraph_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__GraphNavClearGraph_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavClearGraph_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for GraphNavClearGraph_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GraphNavClearGraph_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GraphNavClearGraph_Response {
  type RmwMsg = super::srv::rmw::GraphNavClearGraph_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GraphNavSetLocalization_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavSetLocalization_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub method: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub waypoint_id: std::string::String,

}



impl Default for GraphNavSetLocalization_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GraphNavSetLocalization_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GraphNavSetLocalization_Request {
  type RmwMsg = super::srv::rmw::GraphNavSetLocalization_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        method: msg.method.as_str().into(),
        waypoint_id: msg.waypoint_id.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        method: msg.method.as_str().into(),
        waypoint_id: msg.waypoint_id.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      method: msg.method.to_string(),
      waypoint_id: msg.waypoint_id.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GraphNavSetLocalization_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavSetLocalization_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for GraphNavSetLocalization_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GraphNavSetLocalization_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GraphNavSetLocalization_Response {
  type RmwMsg = super::srv::rmw::GraphNavSetLocalization_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GraphNavGetLocalizationPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavGetLocalizationPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GraphNavGetLocalizationPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GraphNavGetLocalizationPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GraphNavGetLocalizationPose_Request {
  type RmwMsg = super::srv::rmw::GraphNavGetLocalizationPose_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to spot_msgs__srv__GraphNavGetLocalizationPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphNavGetLocalizationPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::PoseStamped,

}



impl Default for GraphNavGetLocalizationPose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GraphNavGetLocalizationPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GraphNavGetLocalizationPose_Response {
  type RmwMsg = super::srv::rmw::GraphNavGetLocalizationPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to spot_msgs__srv__Dock_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub dock_id: i16,

}



impl Default for Dock_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Dock_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Dock_Request {
  type RmwMsg = super::srv::rmw::Dock_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        dock_id: msg.dock_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      dock_id: msg.dock_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      dock_id: msg.dock_id,
    }
  }
}


// Corresponds to spot_msgs__srv__Dock_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for Dock_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Dock_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Dock_Response {
  type RmwMsg = super::srv::rmw::Dock_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__GetGripperCameraParameters_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGripperCameraParameters_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bosdyn_api_msgs::msg::GripperCameraGetParamRequest,

}



impl Default for GetGripperCameraParameters_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetGripperCameraParameters_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetGripperCameraParameters_Request {
  type RmwMsg = super::srv::rmw::GetGripperCameraParameters_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: bosdyn_api_msgs::msg::GripperCameraGetParamRequest::into_rmw_message(std::borrow::Cow::Owned(msg.request)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: bosdyn_api_msgs::msg::GripperCameraGetParamRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.request)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      request: bosdyn_api_msgs::msg::GripperCameraGetParamRequest::from_rmw_message(msg.request),
    }
  }
}


// Corresponds to spot_msgs__srv__GetGripperCameraParameters_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGripperCameraParameters_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: bosdyn_api_msgs::msg::GripperCameraGetParamResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for GetGripperCameraParameters_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetGripperCameraParameters_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetGripperCameraParameters_Response {
  type RmwMsg = super::srv::rmw::GetGripperCameraParameters_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: bosdyn_api_msgs::msg::GripperCameraGetParamResponse::into_rmw_message(std::borrow::Cow::Owned(msg.response)).into_owned(),
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: bosdyn_api_msgs::msg::GripperCameraGetParamResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.response)).into_owned(),
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      response: bosdyn_api_msgs::msg::GripperCameraGetParamResponse::from_rmw_message(msg.response),
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__SetGripperCameraParameters_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGripperCameraParameters_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bosdyn_api_msgs::msg::GripperCameraParamRequest,

}



impl Default for SetGripperCameraParameters_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetGripperCameraParameters_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetGripperCameraParameters_Request {
  type RmwMsg = super::srv::rmw::SetGripperCameraParameters_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: bosdyn_api_msgs::msg::GripperCameraParamRequest::into_rmw_message(std::borrow::Cow::Owned(msg.request)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: bosdyn_api_msgs::msg::GripperCameraParamRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.request)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      request: bosdyn_api_msgs::msg::GripperCameraParamRequest::from_rmw_message(msg.request),
    }
  }
}


// Corresponds to spot_msgs__srv__SetGripperCameraParameters_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGripperCameraParameters_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: bosdyn_api_msgs::msg::GripperCameraParamResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetGripperCameraParameters_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetGripperCameraParameters_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetGripperCameraParameters_Response {
  type RmwMsg = super::srv::rmw::SetGripperCameraParameters_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: bosdyn_api_msgs::msg::GripperCameraParamResponse::into_rmw_message(std::borrow::Cow::Owned(msg.response)).into_owned(),
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: bosdyn_api_msgs::msg::GripperCameraParamResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.response)).into_owned(),
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      response: bosdyn_api_msgs::msg::GripperCameraParamResponse::from_rmw_message(msg.response),
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__OverrideGraspOrCarry_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OverrideGraspOrCarry_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub grasp_override: bosdyn_api_msgs::msg::ApiGraspOverrideOverride,


    // This member is not documented.
    #[allow(missing_docs)]
    pub carry_override: bosdyn_api_msgs::msg::ManipulatorStateCarryState,

}



impl Default for OverrideGraspOrCarry_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::OverrideGraspOrCarry_Request::default())
  }
}

impl rosidl_runtime_rs::Message for OverrideGraspOrCarry_Request {
  type RmwMsg = super::srv::rmw::OverrideGraspOrCarry_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        grasp_override: bosdyn_api_msgs::msg::ApiGraspOverrideOverride::into_rmw_message(std::borrow::Cow::Owned(msg.grasp_override)).into_owned(),
        carry_override: bosdyn_api_msgs::msg::ManipulatorStateCarryState::into_rmw_message(std::borrow::Cow::Owned(msg.carry_override)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        grasp_override: bosdyn_api_msgs::msg::ApiGraspOverrideOverride::into_rmw_message(std::borrow::Cow::Borrowed(&msg.grasp_override)).into_owned(),
        carry_override: bosdyn_api_msgs::msg::ManipulatorStateCarryState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.carry_override)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      grasp_override: bosdyn_api_msgs::msg::ApiGraspOverrideOverride::from_rmw_message(msg.grasp_override),
      carry_override: bosdyn_api_msgs::msg::ManipulatorStateCarryState::from_rmw_message(msg.carry_override),
    }
  }
}


// Corresponds to spot_msgs__srv__OverrideGraspOrCarry_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OverrideGraspOrCarry_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for OverrideGraspOrCarry_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::OverrideGraspOrCarry_Response::default())
  }
}

impl rosidl_runtime_rs::Message for OverrideGraspOrCarry_Response {
  type RmwMsg = super::srv::rmw::OverrideGraspOrCarry_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__SetStandHeight_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetStandHeight_Request {
    /// In range [-0.15, 0.15]
    pub height: f32,

}



impl Default for SetStandHeight_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetStandHeight_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetStandHeight_Request {
  type RmwMsg = super::srv::rmw::SetStandHeight_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        height: msg.height,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      height: msg.height,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      height: msg.height,
    }
  }
}


// Corresponds to spot_msgs__srv__SetStandHeight_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetStandHeight_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetStandHeight_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetStandHeight_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetStandHeight_Response {
  type RmwMsg = super::srv::rmw::SetStandHeight_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__SetStairsMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetStairsMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stairs_mode: bosdyn_spot_api_msgs::msg::MobilityParamsStairsMode,

}



impl Default for SetStairsMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetStairsMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetStairsMode_Request {
  type RmwMsg = super::srv::rmw::SetStairsMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stairs_mode: bosdyn_spot_api_msgs::msg::MobilityParamsStairsMode::into_rmw_message(std::borrow::Cow::Owned(msg.stairs_mode)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stairs_mode: bosdyn_spot_api_msgs::msg::MobilityParamsStairsMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stairs_mode)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      stairs_mode: bosdyn_spot_api_msgs::msg::MobilityParamsStairsMode::from_rmw_message(msg.stairs_mode),
    }
  }
}


// Corresponds to spot_msgs__srv__SetStairsMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetStairsMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetStairsMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetStairsMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetStairsMode_Response {
  type RmwMsg = super::srv::rmw::SetStairsMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to spot_msgs__srv__MutateWorldObject_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MutateWorldObject_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bosdyn_api_msgs::msg::MutateWorldObjectRequest,

}



impl Default for MutateWorldObject_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MutateWorldObject_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MutateWorldObject_Request {
  type RmwMsg = super::srv::rmw::MutateWorldObject_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: bosdyn_api_msgs::msg::MutateWorldObjectRequest::into_rmw_message(std::borrow::Cow::Owned(msg.request)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: bosdyn_api_msgs::msg::MutateWorldObjectRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.request)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      request: bosdyn_api_msgs::msg::MutateWorldObjectRequest::from_rmw_message(msg.request),
    }
  }
}


// Corresponds to spot_msgs__srv__MutateWorldObject_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MutateWorldObject_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: bosdyn_api_msgs::msg::MutateWorldObjectResponse,

}



impl Default for MutateWorldObject_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MutateWorldObject_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MutateWorldObject_Response {
  type RmwMsg = super::srv::rmw::MutateWorldObject_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: bosdyn_api_msgs::msg::MutateWorldObjectResponse::into_rmw_message(std::borrow::Cow::Owned(msg.response)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: bosdyn_api_msgs::msg::MutateWorldObjectResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.response)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      response: bosdyn_api_msgs::msg::MutateWorldObjectResponse::from_rmw_message(msg.response),
    }
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


