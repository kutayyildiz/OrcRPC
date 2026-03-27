use crate::action::ActionKind;
use serde::{Serialize, de::DeserializeOwned};

pub trait ActionSpec {
    type Params: Serialize + DeserializeOwned;
    type Result: Serialize + DeserializeOwned;

    const KIND: ActionKind;
}
