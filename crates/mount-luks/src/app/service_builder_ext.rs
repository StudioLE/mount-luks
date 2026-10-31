use crate::prelude::*;

/// Register all application services on a [`ServiceBuilder`].
pub trait ServiceBuilderExt {
    /// Register all application services.
    fn with_app_services(self) -> Self;

    /// Register all application services with stubs for external calls.
    #[cfg(test)]
    fn with_test_services(self) -> Self;
}

impl ServiceBuilderExt for ServiceBuilder {
    fn with_app_services(self) -> Self {
        self.with_logging(create_logger)
            .with_trait::<dyn IsRoot, IsRootAdapter>()
            .with_trait::<dyn IsLuks, CryptsetupAdapter>()
            .with_trait::<dyn Unlock, CryptsetupAdapter>()
            .with_trait::<dyn CheckKey, CryptsetupAdapter>()
            .with_trait::<dyn AddKey, CryptsetupAdapter>()
            .with_trait::<dyn Unseal, Tpm2Adapter>()
            .with_type::<Retry<dyn Unseal>>()
            .with_trait::<dyn GetHandles, Tpm2Adapter>()
            .with_trait::<dyn CreatePolicy, Tpm2Adapter>()
            .with_trait::<dyn CreatePrimary, Tpm2Adapter>()
            .with_trait::<dyn CreateObject, Tpm2Adapter>()
            .with_trait::<dyn LoadObject, Tpm2Adapter>()
            .with_trait::<dyn Persist, Tpm2Adapter>()
            .with_trait::<dyn FindMount, FindmntAdapter>()
            .with_trait::<dyn MountPartition, MountAdapter>()
            .with_trait::<dyn PromptPassword, RpasswordAdapter>()
            .with_trait::<dyn GetKey, GetKeyAdapter>()
            .with_trait::<dyn ResolvePartition, ResolvePartitionAdapter>()
            .with_trait::<dyn CheckMountExists, MountExistsAdapter>()
            .with_trait::<dyn IsPartitionLocked, PartitionLockedAdapter>()
            .with_type::<MountHandler>()
            .with_type::<ValidateHandler>()
            .with_type::<SetTpmHandler>()
            .with_type::<SetLuksHandler>()
            .with_type::<Options>()
            .with_type::<CliArgs>()
            .with_type::<Ui>()
            .with_type::<SubcommandHandler>()
            .with_init::<Ui>()
    }

    #[cfg(test)]
    fn with_test_services(self) -> Self {
        self.with_app_services()
            .with_trait::<dyn IsRoot, IsRootStub>()
            .with_trait::<dyn IsLuks, IsLuksStub>()
            .with_trait::<dyn Unlock, UnlockStub>()
            .with_trait::<dyn CheckKey, CheckKeyStub>()
            .with_trait::<dyn AddKey, AddKeyStub>()
            .with_trait::<dyn Unseal, UnsealStub>()
            .with_trait::<dyn GetHandles, GetHandlesStub>()
            .with_trait::<dyn CreatePolicy, CreatePolicyStub>()
            .with_trait::<dyn CreatePrimary, CreatePrimaryStub>()
            .with_trait::<dyn CreateObject, CreateObjectStub>()
            .with_trait::<dyn LoadObject, LoadObjectStub>()
            .with_trait::<dyn Persist, PersistStub>()
            .with_trait::<dyn FindMount, FindMountStub>()
            .with_trait::<dyn MountPartition, MountPartitionStub>()
            .with_trait::<dyn PromptPassword, PromptPasswordStub>()
            .with_trait::<dyn ResolvePartition, ResolvePartitionStub>()
            .with_trait::<dyn CheckMountExists, CheckMountExistsStub>()
            .with_trait::<dyn IsPartitionLocked, IsPartitionLockedStub>()
            .with_instance(CliArgs::fixture())
            .with_instance(Options::fixture())
    }
}

#[cfg(not(test))]
fn create_logger(services: &ServiceProvider) -> Result<Logger, Report<ResolveError>> {
    let cli = services.get::<CliArgs>()?;
    let logger = LoggerBuilder::new()
        .with_level(cli.log_level.unwrap_or_default())
        .build();
    Ok(logger)
}

#[cfg(test)]
#[expect(
    clippy::unnecessary_wraps,
    reason = "signature required by with_logging"
)]
fn create_logger(_services: &ServiceProvider) -> Result<Logger, Report<ResolveError>> {
    let logger = LoggerBuilder::new().with_level(LogLevel::Trace).build();
    Ok(logger)
}
