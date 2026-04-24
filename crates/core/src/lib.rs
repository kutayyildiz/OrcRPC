mod convert;

pub mod action;
pub mod error;
pub mod interception;
pub mod interceptor;
pub mod json_rpc;
pub mod participant;

pub use convert::INTERCEPT_METHOD;

pub use actrpc_core_macros::{DescribeOk, DescribeParams, DescribeValue};

