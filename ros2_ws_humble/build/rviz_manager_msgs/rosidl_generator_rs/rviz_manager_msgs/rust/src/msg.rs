#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to rviz_manager_msgs__msg__ManagerLaunch

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ManagerLaunch {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// unique identifier
    pub id: i32,

    /// "start", "stop", "restart", "status"
    pub action: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ns: std::string::String,

    /// true = session ros, false = custom cmd
    pub bash_session: bool,

    /// true = launch file, false = node
    pub is_launch_file: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub package: std::string::String,

    /// node or launch file
    pub executable: std::string::String,

    /// CLI args
    pub arguments: Vec<std::string::String>,

    /// --ros-args ...
    pub ros_arguments: Vec<std::string::String>,

    /// optional
    pub working_dir: std::string::String,

    /// tmux session name (optional override)
    pub session_name: std::string::String,

    /// optional flag
    pub use_sim_time: bool,

    /// optional (sec)
    pub timeout: i32,

}



impl Default for ManagerLaunch {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ManagerLaunch::default())
  }
}

impl rosidl_runtime_rs::Message for ManagerLaunch {
  type RmwMsg = super::msg::rmw::ManagerLaunch;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        id: msg.id,
        action: msg.action.as_str().into(),
        ns: msg.ns.as_str().into(),
        bash_session: msg.bash_session,
        is_launch_file: msg.is_launch_file,
        package: msg.package.as_str().into(),
        executable: msg.executable.as_str().into(),
        arguments: msg.arguments
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        ros_arguments: msg.ros_arguments
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        working_dir: msg.working_dir.as_str().into(),
        session_name: msg.session_name.as_str().into(),
        use_sim_time: msg.use_sim_time,
        timeout: msg.timeout,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      id: msg.id,
        action: msg.action.as_str().into(),
        ns: msg.ns.as_str().into(),
      bash_session: msg.bash_session,
      is_launch_file: msg.is_launch_file,
        package: msg.package.as_str().into(),
        executable: msg.executable.as_str().into(),
        arguments: msg.arguments
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        ros_arguments: msg.ros_arguments
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        working_dir: msg.working_dir.as_str().into(),
        session_name: msg.session_name.as_str().into(),
      use_sim_time: msg.use_sim_time,
      timeout: msg.timeout,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      id: msg.id,
      action: msg.action.to_string(),
      ns: msg.ns.to_string(),
      bash_session: msg.bash_session,
      is_launch_file: msg.is_launch_file,
      package: msg.package.to_string(),
      executable: msg.executable.to_string(),
      arguments: msg.arguments
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      ros_arguments: msg.ros_arguments
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      working_dir: msg.working_dir.to_string(),
      session_name: msg.session_name.to_string(),
      use_sim_time: msg.use_sim_time,
      timeout: msg.timeout,
    }
  }
}


// Corresponds to rviz_manager_msgs__msg__ManagerStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ManagerStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i32,

    /// "running", "stopped", "error"
    pub status: std::string::String,

    /// logs or error
    pub message: std::string::String,

}



impl Default for ManagerStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ManagerStatus::default())
  }
}

impl rosidl_runtime_rs::Message for ManagerStatus {
  type RmwMsg = super::msg::rmw::ManagerStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        status: msg.status.as_str().into(),
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
        status: msg.status.as_str().into(),
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      status: msg.status.to_string(),
      message: msg.message.to_string(),
    }
  }
}


