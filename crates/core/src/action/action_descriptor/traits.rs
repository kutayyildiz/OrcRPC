use crate::action::{
    ActionSpec,
    action_descriptor::types::{ActionDescriptor, OkDescriptor, ParamsDescriptor, ValueDescriptor},
};

pub trait DescribeValue {
    fn describe_value() -> ValueDescriptor;
}

pub trait DescribeParams {
    fn describe_params() -> Option<ParamsDescriptor>;
}

pub trait DescribeOk {
    fn describe_ok() -> Option<OkDescriptor>;
}

pub trait DescribeActionSpec: ActionSpec {
    fn descriptor() -> ActionDescriptor {
        ActionDescriptor {
            kind: Self::action_kind(),
            params: <Self::Params as DescribeParams>::describe_params(),
            ok: <Self::Result as DescribeOk>::describe_ok(),
        }
    }
}

impl<T> DescribeActionSpec for T where T: ActionSpec {}
