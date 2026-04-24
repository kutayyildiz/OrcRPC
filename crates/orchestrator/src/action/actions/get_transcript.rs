use crate::{
    action::TypedActionHandler, error::ActionExecutionError, runtime::TranscriptState,
    transcript::TranscriptEntryView,
};
use actrpc_core::{
    action::{ActionSpec, NoParams, RequestedAction, ResolvedAction},
    interception::InterceptionRequest,
};
use std::sync::Arc;

pub struct GetTranscript;

impl ActionSpec for GetTranscript {
    type Params = NoParams;
    type Result = Vec<TranscriptEntryView>;

    const KIND: &'static str = "get_transcript";
}

pub struct GetTranscriptHandler {
    transcript: Arc<TranscriptState>,
}

impl GetTranscriptHandler {
    pub fn new(transcript: Arc<TranscriptState>) -> Self {
        Self { transcript }
    }
}

impl TypedActionHandler<GetTranscript> for GetTranscriptHandler {
    fn handle_typed(
        &self,
        _request: &InterceptionRequest,
        action: RequestedAction<GetTranscript>,
    ) -> Result<ResolvedAction<GetTranscript>, ActionExecutionError> {
        let entries = self.transcript.snapshot();

        Ok(ResolvedAction {
            params: action.params,
            result: Ok(entries),
        })
    }
}
