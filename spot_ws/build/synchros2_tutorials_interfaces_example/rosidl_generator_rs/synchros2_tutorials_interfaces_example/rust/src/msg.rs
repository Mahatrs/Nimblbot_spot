#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to synchros2_tutorials_interfaces_example__msg__String

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct String {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: std::string::String,

}



impl Default for String {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::String::default())
  }
}

impl rosidl_runtime_rs::Message for String {
  type RmwMsg = super::msg::rmw::String;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      data: msg.data.to_string(),
    }
  }
}


