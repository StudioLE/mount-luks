use crate::prelude::*;

pub fn validate_command() -> Result<(), AnyReport> {
    let options = Options::read_options()?;

    print_header(&options);
    let counter = Mutex::new(0);
    let total_steps = 2;

    print_step_start(&counter, total_steps, "Checking if root");
    is_root()?;
    print_step_completed("Access granted");

    print_step_start(&counter, total_steps, "Validating key");
    let key = get_key(&options)?;
    debug!("Key is {} characters", key.len());
    check_key(&options, &key)?;
    print_step_completed("Key is valid");

    Ok(())
}
