use failure::Fallible;
use ffi_utils::*;
use ffi_utils_derive::{CReprOf, AsRust};
use snips_nlu_ontology_ffi_macros::*;
use hermes::*;

use crate::ontology::asr::CAsrToken;

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(NluQueryMessage)]
pub struct CNluQueryMessage {
    pub input: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub asr_tokens: *const CArray<CAsrToken>,
    /// Nullable
    #[nullable]
    pub intent_filter: *const CStringArray,
    /// Nullable
    #[nullable]
    pub id: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluQueryMessage {}

impl CNluQueryMessage {
    pub fn from(input: hermes::NluQueryMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl Drop for CNluQueryMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_string_array!(self.intent_filter);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
        if !self.asr_tokens.is_null() {
            let _ = unsafe { CArray::<CAsrToken>::drop_raw_pointer(self.asr_tokens) };
        }
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(NluSlotQueryMessage)]
pub struct CNluSlotQueryMessage {
    pub input: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub asr_tokens: *const CArray<CAsrToken>,
    pub intent_name: *const libc::c_char,
    pub slot_name: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub id: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluSlotQueryMessage {}

impl CNluSlotQueryMessage {
    pub fn from(input: hermes::NluSlotQueryMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl Drop for CNluSlotQueryMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_c_string!(self.intent_name);
        take_back_c_string!(self.slot_name);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
        if !self.asr_tokens.is_null() {
            let _ = unsafe { CArray::<CAsrToken>::drop_raw_pointer(self.asr_tokens) };
        }
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(NluSlotMessage)]
pub struct CNluSlotMessage {
    /// Nullable
    #[nullable]
    pub id: *const libc::c_char,
    pub input: *const libc::c_char,
    pub intent_name: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub slot: *const CNluSlot,
    /// Nullable
    #[nullable]
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluSlotMessage {}

impl CNluSlotMessage {
    pub fn from(input: hermes::NluSlotMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl Drop for CNluSlotMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.input);
        take_back_c_string!(self.intent_name);
        take_back_nullable_c_string!(self.session_id);
        if !self.slot.is_null() {
            let _ = unsafe { CNluSlot::drop_raw_pointer(self.slot) };
        }
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(NluIntentNotRecognizedMessage)]
pub struct CNluIntentNotRecognizedMessage {
    pub input: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub id: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub session_id: *const libc::c_char,
    pub confidence_score: f32,
    /// Nullable
    #[nullable]
    pub alternatives: *const CArray<CNluIntentAlternative>,
}

unsafe impl Sync for CNluIntentNotRecognizedMessage {}

impl CNluIntentNotRecognizedMessage {
    pub fn from(input: hermes::NluIntentNotRecognizedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl Drop for CNluIntentNotRecognizedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
        if !self.alternatives.is_null() {
            let _ = unsafe { CArray::<CNluIntentAlternative>::drop_raw_pointer(self.alternatives) };
        }
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
        let _ = unsafe { CSlot::drop_raw_pointer(self.nlu_slot) };
    }
}

pub type CNluSlotArray = CArray<CNluSlot>;

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(NluIntentMessage)]
pub struct CNluIntentMessage {
    /// Nullable
    #[nullable]
    pub id: *const libc::c_char,
    pub input: *const libc::c_char,
    pub intent: *const CNluIntentClassifierResult,
    /// Nullable
    pub slots: *const CArray<CNluSlot>,
    /// Nullable
    #[nullable]
    pub session_id: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub alternatives: *const CArray<CNluIntentAlternative>,
}

unsafe impl Sync for CNluIntentMessage {}

impl CNluIntentMessage {
    pub fn from(input: hermes::NluIntentMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl Drop for CNluIntentMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.input);
        if !self.alternatives.is_null() {
            let _ = unsafe { CArray::<CNluIntentAlternative>::drop_raw_pointer(self.alternatives) };
        }
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(NluIntentAlternative)]
pub struct CNluIntentAlternative {
    /// Nullable, name of the intent detected (null = no intent)
    #[nullable]
    pub intent_name: *const libc::c_char,
    /// Nullable
    pub slots: *const CArray<CNluSlot>,
    /// Between 0 and 1
    pub confidence_score: f32,
}

impl Drop for CNluIntentAlternative {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.intent_name);
        if !self.slots.is_null() {
            let _ = unsafe { CArray::<CNluSlot>::drop_raw_pointer(self.slots) };
        }
    }
}

pub type CNluIntentAlternativeArray = CArray<CNluIntentAlternative>;

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(NluIntentClassifierResult)]
/// Result of the intent classifier
pub struct CNluIntentClassifierResult {
    /// Name of the intent detected
    pub intent_name: *const libc::c_char,
    /// Between 0 and 1
    pub confidence_score: f32,
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
    use hermes::hermes_utils::Example;

    #[test]
    fn round_trip_intent_classifier_result() {
        round_trip_test::<_, CNluIntentClassifierResult>(hermes::NluIntentClassifierResult::minimal_example());
        round_trip_test::<_, CNluIntentClassifierResult>(hermes::NluIntentClassifierResult::full_example());
    }
}
