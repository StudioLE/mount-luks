use crate::prelude::*;

pub fn set_tpm_command() -> Result<(), AnyReport> {
    let options = Options::read_options()?;

    print_header(&options);
    let counter = Mutex::new(0);
    let total_steps = 7;

    print_step_start(&counter, total_steps, "Checking if root");
    is_root()?;
    print_step_completed("Access granted");

    print_step_start(&counter, total_steps, "Checking TPM handle");
    check_handle(&options)?;
    print_step_completed("TPM handle is available");

    print_step_start(&counter, total_steps, "Creating TPM PCR policy");
    create_policy()?;
    print_step_completed("Created TPM PCR policy");

    print_step_start(&counter, total_steps, "Creating TPM primary key");
    create_primary()?;
    print_step_completed("Created TPM primary key");

    print_step_start(&counter, total_steps, "Creating TPM object");
    create_object()?;
    print_step_completed("Created TPM object");

    print_step_start(&counter, total_steps, "Loading object into TPM");
    load_object()?;
    print_step_completed("Loaded object into TPM");

    print_step_start(&counter, total_steps, "Making TPM object persistent");
    persist_object(&options)?;
    print_step_completed("Made TPM object persistent");

    Ok(())
}
