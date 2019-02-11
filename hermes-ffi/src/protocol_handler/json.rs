use failure::Fallible;
use hermes::ontology::HermesMessage;

use super::protocol_handler::UserData;

pub type CJsonCallback = unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void);

pub fn json_ptr_to_callback<'de, T>(
    ptr: Option<CJsonCallback>,
    user_data: UserData,
) -> Fallible<hermes::Callback<T>>
    where
        T: HermesMessage<'de>,
{
    match ptr {
        Some(ptr) => Ok(hermes::Callback::new(move |payload: &T| {
            let json = serde_json::to_string(&payload).expect("json serialization failed");
            let c_string = std::ffi::CString::new(json).expect("CString::new failed");
            unsafe { ptr(c_string.as_ptr(), user_data.0) }
        })),
        None => failure::bail!("null pointer"),
    }
}

pub fn json_from_slice<'a, T>(v: &'a [u8]) -> Fallible<T> where T: HermesMessage<'a> {
    Ok(serde_json::from_slice(v)?)
}

#[macro_export]
macro_rules! generate_facade_publish_json {
    ($c_symbol:ident = $facade:ty:$method:ident) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(facade: *const $facade) -> ffi_utils::SNIPS_RESULT {
            ffi_utils::wrap!(unsafe { (*facade).extract() }.$method())
        }
    };

    ($c_symbol:ident = $facade:ty:$method:ident($($filter_name:ident)*)) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(
            facade: *const $facade,
            $($filter_name: *const libc::c_char,)*
            message: *const libc::c_char,
        ) -> ffi_utils::SNIPS_RESULT {
            fn fun(
                facade: *const $facade,
                $($filter_name: *const libc::c_char,)*
                message: *const libc::c_char,
            ) -> failure::Fallible<()> {
                use std::ffi::CStr;

                let c_str = unsafe { CStr::from_ptr(message) };
                let message = $crate::json_from_slice(c_str.to_bytes())?;

                unsafe { (*facade).extract() }.$method(
                    $(unsafe { CStr::from_ptr($filter_name) }.to_string_lossy().into_owned(),)*
                    message,
                )
            }
            ffi_utils::wrap!(fun(facade, $($filter_name,)* message))
        }
    };
}

#[macro_export]
macro_rules! generate_facade_subscribe_json {
    ($c_symbol:ident = $facade:ty:$method:ident($($filter_name:ident)*)) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(
            facade: *const $facade,
            $($filter_name: *const libc::c_char,)*
            handler: Option<unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void)>,
        ) -> ffi_utils::SNIPS_RESULT {

            fn fun(facade: *const $facade,
                $($filter_name: *const libc::c_char,)*
                handler: Option<$crate::CJsonCallback>,
            )-> failure::Fallible<()> {
                use std::ffi::CStr;

                let user_data = unsafe { (*facade).user_data().duplicate() };
                let callback = $crate::json_ptr_to_callback(handler, user_data)?;

                unsafe { (*facade).extract() }.$method(
                    $(unsafe { CStr::from_ptr($filter_name) }.to_string_lossy().into_owned(),)*
                    callback,
                )
            }

            ffi_utils::wrap!(fun(facade, $($filter_name,)* handler))
        }
    };
}
