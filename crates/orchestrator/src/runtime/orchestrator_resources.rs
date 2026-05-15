use crate::{external_method::ExternalMethodCatalog, interceptor::InterceptorCatalog};
use std::sync::Arc;

#[derive(Clone)]
pub struct OrchestratorResources {
    pub interceptor_catalog: Arc<InterceptorCatalog>,
    pub external_method_catalog: Arc<ExternalMethodCatalog>,
}

impl OrchestratorResources {
    pub fn new(
        interceptor_catalog: Arc<InterceptorCatalog>,
        external_method_catalog: Arc<ExternalMethodCatalog>,
    ) -> Self {
        Self {
            interceptor_catalog,
            external_method_catalog,
        }
    }
}
