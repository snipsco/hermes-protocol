extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate ffi_utils;
extern crate hermes;
extern crate libc;
extern crate snips_nlu_ontology_ffi_macros;

#[cfg(test)]
extern crate spectral;

mod ontology;
mod protocol_handler;

pub use crate::ontology::*;
pub use crate::protocol_handler::*;

pub fn init_debug_logs() -> failure::Fallible<()> {
    env_logger::try_init()?;
    Ok(())
}
