#[cfg(feature = "debug-prints")]
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => { println!($($arg)*) };
}

#[cfg(not(feature = "debug-prints"))]
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        ()
    };
}

mod components;
mod utils_planets;
mod gui;

//This main let us terminate in an elegant and simple way, returning the error message
fn main() -> Result<(), String> {

    gui::galaxy_vis::main()
}
