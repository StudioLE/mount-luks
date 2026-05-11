use crate::prelude::*;

/// Register all application services on a [`ServiceBuilder`].
pub trait ServiceBuilderExt {
    /// Register all application services.
    fn with_app_services(self) -> Self;
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
            .with_type::<CliOptions>()
            .with_type::<Ui>()
            .with_type::<SubCommandHandler>()
            .with_init::<Ui>()
    }
}

fn create_logger(services: &ServiceProvider) -> Result<Logger, Report<ResolveError>> {
    let cli = services.get::<CliOptions>()?;
    let logger = LoggerBuilder::new()
        .with_level(cli.log_level.unwrap_or_default())
        .build();
    Ok(logger)
}
