extern crate error_chain;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate ffi_utils;
#[macro_use]
extern crate failure;
extern crate hermes;
extern crate libc;
extern crate snips_nlu_ontology_ffi_macros;

type Result<T> = std::result::Result<T, failure::Error>;

mod protocol_handler;
pub use protocol_handler::*;

mod ontology;
pub use ontology::*;

