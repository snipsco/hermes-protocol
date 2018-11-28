#![allow(non_camel_case_types)]

use failure::ResultExt;
use ffi_utils::{AsRust, CReprOf, CStringArray, RawBorrow, RawPointerConverter};
use hermes;
use libc;
use Result;
use std::collections::HashMap;
use std::ptr::null;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct CSiteMessage {
    pub site_id: *const libc::c_char,
    /// Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CSiteMessage {}

impl CSiteMessage {
    pub fn from(input: hermes::SiteMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SiteMessage> for CSiteMessage {
    fn c_repr_of(input: hermes::SiteMessage) -> Result<Self> {
        Ok(Self {
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::SiteMessage> for CSiteMessage {
    fn as_rust(&self) -> Result<hermes::SiteMessage> {
        Ok(hermes::SiteMessage {
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CSiteMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

pub mod hotword;

pub use hotword::*;

pub mod asr;

pub use asr::*;

pub mod nlu;

pub use nlu::*;

pub mod audio_server;

pub use audio_server::*;

pub mod tts;

pub use tts::*;

pub mod dialogue;

pub use dialogue::*;

#[repr(C)]
#[derive(Debug)]
pub struct CVersionMessage {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl CVersionMessage {
    pub fn from(input: &hermes::VersionMessage) -> Result<Self> {
        Ok(Self {
            major: input.version.major,
            minor: input.version.minor,
            patch: input.version.patch,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CErrorMessage {
    /// Nullable
    pub session_id: *const libc::c_char,
    pub error: *const libc::c_char,
    /// Nullable
    pub context: *const libc::c_char,
}

impl CErrorMessage {
    pub fn from(input: hermes::ErrorMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_nullable_c_string!(input.session_id),
            error: convert_to_c_string!(input.error),
            context: convert_to_nullable_c_string!(input.context),
        })
    }
}

impl Drop for CErrorMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.session_id);
        take_back_c_string!(self.error);
        take_back_nullable_c_string!(self.context);
    }
}

pub mod injection;

pub use injection::*;


#[repr(C)]
#[derive(Debug)]
pub struct CMapStringToStringArrayEntry {
    pub key: *const libc::c_char,
    pub value: *const CStringArray,
}

impl Drop for CMapStringToStringArrayEntry {
    fn drop(&mut self) {
        take_back_c_string!(self.key);
    }
}

impl CReprOf<(String, Vec<String>)> for CMapStringToStringArrayEntry {
    fn c_repr_of(input: (String, Vec<String>)) -> Result<Self> {
        Ok( Self {
            key: convert_to_c_string!(input.0),
            value: CStringArray::c_repr_of(input.1)?.into_raw_pointer(),
        })
    }
}

impl AsRust<(String, Vec<String>)> for CMapStringToStringArrayEntry {
    fn as_rust(&self) -> Result<(String, Vec<String>)> {
        Ok((
            create_rust_string_from!(self.key),
            unsafe { CStringArray::raw_borrow(self.value) }?.as_rust()?
        ))
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CMapStringToStringArray {
    pub entries: *const *const CMapStringToStringArrayEntry,
    pub count: libc::c_int,
}

impl Drop for CMapStringToStringArray {
    fn drop(&mut self) {
        let _ = unsafe {
            for e in Box::from_raw(::std::slice::from_raw_parts_mut(
                self.entries as *mut *mut CMapStringToStringArrayEntry,
                self.count as usize,
            )).iter() {
                let _ = CMapStringToStringArrayEntry::drop_raw_pointer(*e).unwrap();
            }
        };
    }
}

impl CReprOf<HashMap<String, Vec<String>>> for CMapStringToStringArray {
    fn c_repr_of(input: HashMap<String, Vec<String>>) -> Result<Self> {
        let array = Self {
            count: input.len() as libc::c_int,
            entries: Box::into_raw(
                input
                    .into_iter()
                    .map(|e| CMapStringToStringArrayEntry::c_repr_of(e).map(|c| c.into_raw_pointer()))
                    .collect::<Result<Vec<*const CMapStringToStringArrayEntry>>>()
                    .context("Could not convert map to C Repr")?
                    .into_boxed_slice(),
            ) as *const *const CMapStringToStringArrayEntry,
        };
        Ok(array)
    }
}

impl AsRust<HashMap<String, Vec<String>>> for CMapStringToStringArray {
    fn as_rust(&self) -> Result<HashMap<String, Vec<String>>> {
        let mut result = HashMap::with_capacity(self.count as usize);
        for e in unsafe { slice::from_raw_parts(self.entries, self.count as usize) } {
            let (key, value) = unsafe { CMapStringToStringArrayEntry::raw_borrow(*e) }?.as_rust()?;
            result.insert(key, value);
        }

        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use super::*;

    pub fn round_trip_test<T, U>(input: T) where T: Clone + PartialEq + ::std::fmt::Debug, U: CReprOf<T> + AsRust<T> {
        let c = U::c_repr_of(input.clone()).expect("could not convert to c_repr");

        let result = c.as_rust().expect("could not convert back to rust");

        assert_that!(result).is_equal_to(input);
    }



    #[test]
    fn round_trip_map_string_to_string_array_entry() {
        round_trip_test::<_, CMapStringToStringArrayEntry>(
            ("hello".to_string(), vec!["hello".to_string(), "world".to_string()])
        );

        round_trip_test::<_, CMapStringToStringArrayEntry>(
            ("hello".to_string(), vec![])
        );
    }

    #[test]
    fn round_trip_map_string_to_string_array() {
        round_trip_test::<_, CMapStringToStringArray>(HashMap::new());

        let mut test_map = HashMap::new();
        test_map.insert("hello".into(), vec!["hello".to_string(), "world".to_string()]);
        test_map.insert("foo".into(), vec!["bar".to_string(), "baz".to_string()]);

        round_trip_test::<_, CMapStringToStringArray>(test_map);
    }


}
