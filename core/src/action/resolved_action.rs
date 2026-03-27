use crate::{
    action::{ActionSpec, ResolvedActionRecord},
    error::{ActionError, CodecError},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedAction<A: ActionSpec> {
    pub params: A::Params,
    pub result: Result<A::Result, ActionError>,
}

impl<A> ResolvedAction<A>
where
    A: ActionSpec,
{
    fn decode_from_record(value: &ResolvedActionRecord) -> Result<Self, CodecError> {
        if value.kind != A::KIND {
            return Err(CodecError::MismatchedActionKind {
                expected: A::KIND.to_string(),
                actual: value.kind.to_string(),
            });
        }

        let params = serde_json::from_value(value.params.clone()).map_err(|e| {
            CodecError::DecodeResolvedParams {
                action: value.kind.to_string(),
                params: value.params.clone(),
                reason: e.to_string(),
            }
        })?;

        let result = match &value.result {
            Ok(v) => Ok(serde_json::from_value(v.clone()).map_err(|e| {
                CodecError::DecodeResolvedResult {
                    action: value.kind.to_string(),
                    result: v.clone(),
                    reason: e.to_string(),
                }
            })?),
            Err(err) => Err(err.clone()),
        };

        Ok(Self { params, result })
    }
}

impl<A> TryFrom<&ResolvedActionRecord> for ResolvedAction<A>
where
    A: ActionSpec,
{
    type Error = CodecError;

    fn try_from(value: &ResolvedActionRecord) -> Result<Self, Self::Error> {
        Self::decode_from_record(value)
    }
}

impl<A> TryFrom<ResolvedActionRecord> for ResolvedAction<A>
where
    A: ActionSpec,
{
    type Error = CodecError;

    fn try_from(value: ResolvedActionRecord) -> Result<Self, Self::Error> {
        Self::decode_from_record(&value)
    }
}
