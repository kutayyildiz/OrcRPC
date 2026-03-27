use crate::{
    action::{ActionSpec, RequestedActionRecord},
    error::CodecError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RequestedAction<A: ActionSpec> {
    pub params: A::Params,
}

impl<A> RequestedAction<A>
where
    A: ActionSpec,
{
    fn decode_from_record(value: &RequestedActionRecord) -> Result<Self, CodecError> {
        if value.kind != A::KIND {
            return Err(CodecError::UnsupportedAction {
                action: value.kind.to_string(),
            });
        }

        let params = serde_json::from_value(value.params.clone()).map_err(|e| {
            CodecError::DecodeRequestedParams {
                action: value.kind.to_string(),
                params: value.params.clone(),
                reason: e.to_string(),
            }
        })?;

        Ok(Self { params })
    }
}

impl<A> TryFrom<RequestedActionRecord> for RequestedAction<A>
where
    A: ActionSpec,
{
    type Error = CodecError;

    fn try_from(value: RequestedActionRecord) -> Result<Self, Self::Error> {
        Self::decode_from_record(&value)
    }
}

impl<A> TryFrom<&RequestedActionRecord> for RequestedAction<A>
where
    A: ActionSpec,
{
    type Error = CodecError;

    fn try_from(value: &RequestedActionRecord) -> Result<Self, Self::Error> {
        Self::decode_from_record(value)
    }
}
