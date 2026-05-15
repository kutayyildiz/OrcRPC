use actrpc_core::action::{ActionDescriptor, ActionKind};
use actrpc_transport::{JsonRpcClient, JsonRpcClientProvider, TransportError, TransportTarget};

use crate::{
    error::{ActionExecutionError, InterceptorError, OrchestratorError},
    interceptor::{
        ImmutableInterceptorPipeline, Interceptor, InterceptorConfig, InterceptorPolicy,
        JsonRpcBackedInterceptor, WorkingInterceptorPipeline,
        initialization::validate_interceptor_registration,
    },
};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct InterceptorCatalogEntry {
    pub name: String,
    pub policy: InterceptorPolicy,
    pub interceptor: Arc<dyn Interceptor>,
}

impl core::fmt::Debug for InterceptorCatalogEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InterceptorCatalogEntry")
            .field("name", &self.name)
            .field("policy", &self.policy)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct InterceptorCatalog {
    entries: HashMap<String, InterceptorCatalogEntry>,
    outbound_pipeline: ImmutableInterceptorPipeline,
    inbound_pipeline: ImmutableInterceptorPipeline,
}

impl InterceptorCatalog {
    pub fn new(
        entries: HashMap<String, InterceptorCatalogEntry>,
        outbound_pipeline: ImmutableInterceptorPipeline,
        inbound_pipeline: ImmutableInterceptorPipeline,
    ) -> Self {
        Self {
            entries,
            outbound_pipeline,
            inbound_pipeline,
        }
    }

    pub async fn build(
        interceptors: Vec<(InterceptorConfig, Arc<dyn Interceptor>)>,
        available_actions: &HashMap<ActionKind, ActionDescriptor>,
    ) -> Result<Self, OrchestratorError> {
        let mut entries = HashMap::new();
        let mut outbound = Vec::new();
        let mut inbound = Vec::new();

        let mut ordered = interceptors;

        ordered.sort_by(|(a_config, _), (b_config, _)| {
            a_config
                .priority
                .cmp(&b_config.priority)
                .then_with(|| a_config.name.cmp(&b_config.name))
        });

        for (config, interceptor) in ordered {
            let initialization = interceptor.initialize().await.map_err(|source| {
                OrchestratorError::Interceptor(InterceptorError::InitializationFailed {
                    name: config.name.clone(),
                    source,
                })
            })?;

            validate_interceptor_registration(
                &config.name,
                &config.policy,
                &initialization,
                available_actions,
            )?;

            if entries.contains_key(&config.name) {
                return Err(OrchestratorError::Interceptor(
                    InterceptorError::DuplicateRegistration { name: config.name },
                ));
            }

            if initialization.supports_outbound {
                outbound.push(config.name.clone());
            }

            if initialization.supports_inbound {
                inbound.push(config.name.clone());
            }

            entries.insert(
                config.name.clone(),
                InterceptorCatalogEntry {
                    name: config.name,
                    policy: config.policy,
                    interceptor,
                },
            );
        }

        Ok(Self::new(
            entries,
            ImmutableInterceptorPipeline::new(outbound),
            ImmutableInterceptorPipeline::new(inbound),
        ))
    }

    pub async fn build_from_targets<P>(
        sources: Vec<(InterceptorConfig, TransportTarget)>,
        available_actions: &HashMap<ActionKind, ActionDescriptor>,
        client_provider: &P,
    ) -> Result<Self, OrchestratorError>
    where
        P: JsonRpcClientProvider<
                Client = Arc<dyn JsonRpcClient<Error = TransportError>>,
                Error = TransportError,
            > + Send
            + Sync,
    {
        let mut interceptors = Vec::with_capacity(sources.len());

        for (config, target) in sources {
            let client = client_provider
                .get_client(&target)
                .await
                .map_err(OrchestratorError::Transport)?;

            let interceptor: Arc<dyn Interceptor> = Arc::new(JsonRpcBackedInterceptor::new(client));

            interceptors.push((config, interceptor));
        }

        Self::build(interceptors, available_actions).await
    }

    pub fn get_entry(&self, name: &str) -> Result<InterceptorCatalogEntry, ActionExecutionError> {
        let Some(entry) = self.entries.get(name) else {
            return Err(ActionExecutionError::NotFound {
                target: name.to_owned(),
            });
        };

        Ok(entry.clone())
    }

    pub fn entries(&self) -> Vec<InterceptorCatalogEntry> {
        self.entries.values().cloned().collect()
    }

    pub fn entries_for_names(
        &self,
        names: &[String],
    ) -> Result<Vec<InterceptorCatalogEntry>, ActionExecutionError> {
        let mut result = Vec::with_capacity(names.len());

        for name in names {
            let Some(entry) = self.entries.get(name) else {
                return Err(ActionExecutionError::NotFound {
                    target: name.clone(),
                });
            };

            result.push(entry.clone());
        }

        Ok(result)
    }

    pub fn outbound_pipeline_snapshot(&self) -> WorkingInterceptorPipeline {
        self.outbound_pipeline.snapshot()
    }

    pub fn inbound_pipeline_snapshot(&self) -> WorkingInterceptorPipeline {
        self.inbound_pipeline.snapshot()
    }
}
