#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to ros_babel_fish_test_msgs__msg__TestArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TestArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub bools: Vec<bool>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub uint8s: Vec<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub uint16s: [u16; 32],


    // This member is not documented.
    #[allow(missing_docs)]
    pub uint32s: Vec<u32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub uint64s: Vec<u64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub int8s: Vec<i8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub int16s: Vec<i16>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub int32s: Vec<i32>,

    /// Comment
    pub int64s: [i64; 32],


    // This member is not documented.
    #[allow(missing_docs)]
    pub float32s: Vec<f32>,

    /// Bounded array
    pub float64s: rosidl_runtime_rs::BoundedSequence<f64, 16>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub times: Vec<builtin_interfaces::msg::Time>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub durations: [builtin_interfaces::msg::Duration; 12],


    // This member is not documented.
    #[allow(missing_docs)]
    pub strings: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub subarrays_fixed: [super::msg::TestSubArray; 10],


    // This member is not documented.
    #[allow(missing_docs)]
    pub subarrays: Vec<super::msg::TestSubArray>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub subarray: super::msg::TestSubArray,

}



impl Default for TestArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TestArray::default())
  }
}

impl rosidl_runtime_rs::Message for TestArray {
  type RmwMsg = super::msg::rmw::TestArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        bools: msg.bools.into(),
        uint8s: msg.uint8s.into(),
        uint16s: msg.uint16s,
        uint32s: msg.uint32s.into(),
        uint64s: msg.uint64s.into(),
        int8s: msg.int8s.into(),
        int16s: msg.int16s.into(),
        int32s: msg.int32s.into(),
        int64s: msg.int64s,
        float32s: msg.float32s.into(),
        float64s: msg.float64s,
        times: msg.times
          .into_iter()
          .map(|elem| builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        durations: msg.durations
          .map(|elem| builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
        strings: msg.strings
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        subarrays_fixed: msg.subarrays_fixed
          .map(|elem| super::msg::TestSubArray::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
        subarrays: msg.subarrays
          .into_iter()
          .map(|elem| super::msg::TestSubArray::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        subarray: super::msg::TestSubArray::into_rmw_message(std::borrow::Cow::Owned(msg.subarray)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        bools: msg.bools.as_slice().into(),
        uint8s: msg.uint8s.as_slice().into(),
        uint16s: msg.uint16s,
        uint32s: msg.uint32s.as_slice().into(),
        uint64s: msg.uint64s.as_slice().into(),
        int8s: msg.int8s.as_slice().into(),
        int16s: msg.int16s.as_slice().into(),
        int32s: msg.int32s.as_slice().into(),
        int64s: msg.int64s,
        float32s: msg.float32s.as_slice().into(),
        float64s: msg.float64s.clone(),
        times: msg.times
          .iter()
          .map(|elem| builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        durations: msg.durations
          .iter()
          .map(|elem| builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
        strings: msg.strings
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        subarrays_fixed: msg.subarrays_fixed
          .iter()
          .map(|elem| super::msg::TestSubArray::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
        subarrays: msg.subarrays
          .iter()
          .map(|elem| super::msg::TestSubArray::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        subarray: super::msg::TestSubArray::into_rmw_message(std::borrow::Cow::Borrowed(&msg.subarray)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      bools: msg.bools
          .into_iter()
          .collect(),
      uint8s: msg.uint8s
          .into_iter()
          .collect(),
      uint16s: msg.uint16s,
      uint32s: msg.uint32s
          .into_iter()
          .collect(),
      uint64s: msg.uint64s
          .into_iter()
          .collect(),
      int8s: msg.int8s
          .into_iter()
          .collect(),
      int16s: msg.int16s
          .into_iter()
          .collect(),
      int32s: msg.int32s
          .into_iter()
          .collect(),
      int64s: msg.int64s,
      float32s: msg.float32s
          .into_iter()
          .collect(),
      float64s: msg.float64s,
      times: msg.times
          .into_iter()
          .map(builtin_interfaces::msg::Time::from_rmw_message)
          .collect(),
      durations: msg.durations
        .map(builtin_interfaces::msg::Duration::from_rmw_message),
      strings: msg.strings
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      subarrays_fixed: msg.subarrays_fixed
        .map(super::msg::TestSubArray::from_rmw_message),
      subarrays: msg.subarrays
          .into_iter()
          .map(super::msg::TestSubArray::from_rmw_message)
          .collect(),
      subarray: super::msg::TestSubArray::from_rmw_message(msg.subarray),
    }
  }
}


// Corresponds to ros_babel_fish_test_msgs__msg__TestMessage

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TestMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    pub str: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bounded_str: rosidl_runtime_rs::BoundedString<12>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub t: builtin_interfaces::msg::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: builtin_interfaces::msg::Duration,

    /// more comment
    pub point_arr: Vec<geometry_msgs::msg::Point>,

}



impl Default for TestMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TestMessage::default())
  }
}

impl rosidl_runtime_rs::Message for TestMessage {
  type RmwMsg = super::msg::rmw::TestMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        b: msg.b,
        ui8: msg.ui8,
        ui16: msg.ui16,
        ui32: msg.ui32,
        ui64: msg.ui64,
        i8: msg.i8,
        i16: msg.i16,
        i32: msg.i32,
        i64: msg.i64,
        f32: msg.f32,
        f64: msg.f64,
        str: msg.str.as_str().into(),
        bounded_str: msg.bounded_str,
        t: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.t)).into_owned(),
        d: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.d)).into_owned(),
        point_arr: msg.point_arr
          .into_iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      b: msg.b,
      ui8: msg.ui8,
      ui16: msg.ui16,
      ui32: msg.ui32,
      ui64: msg.ui64,
      i8: msg.i8,
      i16: msg.i16,
      i32: msg.i32,
      i64: msg.i64,
      f32: msg.f32,
      f64: msg.f64,
        str: msg.str.as_str().into(),
        bounded_str: msg.bounded_str.clone(),
        t: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.t)).into_owned(),
        d: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.d)).into_owned(),
        point_arr: msg.point_arr
          .iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      b: msg.b,
      ui8: msg.ui8,
      ui16: msg.ui16,
      ui32: msg.ui32,
      ui64: msg.ui64,
      i8: msg.i8,
      i16: msg.i16,
      i32: msg.i32,
      i64: msg.i64,
      f32: msg.f32,
      f64: msg.f64,
      str: msg.str.to_string(),
      bounded_str: msg.bounded_str,
      t: builtin_interfaces::msg::Time::from_rmw_message(msg.t),
      d: builtin_interfaces::msg::Duration::from_rmw_message(msg.d),
      point_arr: msg.point_arr
          .into_iter()
          .map(geometry_msgs::msg::Point::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to ros_babel_fish_test_msgs__msg__TestSubArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TestSubArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ints: Vec<i32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub strings: rosidl_runtime_rs::BoundedSequence<rosidl_runtime_rs::String, 10>,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub times: [builtin_interfaces::msg::Time; 42],


    // This member is not documented.
    #[allow(missing_docs)]
    pub floats: [f64; 12],

}



impl Default for TestSubArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TestSubArray::default())
  }
}

impl rosidl_runtime_rs::Message for TestSubArray {
  type RmwMsg = super::msg::rmw::TestSubArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ints: msg.ints.into(),
        strings: msg.strings,
        times: msg.times
          .map(|elem| builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
        floats: msg.floats,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ints: msg.ints.as_slice().into(),
        strings: msg.strings.clone(),
        times: msg.times
          .iter()
          .map(|elem| builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
        floats: msg.floats,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ints: msg.ints
          .into_iter()
          .collect(),
      strings: msg.strings,
      times: msg.times
        .map(builtin_interfaces::msg::Time::from_rmw_message),
      floats: msg.floats,
    }
  }
}


