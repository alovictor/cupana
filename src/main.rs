pub mod machine;
pub mod memory;
pub mod cupana;
pub mod error;

use cupana::Cupana;
use error::CError;

fn main() -> Result<(), CError> {
    let mut emulator = Cupana::new();
    emulator.load_program(&[0x11, 0x01, 0x0A, 0x00, 0x10, 0x02, 0x01, 0x01]);
    emulator.run()?;
    Ok(())
}
