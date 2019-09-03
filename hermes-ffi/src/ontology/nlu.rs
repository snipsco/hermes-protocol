use std::ptr::null;
use std::slice;

use failure::Fallible;
use failure::ResultExt;
use ffi_utils::*;
use snips_nlu_ontology_ffi_macros::*;

use crate::ontology::asr::CAsrTokenArray;

#[repr(C)]
#[derive(Debug)]
pub struct CNluQueryMessage {
    pub input: *const libc::c_char,
    /// Nullable
    pub asr_tokens: *const CAsrTokenArray,
    /// Nullable
    pub intent_filter: *const CStringArray,
    /// Nullable
    pub id: *const libc::c_char,
    /// Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluQueryMessage {}

impl CNluQueryMessage {
    pub fn from(input: hermes::NluQueryMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluQueryMessage> for CNluQueryMessage {
    fn c_repr_of(input: hermes::NluQueryMessage) -> Fallible<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            asr_tokens: if let Some(asr_tokens) = input.asr_tokens {
                CAsrTokenArray::c_repr_of(asr_tokens)?.into_raw_pointer()
            } else {
                null()
            },
            intent_filter: convert_to_nullable_c_string_array!(input.intent_filter),
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::NluQueryMessage> for CNluQueryMessage {
    fn as_rust(&self) -> Fallible<hermes::NluQueryMessage> {
        Ok(hermes::NluQueryMessage {
            input: create_rust_string_from!(self.input),
            asr_tokens: match unsafe { self.asr_tokens.as_ref() } {
                Some(tokens) => Some(unsafe { CAsrTokenArray::raw_borrow(tokens)? }.as_rust()?),
                None => None,
            },
            intent_filter: create_optional_rust_vec_string_from!(self.intent_filter),
            id: create_optional_rust_string_from!(self.id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CNluQueryMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_string_array!(self.intent_filter);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
        let _ = unsafe { CAsrTokenArray::drop_raw_pointer(self.asr_tokens) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlotQueryMessage {
    pub input: *const libc::c_char,
    pub asr_tokens: *const CAsrTokenArray,
    pub intent_name: *const libc::c_char,
    pub slot_name: *const libc::c_char,
    /// Nullable
    pub id: *const libc::c_char,
    /// Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluSlotQueryMessage {}

impl CNluSlotQueryMessage {
    pub fn from(input: hermes::NluSlotQueryMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluSlotQueryMessage> for CNluSlotQueryMessage {
    fn c_repr_of(input: hermes::NluSlotQueryMessage) -> Fallible<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            asr_tokens: if let Some(asr_tokens) = input.asr_tokens {
                CAsrTokenArray::c_repr_of(asr_tokens)?.into_raw_pointer()
            } else {
                null()
            },
            intent_name: convert_to_c_string!(input.intent_name),
            slot_name: convert_to_c_string!(input.slot_name),
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::NluSlotQueryMessage> for CNluSlotQueryMessage {
    fn as_rust(&self) -> Fallible<hermes::NluSlotQueryMessage> {
        Ok(hermes::NluSlotQueryMessage {
            input: create_rust_string_from!(self.input),
            asr_tokens: match unsafe { self.asr_tokens.as_ref() } {
                Some(tokens) => Some(unsafe { CAsrTokenArray::raw_borrow(tokens)? }.as_rust()?),
                None => None,
            },
            intent_name: create_rust_string_from!(self.intent_name),
            slot_name: create_rust_string_from!(self.slot_name),
            id: create_optional_rust_string_from!(self.id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CNluSlotQueryMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_c_string!(self.intent_name);
        take_back_c_string!(self.slot_name);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
        let _ = unsafe { CAsrTokenArray::drop_raw_pointer(self.asr_tokens) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlotMessage {
    /// Nullable
    pub id: *const libc::c_char,
    pub input: *const libc::c_char,
    pub intent_name: *const libc::c_char,
    /// Nullable
    pub slot: *const CNluSlot,
    /// Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluSlotMessage {}

impl CNluSlotMessage {
    pub fn from(input: hermes::NluSlotMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluSlotMessage> for CNluSlotMessage {
    fn c_repr_of(input: hermes::NluSlotMessage) -> Fallible<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            input: convert_to_c_string!(input.input),
            intent_name: convert_to_c_string!(input.intent_name),
            slot: if let Some(s) = input.slot {
                CNluSlot::c_repr_of(s)?.into_raw_pointer()
            } else {
                null()
            },
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::NluSlotMessage> for CNluSlotMessage {
    fn as_rust(&self) -> Fallible<hermes::NluSlotMessage> {
        Ok(hermes::NluSlotMessage {
            id: create_optional_rust_string_from!(self.id),
            input: create_rust_string_from!(self.input),
            intent_name: create_rust_string_from!(self.intent_name),
            session_id: create_optional_rust_string_from!(self.session_id),
            slot: match unsafe { self.slot.as_ref() } {
                Some(slot) => Some(unsafe { CNluSlot::raw_borrow(slot)? }.as_rust()?),
                None => None,
            },
        })
    }
}

impl Drop for CNluSlotMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.input);
        take_back_c_string!(self.intent_name);
        take_back_nullable_c_string!(self.session_id);
        let _ = unsafe { CNluSlot::drop_raw_pointer(self.slot) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluIntentNotRecognizedMessage {
    pub input: *const libc::c_char,
    /// Nullable
    pub id: *const libc::c_char,
    /// Nullable
    pub session_id: *const libc::c_char,
    pub confidence_score: libc::c_float,
    /// Nullable
    pub alternatives: *const CNluIntentAlternativeArray,
}

unsafe impl Sync for CNluIntentNotRecognizedMessage {}

impl CNluIntentNotRecognizedMessage {
    pub fn from(input: hermes::NluIntentNotRecognizedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluIntentNotRecognizedMessage> for CNluIntentNotRecognizedMessage {
    fn c_repr_of(input: hermes::NluIntentNotRecognizedMessage) -> Fallible<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
            confidence_score: input.confidence_score,
            alternatives: if let Some(alternatives) = input.alternatives {
                CNluIntentAlternativeArray::c_repr_of(alternatives)?.into_raw_pointer()
            } else {
                null()
            },
        })
    }
}

impl AsRust<hermes::NluIntentNotRecognizedMessage> for CNluIntentNotRecognizedMessage {
    fn as_rust(&self) -> Fallible<hermes::NluIntentNotRecognizedMessage> {
        Ok(hermes::NluIntentNotRecognizedMessage {
            input: create_rust_string_from!(self.input),
            id: create_optional_rust_string_from!(self.id),
            session_id: create_optional_rust_string_from!(self.session_id),
            confidence_score: self.confidence_score,
            alternatives: if !self.alternatives.is_null() {
                Some(unsafe { CNluIntentAlternativeArray::raw_borrow(self.alternatives) }?.as_rust()?)
            } else {
                None
            },
        })
    }
}

impl Drop for CNluIntentNotRecognizedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
        if !self.alternatives.is_null() {
            let _ = unsafe { CNluIntentAlternativeArray::drop_raw_pointer(self.alternatives) };
        };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlot {
    pub nlu_slot: *const CSlot,
}

impl CReprOf<hermes::NluSlot> for CNluSlot {
    fn c_repr_of(input: hermes::NluSlot) -> Fallible<Self> {
        Ok(Self {
            nlu_slot: CSlot::from(input.nlu_slot).into_raw_pointer(),
        })
    }
}

impl AsRust<hermes::NluSlot> for CNluSlot {
    fn as_rust(&self) -> Fallible<hermes::NluSlot> {
        Ok(hermes::NluSlot {
            nlu_slot: unsafe { &*self.nlu_slot }.as_rust()?,
        })
    }
}

impl Drop for CNluSlot {
    fn drop(&mut self) {
        let _ = unsafe { CSlot::from_raw_pointer(self.nlu_slot) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlotArray {
    pub entries: *const *const CNluSlot,
    pub count: libc::c_int,
}

impl CReprOf<Vec<hermes::NluSlot>> for CNluSlotArray {
    fn c_repr_of(input: Vec<hermes::NluSlot>) -> Fallible<Self> {
        let array = Self {
            count: input.len() as _,
            entries: Box::into_raw(
                input
                    .into_iter()
                    .map(|e| CNluSlot::c_repr_of(e).map(RawPointerConverter::into_raw_pointer))
                    .collect::<Fallible<Vec<_>>>()
                    .context("Could not convert map to C Repr")?
                    .into_boxed_slice(),
            ) as *const *const _,
        };
        Ok(array)
    }
}

impl AsRust<Vec<hermes::NluSlot>> for CNluSlotArray {
    fn as_rust(&self) -> Fallible<Vec<hermes::NluSlot>> {
        let mut result = Vec::with_capacity(self.count as usize);
        if self.count > 0 {
            for e in unsafe { slice::from_raw_parts(self.entries, self.count as usize) } {
                result.push(unsafe { CNluSlot::raw_borrow(*e) }?.as_rust()?);
            }
        }
        Ok(result)
    }
}

impl Drop for CNluSlotArray {
    fn drop(&mut self) {
        unsafe {
            let slots = Box::from_raw(std::slice::from_raw_parts_mut(
                self.entries as *mut *mut CNluSlot,
                self.count as usize,
            ));

            for e in slots.iter() {
                let _ = CNluSlot::drop_raw_pointer(*e);
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluIntentMessage {
    /// Nullable
    pub id: *const libc::c_char,
    pub input: *const libc::c_char,
    pub intent: *const CNluIntentClassifierResult,
    /// Nullable
    pub slots: *const CNluSlotArray,
    /// Nullable
    pub session_id: *const libc::c_char,
    /// Nullable
    pub alternatives: *const CNluIntentAlternativeArray,
}

unsafe impl Sync for CNluIntentMessage {}

impl CNluIntentMessage {
    pub fn from(input: hermes::NluIntentMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluIntentMessage> for CNluIntentMessage {
    fn c_repr_of(input: hermes::NluIntentMessage) -> Fallible<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            input: convert_to_c_string!(input.input),
            intent: CNluIntentClassifierResult::c_repr_of(input.intent)?.into_raw_pointer(),
            slots: if !input.slots.is_empty() {
                CNluSlotArray::c_repr_of(input.slots)?.into_raw_pointer()
            } else {
                null()
            },
            session_id: convert_to_nullable_c_string!(input.session_id),
            alternatives: if let Some(alternatives) = input.alternatives {
                CNluIntentAlternativeArray::c_repr_of(alternatives)?.into_raw_pointer()
            } else {
                null()
            },
        })
    }
}

impl AsRust<hermes::NluIntentMessage> for CNluIntentMessage {
    fn as_rust(&self) -> Fallible<hermes::NluIntentMessage> {
        Ok(hermes::NluIntentMessage {
            session_id: create_optional_rust_string_from!(self.session_id),
            id: create_optional_rust_string_from!(self.id),
            input: create_rust_string_from!(self.input),
            intent: unsafe { CNluIntentClassifierResult::raw_borrow(self.intent) }?.as_rust()?,
            slots: if !self.slots.is_null() {
                unsafe { CNluSlotArray::raw_borrow(self.slots) }?.as_rust()?
            } else {
                vec![]
            },
            alternatives: if !self.alternatives.is_null() {
                Some(unsafe { CNluIntentAlternativeArray::raw_borrow(self.alternatives) }?.as_rust()?)
            } else {
                None
            },
        })
    }
}

impl Drop for CNluIntentMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.input);
        let _ = unsafe { CNluIntentClassifierResult::drop_raw_pointer(self.intent) };
        if !self.slots.is_null() {
            let _ = unsafe { CNluSlotArray::drop_raw_pointer(self.slots) };
        }
        take_back_nullable_c_string!(self.session_id);
        if !self.alternatives.is_null() {
            let _ = unsafe { CNluIntentAlternativeArray::drop_raw_pointer(self.alternatives) };
        };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluIntentAlternative {
    /// Nullable, name of the intent detected (null = no intent)
    pub intent_name: *const libc::c_char,
    /// Nullable
    pub slots: *const CNluSlotArray,
    /// Between 0 and 1
    pub confidence_score: libc::c_float,
}

impl CReprOf<hermes::NluIntentAlternative> for CNluIntentAlternative {
    fn c_repr_of(input: hermes::NluIntentAlternative) -> Fallible<Self> {
        Ok(Self {
            intent_name: convert_to_nullable_c_string!(input.intent_name),
            confidence_score: input.confidence_score,
            slots: if !input.slots.is_empty() {
                CNluSlotArray::c_repr_of(input.slots)?.into_raw_pointer()
            } else {
                null()
            },
        })
    }
}

impl AsRust<hermes::NluIntentAlternative> for CNluIntentAlternative {
    fn as_rust(&self) -> Fallible<hermes::NluIntentAlternative> {
        Ok(hermes::NluIntentAlternative {
            intent_name: create_optional_rust_string_from!(self.intent_name),
            confidence_score: self.confidence_score,
            slots: if !self.slots.is_null() {
                unsafe { CNluSlotArray::raw_borrow(self.slots) }?.as_rust()?
            } else {
                vec![]
            },
        })
    }
}

impl Drop for CNluIntentAlternative {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.intent_name);
        if !self.slots.is_null() {
            let _ = unsafe { CNluSlotArray::drop_raw_pointer(self.slots) };
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluIntentAlternativeArray {
    pub entries: *const *const CNluIntentAlternative,
    pub count: libc::c_int,
}

impl CReprOf<Vec<hermes::NluIntentAlternative>> for CNluIntentAlternativeArray {
    fn c_repr_of(input: Vec<hermes::NluIntentAlternative>) -> Fallible<Self> {
        let array = Self {
            count: input.len() as _,
            entries: Box::into_raw(
                input
                    .into_iter()
                    .map(|e| CNluIntentAlternative::c_repr_of(e).map(RawPointerConverter::into_raw_pointer))
                    .collect::<Fallible<Vec<_>>>()
                    .context("Could not convert map to C Repr")?
                    .into_boxed_slice(),
            ) as *const *const _,
        };
        Ok(array)
    }
}

impl AsRust<Vec<hermes::NluIntentAlternative>> for CNluIntentAlternativeArray {
    fn as_rust(&self) -> Fallible<Vec<hermes::NluIntentAlternative>> {
        let mut result = Vec::with_capacity(self.count as usize);

        if self.count > 0 {
            for e in unsafe { slice::from_raw_parts(self.entries, self.count as usize) } {
                result.push(unsafe { CNluIntentAlternative::raw_borrow(*e) }?.as_rust()?);
            }
        }
        Ok(result)
    }
}

impl Drop for CNluIntentAlternativeArray {
    fn drop(&mut self) {
        unsafe {
            let slots = Box::from_raw(std::slice::from_raw_parts_mut(
                self.entries as *mut *mut CNluIntentAlternative,
                self.count as usize,
            ));

            for e in slots.iter() {
                let _ = CNluIntentAlternative::drop_raw_pointer(*e);
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
/// Result of the intent classifier
pub struct CNluIntentClassifierResult {
    /// Name of the intent detected
    pub intent_name: *const libc::c_char,
    /// Between 0 and 1
    pub confidence_score: libc::c_float,
}

impl CReprOf<hermes::NluIntentClassifierResult> for CNluIntentClassifierResult {
    fn c_repr_of(input: hermes::NluIntentClassifierResult) -> Fallible<Self> {
        Ok(Self {
            intent_name: convert_to_c_string!(input.intent_name),
            confidence_score: input.confidence_score,
        })
    }
}

impl AsRust<hermes::NluIntentClassifierResult> for CNluIntentClassifierResult {
    fn as_rust(&self) -> Fallible<hermes::NluIntentClassifierResult> {
        Ok(hermes::NluIntentClassifierResult {
            intent_name: create_rust_string_from!(self.intent_name),
            confidence_score: self.confidence_score,
        })
    }
}

impl Drop for CNluIntentClassifierResult {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.intent_name);
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::round_trip_test;
    use super::*;

    #[test]
    fn round_trip_intent_classifier_result() {
        round_trip_test::<_, CNluIntentClassifierResult>(hermes::NluIntentClassifierResult {
            intent_name: "MakeCoffee".into(),
            confidence_score: 0.5,
        });

        round_trip_test::<_, CNluIntentClassifierResult>(hermes::NluIntentClassifierResult {
            intent_name: "MakeCoffee".into(),
            confidence_score: 0.5,
        });
    }
}
