mod protocol_handler;
pub mod ontology;

pub use crate::ontology::*;
pub use crate::protocol_handler::*;

pub fn init_debug_logs() -> failure::Fallible<()> {
    env_logger::try_init()?;
    Ok(())
}
