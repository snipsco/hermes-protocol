extern crate chrono;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate ffi_utils;
extern crate hermes;
extern crate libc;
extern crate snips_nlu_ontology_ffi_macros;

extern crate env_logger;

#[cfg(test)]
extern crate spectral;

type Result<T> = std::result::Result<T, failure::Error>;

mod protocol_handler;
pub use protocol_handler::*;

mod ontology;
pub use ontology::*;

pub fn init_debug_logs() -> Result<()> {
    Ok(env_logger::try_init()?)
}
