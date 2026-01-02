use crate::prelude::*;

pub fn set_luks_command() -> Result<(), AnyReport> {
    let options = Options::read_options()?;

    print_header(&options);
    let counter = Mutex::new(0);
    let total_steps = 4;

    print_step_start(&counter, total_steps, "Checking if root");
    is_root()?;
    print_step_completed("Access granted");

    print_step_start(&counter, total_steps, "Checking if partition exists");
    check_partition_exist(&options)?;
    print_step_completed("Partition exists");

    print_step_start(
        &counter,
        total_steps,
        "Checking if partition is encrypted with LUKS",
    );
    is_luks_partition(&options)?;
    print_step_completed("Partition is encrypted with LUKS");

    print_step_start(&counter, total_steps, "Adding LUKS key");
    add_key(&options)?;
    print_step_completed("Added LUKS key");

    Ok(())
}
