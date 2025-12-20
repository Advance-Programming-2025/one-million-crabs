#[cfg(feature = "debug-prints")]
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => { println!($($arg)*); };
}

#[cfg(not(feature = "debug-prints"))]
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        ()  // <-- Aggiungi questo per ritornare unit type
    };
}

mod components;

use std::io::{self, Write};
use components::Orchestrator;
use std::env;

//This main let us terminate in an elegant and simple way, returning the error message
fn main() -> Result<(), String> {
    // Load env
    dotenv::dotenv().ok();
    //Init and check orchestrator
    let mut orchestrator = Orchestrator::new()?;

    //Give the absolute path for the init file
    let file_path = env::var("INPUT_FILE")
        .expect("Imposta INPUT_FILE nel file .env o come variabile d'ambiente");

    let _init = orchestrator.initialize_galaxy_by_file(file_path.as_str().trim())?;
    let _running_program = orchestrator.run_example()?;

    Ok(())
}
