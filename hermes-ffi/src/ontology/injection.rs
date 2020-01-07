use std::collections::HashMap;
use std::ptr::null;
use std::slice;

use failure::Fallible;
use failure::ResultExt;
use ffi_utils::*;
use ffi_utils_derive::{CReprOf, AsRust};

use hermes::*;
use super::CMapStringToStringArray;

#[repr(C)]
#[derive(CReprOf, AsRust)]
#[target_type(EntityValue)]
pub struct CEntityValue {
    pub value: *const libc::c_char,
    pub weight: u32,
}

impl Drop for CEntityValue {
    fn drop(&mut self) {
        take_back_c_string!(self.value);
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum SNIPS_INJECTION_KIND {
    SNIPS_INJECTION_KIND_ADD = 1,
    SNIPS_INJECTION_KIND_ADD_FROM_VANILLA = 2,
}

impl CReprOf<hermes::InjectionKind> for SNIPS_INJECTION_KIND {
    fn c_repr_of(input: hermes::InjectionKind) -> Fallible<Self> {
        Ok(match input {
            hermes::InjectionKind::Add => SNIPS_INJECTION_KIND::SNIPS_INJECTION_KIND_ADD,
            hermes::InjectionKind::AddFromVanilla => SNIPS_INJECTION_KIND::SNIPS_INJECTION_KIND_ADD_FROM_VANILLA,
        })
    }
}

impl AsRust<hermes::InjectionKind> for SNIPS_INJECTION_KIND {
    fn as_rust(&self) -> Fallible<hermes::InjectionKind> {
        Ok(match self {
            SNIPS_INJECTION_KIND::SNIPS_INJECTION_KIND_ADD => hermes::InjectionKind::Add,
            SNIPS_INJECTION_KIND::SNIPS_INJECTION_KIND_ADD_FROM_VANILLA => hermes::InjectionKind::AddFromVanilla,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CInjectionRequestOperation {
    pub values: *const CMapStringToStringArray,
    pub kind: SNIPS_INJECTION_KIND,
}

impl Drop for CInjectionRequestOperation {
    fn drop(&mut self) {
        let _ = unsafe { CMapStringToStringArray::drop_raw_pointer(self.values) };
    }
}

impl CReprOf<(hermes::InjectionKind, HashMap<String, Vec<hermes::EntityValue>>)> for CInjectionRequestOperation {
    fn c_repr_of(input: (hermes::InjectionKind, HashMap<String, Vec<hermes::EntityValue>>)) -> Fallible<Self> {
        // FIXME: Ugly shortcut to compile faster. We're losing the weight information.
        let mut hash = HashMap::with_capacity(input.1.capacity());
        for (key, entity_values) in input.1 {
            let entity_values = entity_values.into_iter().map(|v| v.value).collect();
            hash.insert(key, entity_values);
        }

        Ok(Self {
            kind: SNIPS_INJECTION_KIND::c_repr_of(input.0)?,
            values: CMapStringToStringArray::c_repr_of(hash)?.into_raw_pointer(),
        })
    }
}

impl AsRust<(hermes::InjectionKind, HashMap<String, Vec<hermes::EntityValue>>)> for CInjectionRequestOperation {
    fn as_rust(&self) -> Fallible<(hermes::InjectionKind, HashMap<String, Vec<hermes::EntityValue>>)> {
        let values = unsafe { CMapStringToStringArray::raw_borrow(self.values) }?.as_rust()?;

        // FIXME: Ugly shortcut to compile faster. We're losing the weight information.
        let mut hash = HashMap::with_capacity(values.capacity());
        for (key, entity_values) in values {
            let entity_values = entity_values
                .into_iter()
                .map(|value| hermes::EntityValue { value, weight: 1 })
                .collect();
            hash.insert(key, entity_values);
        }

        Ok((self.kind.as_rust()?, hash))
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CInjectionRequestMessage {
    operations: *const CArray<CInjectionRequestOperation>,
    lexicon: *const CMapStringToStringArray,
    /// Nullable
    cross_language: *const libc::c_char,
    /// Nullable
    id: *const libc::c_char,
}

impl Drop for CInjectionRequestMessage {
    fn drop(&mut self) {
        //let _ = unsafe { CInjectionRequestOperations::drop_raw_pointer(self.operations) };
        let _ = unsafe { CMapStringToStringArray::drop_raw_pointer(self.lexicon) };
        take_back_nullable_c_string!(self.cross_language);
        take_back_nullable_c_string!(self.id);
    }
}

impl CReprOf<hermes::InjectionRequestMessage> for CInjectionRequestMessage {
    fn c_repr_of(input: hermes::InjectionRequestMessage) -> Fallible<Self> {
        Ok(Self {
            operations: CArray::<CInjectionRequestOperation>::c_repr_of(input.operations)?.into_raw_pointer(),
            lexicon: CMapStringToStringArray::c_repr_of(input.lexicon)?.into_raw_pointer(),
            cross_language: convert_to_nullable_c_string!(input.cross_language),
            id: convert_to_nullable_c_string!(input.id),
        })
    }
}

impl AsRust<hermes::InjectionRequestMessage> for CInjectionRequestMessage {
    fn as_rust(&self) -> Fallible<hermes::InjectionRequestMessage> {
        let lexicon = unsafe { CMapStringToStringArray::raw_borrow(self.lexicon) }?.as_rust()?;
        Ok(hermes::InjectionRequestMessage {
            operations: self.operations.as_rust()?,
            lexicon,
            cross_language: create_optional_rust_string_from!(self.cross_language),
            id: create_optional_rust_string_from!(self.id),
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CInjectionStatusMessage {
    pub last_injection_date: *const libc::c_char,
}

unsafe impl Sync for CInjectionStatusMessage {}

impl Drop for CInjectionStatusMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.last_injection_date);
    }
}

impl CReprOf<hermes::InjectionStatusMessage> for CInjectionStatusMessage {
    fn c_repr_of(status: hermes::InjectionStatusMessage) -> Fallible<Self> {
        let last_injection_date_str = status.last_injection_date.map(|d| d.to_rfc3339());

        Ok(Self {
            last_injection_date: convert_to_nullable_c_string!(last_injection_date_str),
        })
    }
}

impl AsRust<hermes::InjectionStatusMessage> for CInjectionStatusMessage {
    fn as_rust(&self) -> Fallible<hermes::InjectionStatusMessage> {
        let last_injection_date = create_optional_rust_string_from!(self.last_injection_date);
        let last_injection_date = if let Some(date_str) = last_injection_date {
            Some(date_str.parse()?)
        } else {
            None
        };

        Ok(hermes::InjectionStatusMessage { last_injection_date })
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(InjectionCompleteMessage)]
pub struct CInjectionCompleteMessage {
    /// Nullable
    #[nullable]
    pub request_id: *const libc::c_char,
}

unsafe impl Sync for CInjectionCompleteMessage {}

impl Drop for CInjectionCompleteMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.request_id);
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(InjectionResetRequestMessage)]
pub struct CInjectionResetRequestMessage {
    /// Nullable
    #[nullable]
    pub request_id: *const libc::c_char,
}

unsafe impl Sync for CInjectionResetRequestMessage {}

impl Drop for CInjectionResetRequestMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.request_id);
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(InjectionResetCompleteMessage)]
pub struct CInjectionResetCompleteMessage {
    /// Nullable
    #[nullable]
    pub request_id: *const libc::c_char,
}

unsafe impl Sync for CInjectionResetCompleteMessage {}

impl Drop for CInjectionResetCompleteMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.request_id);
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::round_trip_test;
    use super::*;
    use hermes::hermes_utils::Example;

    #[test]
    fn round_trip_injection_request_operation() {
        round_trip_test::<_, CInjectionRequestOperation>((hermes::InjectionKind::Add, HashMap::new()));

        let mut test_map = HashMap::new();
        test_map.insert(
            "hello".into(),
            vec![
                hermes::EntityValue {
                    value: "hello".to_string(),
                    weight: 1,
                },
                hermes::EntityValue {
                    value: "world".to_string(),
                    weight: 1,
                },
            ],
        );
        test_map.insert(
            "foo".into(),
            vec![
                hermes::EntityValue {
                    value: "bar".to_string(),
                    weight: 1,
                },
                hermes::EntityValue {
                    value: "baz".to_string(),
                    weight: 1,
                },
            ],
        );

        round_trip_test::<_, CInjectionRequestOperation>((hermes::InjectionKind::Add, test_map));
    }

    #[test]
    fn round_trip_injection_request() {
        let mut injections = HashMap::new();
        injections.insert(
            "hello".into(),
            vec![
                hermes::EntityValue {
                    value: "hello".to_string(),
                    weight: 1,
                },
                hermes::EntityValue {
                    value: "world".to_string(),
                    weight: 1,
                },
            ],
        );
        injections.insert(
            "foo".into(),
            vec![
                hermes::EntityValue {
                    value: "bar".to_string(),
                    weight: 1,
                },
                hermes::EntityValue {
                    value: "baz".to_string(),
                    weight: 1,
                },
            ],
        );

        let mut lexicon = HashMap::new();
        lexicon.insert(
            "this".into(),
            vec!["is ".to_string(), "a".to_string(), "lexicon".to_string()],
        );
        lexicon.insert("baz".into(), vec!["bar".to_string(), "foo".to_string()]);

        round_trip_test::<_, CInjectionRequestMessage>(hermes::InjectionRequestMessage {
            cross_language: Some("en".to_string()),
            operations: vec![
                (hermes::InjectionKind::Add, HashMap::new()),
                (hermes::InjectionKind::Add, injections),
            ],
            lexicon,
            id: Some("some id".to_string()),
        });
    }

    #[test]
    fn round_trip_injection_status() {
        round_trip_test::<_, CInjectionStatusMessage>(hermes::InjectionStatusMessage::minimal_example());
        round_trip_test::<_, CInjectionStatusMessage>(hermes::InjectionStatusMessage::full_example());
    }

    #[test]
    fn round_trip_injection_complete() {
        round_trip_test::<_, CInjectionCompleteMessage>(hermes::InjectionCompleteMessage::minimal_example());
        round_trip_test::<_, CInjectionCompleteMessage>(hermes::InjectionCompleteMessage::full_example());
    }

    #[test]
    fn round_trip_injection_reset_request() {
        round_trip_test::<_, CInjectionResetRequestMessage>(hermes::InjectionResetRequestMessage::minimal_example());
        round_trip_test::<_, CInjectionResetRequestMessage>(hermes::InjectionResetRequestMessage::full_example());
    }

    #[test]
    fn round_trip_injection_reset_complete() {
        round_trip_test::<_, CInjectionResetCompleteMessage>(hermes::InjectionResetCompleteMessage::minimal_example());
        round_trip_test::<_, CInjectionResetCompleteMessage>(hermes::InjectionResetCompleteMessage::full_example());
    }
}
