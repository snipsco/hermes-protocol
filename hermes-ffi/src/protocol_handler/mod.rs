pub mod facades;
#[cfg(feature = "structures")]
pub mod structures;
#[cfg(feature = "json")]
pub mod json;

pub use facades::{CProtocolHandler, UserData};
#[cfg(feature = "structures")]
pub use structures::structure_ptr_to_callback;
#[cfg(feature = "json")]
pub use json::{json_ptr_to_callback, json_from_slice, CJsonCallback};

#[macro_export]
macro_rules! generate_hermes_c_symbols {
    () => {
        #[no_mangle]
        pub extern "C" fn hermes_enable_debug_logs() -> ffi_utils::SNIPS_RESULT {
            ffi_utils::wrap!($crate::init_debug_logs())
        }

        generate_facade_c_symbols!();

        #[cfg(feature = "structures")]
        generate_structures_c_symbols!();

        #[cfg(feature = "json")]
        generate_json_c_symbols!();
    };
}
