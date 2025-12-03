mod components;

use components::{Orchestrator};

//This main let us terminate in an elegant and simple way, returning the error message
fn main() -> Result<(), String> {
    //Init and check orchestrator
    let mut orchestrator = Orchestrator::new()?;
    
    let _initialization = orchestrator.initialize_galaxy()?;


    Ok(())
}
/*
fn main() {
    //Init orchestrator and galaxy
    let gen_orchestrator = Orchestrator::new();

    //Check and get orchestrator initialization
    let orchestrator = match gen_orchestrator {
        Ok(value) => value,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };
}
*/

