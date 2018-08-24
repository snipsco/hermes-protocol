#![allow(non_camel_case_types)]

use failure::ResultExt;
use ffi_utils::{AsRust, CReprOf, CStringArray, RawBorrow, RawPointerConverter};
use hermes;
use libc;
use Result;
use snips_nlu_ontology_ffi_macros::{CIntentClassifierResult, CSlot};
use std::collections::HashMap;
use std::ptr::null;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct CSiteMessage {
    pub site_id: *const libc::c_char,
    // Nullable
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

#[repr(C)]
#[derive(Debug)]
pub struct CHotwordDetectedMessage {
    pub site_id: *const libc::c_char,
    pub model_id: *const libc::c_char,
}

unsafe impl Sync for CHotwordDetectedMessage {}

impl CReprOf<hermes::HotwordDetectedMessage> for CHotwordDetectedMessage {
    fn c_repr_of(input: hermes::HotwordDetectedMessage) -> Result<Self> {
        Ok(Self {
            site_id: convert_to_c_string!(input.site_id),
            model_id: convert_to_c_string!(input.model_id),
        })
    }
}

impl AsRust<hermes::HotwordDetectedMessage> for CHotwordDetectedMessage {
    fn as_rust(&self) -> Result<hermes::HotwordDetectedMessage> {
        Ok(hermes::HotwordDetectedMessage {
            site_id: create_rust_string_from!(self.site_id),
            model_id: create_rust_string_from!(self.model_id),
            model_version: None,
            model_type: None,
            current_sensitivity: None,
        })
    }
}

impl Drop for CHotwordDetectedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.site_id);
        take_back_c_string!(self.model_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CTextCapturedMessage {
    pub text: *const libc::c_char,
    // Nullable
    pub tokens_confidence: *const CAsrTokenConfidenceArray,
    pub likelihood: f32,
    pub seconds: f32,
    pub site_id: *const libc::c_char,
    // Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CTextCapturedMessage {}

impl CTextCapturedMessage {
    pub fn from(input: hermes::TextCapturedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::TextCapturedMessage> for CTextCapturedMessage {
    fn c_repr_of(input: hermes::TextCapturedMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            tokens_confidence: if let Some(tokens_confidence) = input.tokens_confidence {
                CAsrTokenConfidenceArray::c_repr_of(tokens_confidence)?.into_raw_pointer()
            } else {
                null()
            },
            likelihood: input.likelihood,
            seconds: input.seconds,
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::TextCapturedMessage> for CTextCapturedMessage {
    fn as_rust(&self) -> Result<hermes::TextCapturedMessage> {
        Ok(hermes::TextCapturedMessage {
            text: create_rust_string_from!(self.text),
            likelihood: self.likelihood,
            tokens_confidence: match unsafe { self.tokens_confidence.as_ref() } {
                Some(tokens) => Some(unsafe { CAsrTokenConfidenceArray::raw_borrow(tokens)? }.as_rust()?),
                None => None,
            },
            seconds: self.seconds,
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CTextCapturedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.text);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
        let _ = unsafe { CAsrTokenConfidenceArray::drop_raw_pointer(self.tokens_confidence) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluQueryMessage {
    pub input: *const libc::c_char,
    /// Nullable
    pub tokens_confidence: *const CAsrTokenConfidenceArray,
    /// Nullable
    pub intent_filter: *const CStringArray,
    /// Nullable
    pub id: *const libc::c_char,
    /// Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluQueryMessage {}

impl CNluQueryMessage {
    pub fn from(input: hermes::NluQueryMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluQueryMessage> for CNluQueryMessage {
    fn c_repr_of(input: hermes::NluQueryMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            tokens_confidence: if let Some(tokens_confidence) = input.tokens_confidence {
                CAsrTokenConfidenceArray::c_repr_of(tokens_confidence)?.into_raw_pointer()
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
    fn as_rust(&self) -> Result<hermes::NluQueryMessage> {
        Ok(hermes::NluQueryMessage {
            input: create_rust_string_from!(self.input),
            tokens_confidence: match unsafe { self.tokens_confidence.as_ref() } {
                Some(tokens) => Some(unsafe { CAsrTokenConfidenceArray::raw_borrow(tokens)? }.as_rust()?),
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
        let _ = unsafe { CAsrTokenConfidenceArray::drop_raw_pointer(self.tokens_confidence) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlotQueryMessage {
    pub input: *const libc::c_char,
    pub tokens_confidence: *const CAsrTokenConfidenceArray,
    pub intent_name: *const libc::c_char,
    pub slot_name: *const libc::c_char,
    /// Nullable
    pub id: *const libc::c_char,
    /// Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluSlotQueryMessage {}

impl CNluSlotQueryMessage {
    pub fn from(input: hermes::NluSlotQueryMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluSlotQueryMessage> for CNluSlotQueryMessage {
    fn c_repr_of(input: hermes::NluSlotQueryMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            tokens_confidence: if let Some(tokens_confidence) = input.tokens_confidence {
                CAsrTokenConfidenceArray::c_repr_of(tokens_confidence)?.into_raw_pointer()
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
    fn as_rust(&self) -> Result<hermes::NluSlotQueryMessage> {
        Ok(hermes::NluSlotQueryMessage {
            input: create_rust_string_from!(self.input),
            tokens_confidence: match unsafe { self.tokens_confidence.as_ref() } {
                Some(tokens) => Some(unsafe { CAsrTokenConfidenceArray::raw_borrow(tokens)? }.as_rust()?),
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
        let _ = unsafe { CAsrTokenConfidenceArray::drop_raw_pointer(self.tokens_confidence) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CPlayBytesMessage {
    pub id: *const libc::c_char,
    pub wav_bytes: *const u8,
    // Note: we can't use `libc::size_t` because it's not supported by JNA
    pub wav_bytes_len: libc::c_int,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CPlayBytesMessage {}

impl CPlayBytesMessage {
    pub fn from(input: hermes::PlayBytesMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::PlayBytesMessage> for CPlayBytesMessage {
    fn c_repr_of(input: hermes::PlayBytesMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
            wav_bytes_len: input.wav_bytes.len() as libc::c_int,
            wav_bytes: Box::into_raw(input.wav_bytes.into_boxed_slice()) as *const u8,
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::PlayBytesMessage> for CPlayBytesMessage {
    fn as_rust(&self) -> Result<hermes::PlayBytesMessage> {
        Ok(hermes::PlayBytesMessage {
            id: create_rust_string_from!(self.id),
            wav_bytes: unsafe {
                slice::from_raw_parts(self.wav_bytes as *const u8, self.wav_bytes_len as usize)
            }.to_vec(),
            site_id: create_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CPlayBytesMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.id);
        let _ = unsafe {
            Box::from_raw(slice::from_raw_parts_mut(
                self.wav_bytes as *mut u8,
                self.wav_bytes_len as usize,
            ))
        };
        take_back_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CAudioFrameMessage {
    pub wav_frame: *const u8,
    // Note: we can't use `libc::size_t` because it's not supported by JNA
    pub wav_frame_len: libc::c_int,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CAudioFrameMessage {}

impl CAudioFrameMessage {
    pub fn from(input: hermes::AudioFrameMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::AudioFrameMessage> for CAudioFrameMessage {
    fn c_repr_of(input: hermes::AudioFrameMessage) -> Result<Self> {
        Ok(Self {
            wav_frame_len: input.wav_frame.len() as libc::c_int,
            wav_frame: Box::into_raw(input.wav_frame.into_boxed_slice()) as *const u8,
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::AudioFrameMessage> for CAudioFrameMessage {
    fn as_rust(&self) -> Result<hermes::AudioFrameMessage> {
        Ok(hermes::AudioFrameMessage {
            wav_frame: unsafe {
                slice::from_raw_parts(self.wav_frame as *const u8, self.wav_frame_len as usize)
            }.to_vec(),
            site_id: create_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CAudioFrameMessage {
    fn drop(&mut self) {
        let _ = unsafe {
            Box::from_raw(slice::from_raw_parts_mut(
                self.wav_frame as *mut u8,
                self.wav_frame_len as usize,
            ))
        };
        take_back_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CPlayFinishedMessage {
    pub id: *const libc::c_char,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CPlayFinishedMessage {}

impl CPlayFinishedMessage {
    pub fn from(input: hermes::PlayFinishedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::PlayFinishedMessage> for CPlayFinishedMessage {
    fn c_repr_of(input: hermes::PlayFinishedMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::PlayFinishedMessage> for CPlayFinishedMessage {
    fn as_rust(&self) -> Result<hermes::PlayFinishedMessage> {
        Ok(hermes::PlayFinishedMessage {
            id: create_rust_string_from!(self.id),
            site_id: create_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CPlayFinishedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.id);
        take_back_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSayMessage {
    pub text: *const libc::c_char,
    /// Nullable
    pub lang: *const libc::c_char,
    /// Nullable
    pub id: *const libc::c_char,
    pub site_id: *const libc::c_char,
    /// Nullable
    pub session_id: *const libc::c_char,
}

impl CReprOf<hermes::SayMessage> for CSayMessage {
    fn c_repr_of(input: hermes::SayMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            lang: convert_to_nullable_c_string!(input.lang),
            id: convert_to_nullable_c_string!(input.id),
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::SayMessage> for CSayMessage {
    fn as_rust(&self) -> Result<hermes::SayMessage> {
        Ok(hermes::SayMessage {
            text: create_rust_string_from!(self.text),
            lang: create_optional_rust_string_from!(self.lang),
            id: create_optional_rust_string_from!(self.id),
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl CSayMessage {
    pub fn from(input: hermes::SayMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_say_message(&self) -> Result<hermes::SayMessage> {
        self.as_rust()
    }
}

unsafe impl Sync for CSayMessage {}

impl Drop for CSayMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.text);
        take_back_nullable_c_string!(self.lang);
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSayFinishedMessage {
    /// Nullable
    pub id: *const libc::c_char,
    /// Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CSayFinishedMessage {}

impl CSayFinishedMessage {
    pub fn from(input: hermes::SayFinishedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_say_finished_message(&self) -> Result<hermes::SayFinishedMessage> {
        self.as_rust()
    }
}

impl CReprOf<hermes::SayFinishedMessage> for CSayFinishedMessage {
    fn c_repr_of(input: hermes::SayFinishedMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::SayFinishedMessage> for CSayFinishedMessage {
    fn as_rust(&self) -> Result<hermes::SayFinishedMessage> {
        Ok(hermes::SayFinishedMessage {
            id: create_optional_rust_string_from!(self.id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CSayFinishedMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
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
    pub fn from(input: hermes::NluSlotMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluSlotMessage> for CNluSlotMessage {
    fn c_repr_of(input: hermes::NluSlotMessage) -> Result<Self> {
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
    fn as_rust(&self) -> Result<hermes::NluSlotMessage> {
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
}

unsafe impl Sync for CNluIntentNotRecognizedMessage {}

impl CNluIntentNotRecognizedMessage {
    pub fn from(input: hermes::NluIntentNotRecognizedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluIntentNotRecognizedMessage> for CNluIntentNotRecognizedMessage {
    fn c_repr_of(input: hermes::NluIntentNotRecognizedMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::NluIntentNotRecognizedMessage> for CNluIntentNotRecognizedMessage {
    fn as_rust(&self) -> Result<hermes::NluIntentNotRecognizedMessage> {
        Ok(hermes::NluIntentNotRecognizedMessage {
            input: create_rust_string_from!(self.input),
            id: create_optional_rust_string_from!(self.id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CNluIntentNotRecognizedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlot {
    pub confidence: f32,
    pub nlu_slot: *const CSlot,
}

impl CReprOf<hermes::NluSlot> for CNluSlot {
    fn c_repr_of(input: hermes::NluSlot) -> Result<Self> {
        Ok(Self {
            confidence: input.confidence.unwrap_or(-1.),
            nlu_slot: CSlot::from(input.nlu_slot).into_raw_pointer(),
        })
    }
}

impl AsRust<hermes::NluSlot> for CNluSlot {
    fn as_rust(&self) -> Result<hermes::NluSlot> {
        //hermes::NluSlot {
            //confidence: self.confidence,
            //nlu_slot: unimplemented!(),
        //}
        bail!("Missing converter for CSlot, if you need this feature, please tell us !")
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
    fn c_repr_of(input: Vec<hermes::NluSlot>) -> Result<Self> {
        let array = Self {
            count: input.len() as _,
            entries: Box::into_raw(
                input
                    .into_iter()
                    .map(|e| CNluSlot::c_repr_of(e).map(|c| c.into_raw_pointer()))
                    .collect::<Result<Vec<_>>>()
                    .context("Could not convert map to C Repr")?
                    .into_boxed_slice(),
            ) as *const *const _,
        };
        Ok(array)
    }
}

impl AsRust<Vec<hermes::NluSlot>> for CNluSlotArray {
    fn as_rust(&self) -> Result<Vec<hermes::NluSlot>> {
        let mut result = Vec::with_capacity(self.count as usize);

        for e in unsafe { slice::from_raw_parts(self.entries, self.count as usize) } {
            result.push(unsafe { CNluSlot::raw_borrow(*e) }?.as_rust()?);
        }
        Ok(result)
    }
}

impl Drop for CNluSlotArray {
    fn drop(&mut self) {
        let _ = unsafe {
            for e in Box::from_raw(::std::slice::from_raw_parts_mut(
                    self.entries as *mut *mut CNluSlot,
                    self.count as usize,
                    )).iter() {
                let _ = CNluSlot::drop_raw_pointer(*e).unwrap();
            }
        };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluIntentMessage {
    /// Nullable
    pub id: *const libc::c_char,
    pub input: *const libc::c_char,
    pub intent: *const CIntentClassifierResult,
    /// Nullable
    pub slots: *const CNluSlotArray,
    /// Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluIntentMessage {}

impl CNluIntentMessage {
    pub fn from(input: hermes::NluIntentMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluIntentMessage> for CNluIntentMessage {
    fn c_repr_of(input: hermes::NluIntentMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            input: convert_to_c_string!(input.input),
            intent: CIntentClassifierResult::from(input.intent).into_raw_pointer(),
            slots: if let Some(slots) = input.slots {
                CNluSlotArray::c_repr_of(slots)?.into_raw_pointer()
            } else {
                null()
            },
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::NluIntentMessage> for CNluIntentMessage {
    fn as_rust(&self) -> Result<hermes::NluIntentMessage> {
        /*Ok(hermes::NluIntentMessage {
            id: create_optional_rust_string_from!(self.id),
            input: create_rust_string_from!(self.input),
            intent: unsafe {CIntentClassifierResult::raw_borrow(self.intent) }?.as_rust()?, // TODO impl in snips-nlu-ontology
            slots: if self.slots.is_null() { None }  else { unsafe {CSlotList::raw_borrow(self.slots)}?.as_rust()? }, // TODO impl in snips-nlu-ontology
            session_id: create_optional_rust_string_from!(self.session_id),
        })*/
        bail!("Missing converter for CIntentClassifierResult and CSlotList, if you need this feature, please tell us !")
    }
}

impl Drop for CNluIntentMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.input);
        let _ = unsafe { CIntentClassifierResult::from_raw_pointer(self.intent) };
        let _ = unsafe { CNluSlotArray::drop_raw_pointer(self.slots) };
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CIntentMessage {
    pub session_id: *const libc::c_char,
    /// Nullable
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,

    pub input: *const libc::c_char,
    pub intent: *const CIntentClassifierResult,
    /// Nullable
    pub slots: *const CNluSlotArray,
}

unsafe impl Sync for CIntentMessage {}

impl CIntentMessage {
    pub fn from(input: hermes::IntentMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::IntentMessage> for CIntentMessage {
    fn c_repr_of(input: hermes::IntentMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
            input: convert_to_c_string!(input.input),
            intent: Box::into_raw(Box::new(CIntentClassifierResult::from(input.intent))),
            slots: if let Some(slots) = input.slots {
                CNluSlotArray::c_repr_of(slots)?.into_raw_pointer()
            } else {
                null()
            },
        })
    }
}

impl AsRust<hermes::IntentMessage> for CIntentMessage {
    fn as_rust(&self) -> Result<hermes::IntentMessage> {
        /*Ok(hermes::IntentMessage {
            session_id: create_rust_string_from!(self.session_id),
            custom_data: create_optional_rust_string_from!(self.custom_data),
            site_id: create_rust_string_from!(self.site_id),
            input: create_rust_string_from!(self.input),
            intent: unsafe {CIntentClassifierResult::raw_borrow(self.intent) }?.as_rust()?, // TODO impl in snips-nlu-ontology
            slots: if self.slots.is_null() { None }  else { unsafe {CSlotList::raw_borrow(self.slots)}?.as_rust()? }, // TODO impl in snips-nlu-ontology
        })*/
        bail!("Missing converter for CIntentClassifierResult and CSlotList, if you need this feature, please tell us !")
    }
}

impl Drop for CIntentMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
        take_back_c_string!(self.input);
        let _ = unsafe { Box::from_raw(self.intent as *mut CIntentClassifierResult) };
        if !self.slots.is_null() {
            let _ = unsafe { Box::from_raw(self.slots as *mut CNluSlotArray) };
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CIntentNotRecognizedMessage {
    pub site_id: *const libc::c_char,
    pub session_id: *const libc::c_char,
    /// Nullable
    pub input: *const libc::c_char,
    /// Nullable
    pub custom_data: *const libc::c_char,
}

unsafe impl Sync for CIntentNotRecognizedMessage {}

impl CReprOf<hermes::IntentNotRecognizedMessage> for CIntentNotRecognizedMessage {
    fn c_repr_of(input: hermes::IntentNotRecognizedMessage) -> Result<Self> {
        Ok(Self {
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_c_string!(input.session_id),
            input: convert_to_nullable_c_string!(input.input),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
        })
    }
}

impl AsRust<hermes::IntentNotRecognizedMessage> for CIntentNotRecognizedMessage {
    fn as_rust(&self) -> Result<hermes::IntentNotRecognizedMessage> {
        Ok(hermes::IntentNotRecognizedMessage {
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_rust_string_from!(self.session_id),
            input: create_optional_rust_string_from!(self.input),
            custom_data: create_optional_rust_string_from!(self.custom_data),
        })
    }
}

impl Drop for CIntentNotRecognizedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.site_id);
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.input);
        take_back_nullable_c_string!(self.custom_data);
    }
}

pub struct CAsrTokenConfidence {
    pub value: *const libc::c_char,
    pub confidence: f32,
    pub range_start: libc::int32_t,
    pub range_end: libc::int32_t,
}

impl CReprOf<hermes::AsrTokenConfidence> for CAsrTokenConfidence {
    fn c_repr_of(input: hermes::AsrTokenConfidence) -> Result<Self> {
        Ok(Self {
            value: convert_to_c_string!(input.value),
            confidence: input.confidence,
            range_start: input.range_start as libc::int32_t,
            range_end: input.range_end as libc::int32_t,
        })
    }
}

impl AsRust<hermes::AsrTokenConfidence> for CAsrTokenConfidence {
    fn as_rust(&self) -> Result<hermes::AsrTokenConfidence> {
        Ok(hermes::AsrTokenConfidence {
            value: create_rust_string_from!(self.value),
            confidence: self.confidence,
            range_start: self.range_start as usize,
            range_end: self.range_end as usize,
        })
    }
}

impl Drop for CAsrTokenConfidence {
    fn drop(&mut self) {
        take_back_c_string!(self.value);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CAsrTokenConfidenceArray {
    pub entries: *const *const CAsrTokenConfidence,
    pub count: libc::c_int,
}

impl CReprOf<Vec<hermes::AsrTokenConfidence>> for CAsrTokenConfidenceArray {
    fn c_repr_of(input: Vec<hermes::AsrTokenConfidence>) -> Result<Self> {
        let array = Self {
            count: input.len() as _,
            entries: Box::into_raw(
                input
                    .into_iter()
                    .map(|e| CAsrTokenConfidence::c_repr_of(e).map(|c| c.into_raw_pointer()))
                    .collect::<Result<Vec<_>>>()
                    .context("Could not convert map to C Repr")?
                    .into_boxed_slice(),
            ) as *const *const _,
        };
        Ok(array)
    }
}

impl AsRust<Vec<hermes::AsrTokenConfidence>> for CAsrTokenConfidenceArray {
    fn as_rust(&self) -> Result<Vec<hermes::AsrTokenConfidence>> {
        let mut result = Vec::with_capacity(self.count as usize);

        for e in unsafe { slice::from_raw_parts(self.entries, self.count as usize) } {
            result.push(unsafe { CAsrTokenConfidence::raw_borrow(*e) }?.as_rust()?);
        }
        Ok(result)
    }
}

impl Drop for CAsrTokenConfidenceArray {
    fn drop(&mut self) {
        let _ = unsafe {
            for e in Box::from_raw(::std::slice::from_raw_parts_mut(
                    self.entries as *mut *mut CAsrTokenConfidence,
                    self.count as usize,
                    )).iter() {
                let _ = CAsrTokenConfidence::drop_raw_pointer(*e).unwrap();
            }
        };
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum SNIPS_SESSION_INIT_TYPE {
    SNIPS_SESSION_INIT_TYPE_ACTION = 1,
    SNIPS_SESSION_INIT_TYPE_NOTIFICATION = 2,
}

impl SNIPS_SESSION_INIT_TYPE {
    pub fn from(slot_value: &hermes::SessionInit) -> Self {
        match *slot_value {
            hermes::SessionInit::Notification { .. } => SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_NOTIFICATION,
            hermes::SessionInit::Action { .. } => SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_ACTION,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct CActionSessionInit {
    /// Nullable
    text: *const libc::c_char,
    /// Nullable
    intent_filter: *const CStringArray,
    can_be_enqueued: libc::c_uchar,
    send_intent_not_recognized: libc::c_uchar,
}

impl CActionSessionInit {
    pub fn new(
        text: Option<String>,
        intent_filter: Option<Vec<String>>,
        can_be_enqueued: bool,
        send_intent_not_recognized: bool,
    ) -> Result<Self> {
        Ok(Self {
            text: convert_to_nullable_c_string!(text),
            intent_filter: convert_to_nullable_c_string_array!(intent_filter),
            can_be_enqueued: if can_be_enqueued { 1 } else { 0 },
            send_intent_not_recognized: if send_intent_not_recognized { 1 } else { 0 },
        })
    }

    pub fn to_action_session_init(&self) -> Result<hermes::SessionInit> {
        Ok(hermes::SessionInit::Action {
            text: create_optional_rust_string_from!(self.text),
            intent_filter: match unsafe { self.intent_filter.as_ref() } {
                Some(it) => Some(it.as_rust()?),
                None => None,
            },
            can_be_enqueued: self.can_be_enqueued == 1,
            send_intent_not_recognized: self.send_intent_not_recognized == 1,
        })
    }
}

impl Drop for CActionSessionInit {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.text);
        take_back_nullable_c_string_array!(self.intent_filter);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionInit {
    init_type: SNIPS_SESSION_INIT_TYPE,
    /// Points to either a *const char, a *const CActionSessionInit
    value: *const libc::c_void,
}

impl CSessionInit {
    fn from(init: hermes::SessionInit) -> Result<Self> {
        let init_type = SNIPS_SESSION_INIT_TYPE::from(&init);
        let value: *const libc::c_void = match init {
            hermes::SessionInit::Action {
                text,
                intent_filter,
                can_be_enqueued,
                send_intent_not_recognized,
            } => Box::into_raw(Box::new(CActionSessionInit::new(
                text,
                intent_filter,
                can_be_enqueued,
                send_intent_not_recognized
            )?)) as _,
            hermes::SessionInit::Notification { text } => convert_to_c_string!(text) as _,
        };
        Ok(Self { init_type, value })
    }

    fn to_session_init(&self) -> Result<hermes::SessionInit> {
        match self.init_type {
            SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_ACTION => {
                unsafe { (self.value as *const CActionSessionInit).as_ref() }
                    .ok_or_else(|| format_err!("unexpected null pointer in SessionInit value"))?
                    .to_action_session_init()
            }
            SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_NOTIFICATION => Ok(hermes::SessionInit::Notification {
                text: create_rust_string_from!((self.value as *const libc::c_char)
                    .as_ref()
                    .ok_or_else(|| format_err!("unexpected null pointer in SessionInit value"))?),
            }),
        }
    }
}

impl Drop for CSessionInit {
    fn drop(&mut self) {
        match self.init_type {
            SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_ACTION => unsafe {
                let _ = CActionSessionInit::from_raw_pointer(self.value as _);
            },
            SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_NOTIFICATION => {
                take_back_c_string!(self.value as *const libc::c_char);
            }
        };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CStartSessionMessage {
    pub init: CSessionInit,
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CStartSessionMessage {}

impl CStartSessionMessage {
    pub fn from(input: hermes::StartSessionMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_start_session_message(&self) -> Result<hermes::StartSessionMessage> {
        self.as_rust()
    }
}

impl CReprOf<hermes::StartSessionMessage> for CStartSessionMessage {
    fn c_repr_of(input: hermes::StartSessionMessage) -> Result<Self> {
        Ok(Self {
            init: CSessionInit::from(input.init)?,
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_nullable_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::StartSessionMessage> for CStartSessionMessage {
    fn as_rust(&self) -> Result<hermes::StartSessionMessage> {
        Ok(hermes::StartSessionMessage {
            init: self.init.to_session_init()?,
            custom_data: create_optional_rust_string_from!(self.custom_data),
            site_id: create_optional_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CStartSessionMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.custom_data);
        take_back_nullable_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionStartedMessage {
    pub session_id: *const libc::c_char,
    /// Nullable
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,
    /// Nullable
    pub reactivated_from_session_id: *const libc::c_char,
}

unsafe impl Sync for CSessionStartedMessage {}

impl CSessionStartedMessage {
    pub fn from(input: hermes::SessionStartedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SessionStartedMessage> for CSessionStartedMessage {
    fn c_repr_of(input: hermes::SessionStartedMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
            reactivated_from_session_id: convert_to_nullable_c_string!(
                input.reactivated_from_session_id
            ),
        })
    }
}

impl AsRust<hermes::SessionStartedMessage> for CSessionStartedMessage {
    fn as_rust(&self) -> Result<hermes::SessionStartedMessage> {
        Ok(hermes::SessionStartedMessage {
            session_id: create_rust_string_from!(self.session_id),
            custom_data: create_optional_rust_string_from!(self.custom_data),
            site_id: create_rust_string_from!(self.site_id),
            reactivated_from_session_id: create_optional_rust_string_from!(
                self.reactivated_from_session_id
            ),
        })
    }
}

impl Drop for CSessionStartedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.reactivated_from_session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionQueuedMessage {
    pub session_id: *const libc::c_char,
    /// Nullable
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CSessionQueuedMessage {}

impl CSessionQueuedMessage {
    pub fn from(input: hermes::SessionQueuedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SessionQueuedMessage> for CSessionQueuedMessage {
    fn c_repr_of(input: hermes::SessionQueuedMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::SessionQueuedMessage> for CSessionQueuedMessage {
    fn as_rust(&self) -> Result<hermes::SessionQueuedMessage> {
        Ok(hermes::SessionQueuedMessage {
            session_id: create_rust_string_from!(self.session_id),
            custom_data: create_optional_rust_string_from!(self.custom_data),
            site_id: create_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CSessionQueuedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CContinueSessionMessage {
    pub session_id: *const libc::c_char,
    pub text: *const libc::c_char,
    /// Nullable
    pub intent_filter: *const CStringArray,
    /// Nullable
    pub custom_data: *const libc::c_char,
    pub send_intent_not_recognized: libc::c_uchar,
}

unsafe impl Sync for CContinueSessionMessage {}

impl CContinueSessionMessage {
    pub fn from(input: hermes::ContinueSessionMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_continue_session_message(&self) -> Result<hermes::ContinueSessionMessage> {
        self.as_rust()
    }
}

impl CReprOf<hermes::ContinueSessionMessage> for CContinueSessionMessage {
    fn c_repr_of(input: hermes::ContinueSessionMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            text: convert_to_c_string!(input.text),
            intent_filter: convert_to_nullable_c_string_array!(input.intent_filter),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            send_intent_not_recognized: if input.send_intent_not_recognized { 1 } else { 0 },
        })
    }
}

impl AsRust<hermes::ContinueSessionMessage> for CContinueSessionMessage {
    fn as_rust(&self) -> Result<hermes::ContinueSessionMessage> {
        Ok(hermes::ContinueSessionMessage {
            session_id: create_rust_string_from!(self.session_id),
            text: create_rust_string_from!(self.text),
            intent_filter: match unsafe { self.intent_filter.as_ref() } {
                Some(it) => Some(it.as_rust()?),
                None => None,
            },
            custom_data: create_optional_rust_string_from!(self.custom_data),
            send_intent_not_recognized: self.send_intent_not_recognized == 1,
        })
    }
}

impl Drop for CContinueSessionMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_c_string!(self.text);
        take_back_nullable_c_string_array!(self.intent_filter);
        take_back_nullable_c_string!(self.custom_data);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CEndSessionMessage {
    pub session_id: *const libc::c_char,
    /// Nullable
    pub text: *const libc::c_char,
}

unsafe impl Sync for CEndSessionMessage {}

impl CEndSessionMessage {
    pub fn from(input: hermes::EndSessionMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_end_session_message(&self) -> Result<hermes::EndSessionMessage> {
        self.as_rust()
    }
}

impl CReprOf<hermes::EndSessionMessage> for CEndSessionMessage {
    fn c_repr_of(input: hermes::EndSessionMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            text: convert_to_nullable_c_string!(input.text),
        })
    }
}

impl AsRust<hermes::EndSessionMessage> for CEndSessionMessage {
    fn as_rust(&self) -> Result<hermes::EndSessionMessage> {
        Ok(hermes::EndSessionMessage {
            session_id: create_rust_string_from!(self.session_id),
            text: create_optional_rust_string_from!(self.text),
        })
    }
}

impl Drop for CEndSessionMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.text);
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum SNIPS_SESSION_TERMINATION_TYPE {
    SNIPS_SESSION_TERMINATION_TYPE_NOMINAL = 1,
    SNIPS_SESSION_TERMINATION_TYPE_SITE_UNAVAILABLE = 2,
    SNIPS_SESSION_TERMINATION_TYPE_ABORTED_BY_USER = 3,
    SNIPS_SESSION_TERMINATION_TYPE_INTENT_NOT_RECOGNIZED = 4,
    SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT = 5,
    SNIPS_SESSION_TERMINATION_TYPE_ERROR = 6,
}

impl SNIPS_SESSION_TERMINATION_TYPE {
    fn from(termination_type: &hermes::SessionTerminationType) -> SNIPS_SESSION_TERMINATION_TYPE {
        match *termination_type {
            hermes::SessionTerminationType::Nominal => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_NOMINAL
            }
            hermes::SessionTerminationType::SiteUnavailable => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_SITE_UNAVAILABLE
            }
            hermes::SessionTerminationType::AbortedByUser => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_ABORTED_BY_USER
            }
            hermes::SessionTerminationType::IntentNotRecognized => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_INTENT_NOT_RECOGNIZED
            }
            hermes::SessionTerminationType::Timeout => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT
            }
            hermes::SessionTerminationType::Error { .. } => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_ERROR
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionTermination {
    termination_type: SNIPS_SESSION_TERMINATION_TYPE,
    /// Nullable,
    data: *const libc::c_char,
}

impl CSessionTermination {
    fn from(termination: ::hermes::SessionTerminationType) -> Result<Self> {
        let termination_type = SNIPS_SESSION_TERMINATION_TYPE::from(&termination);
        let data: *const libc::c_char = match termination {
            ::hermes::SessionTerminationType::Error { error } => convert_to_c_string!(error),
            _ => null(),
        };
        Ok(Self {
            termination_type,
            data,
        })
    }
}

impl AsRust<hermes::SessionTerminationType> for CSessionTermination {
    fn as_rust(&self) -> Result<hermes::SessionTerminationType> {
        Ok(match self.termination_type {
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_NOMINAL => {
                hermes::SessionTerminationType::Nominal
            }
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_SITE_UNAVAILABLE => {
                hermes::SessionTerminationType::SiteUnavailable
            }
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_ABORTED_BY_USER => {
                hermes::SessionTerminationType::AbortedByUser
            }
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_INTENT_NOT_RECOGNIZED => {
                hermes::SessionTerminationType::IntentNotRecognized
            }
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT => {
                hermes::SessionTerminationType::Timeout
            }
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_ERROR => hermes::SessionTerminationType::Error {
                error: create_rust_string_from!(self.data),
            },
        })
    }
}

impl Drop for CSessionTermination {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.data);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionEndedMessage {
    pub session_id: *const libc::c_char,
    /// Nullable
    pub custom_data: *const libc::c_char,
    pub termination: CSessionTermination,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CSessionEndedMessage {}

impl CSessionEndedMessage {
    pub fn from(input: hermes::SessionEndedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SessionEndedMessage> for CSessionEndedMessage {
    fn c_repr_of(input: hermes::SessionEndedMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            termination: CSessionTermination::from(input.termination)?,
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::SessionEndedMessage> for CSessionEndedMessage {
    fn as_rust(&self) -> Result<hermes::SessionEndedMessage> {
        Ok(hermes::SessionEndedMessage {
            session_id: create_rust_string_from!(self.session_id),
            custom_data: create_optional_rust_string_from!(self.custom_data),
            termination: self.termination.as_rust()?,
            site_id: create_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CSessionEndedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
    }
}

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

#[repr(C)]
#[derive(Debug)]
pub enum SNIPS_INJECTION_KIND {
    SNIPS_INJECTION_KIND_ADD = 1,
    SNIPS_INJECTION_KIND_ADD_FROM_VANILLA = 2,
}

impl CReprOf<hermes::InjectionKind> for SNIPS_INJECTION_KIND {
    fn c_repr_of(input: hermes::InjectionKind) -> Result<Self> {
        Ok(match input {
            hermes::InjectionKind::Add => SNIPS_INJECTION_KIND::SNIPS_INJECTION_KIND_ADD,
            hermes::InjectionKind::AddFromVanilla => SNIPS_INJECTION_KIND::SNIPS_INJECTION_KIND_ADD_FROM_VANILLA,
        })
    }
}

impl AsRust<hermes::InjectionKind> for SNIPS_INJECTION_KIND {
    fn as_rust(&self) -> Result<hermes::InjectionKind> {
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

impl CReprOf<(hermes::InjectionKind, HashMap<String, Vec<String>>)> for CInjectionRequestOperation {
    fn c_repr_of(input: (hermes::InjectionKind, HashMap<String, Vec<String>>)) -> Result<Self> {
        Ok(Self {
            kind: SNIPS_INJECTION_KIND::c_repr_of(input.0)?,
            values: CMapStringToStringArray::c_repr_of(input.1)?.into_raw_pointer(),
        })
    }
}

impl AsRust<(hermes::InjectionKind, HashMap<String, Vec<String>>)> for CInjectionRequestOperation {
    fn as_rust(&self) -> Result<(hermes::InjectionKind, HashMap<String, Vec<String>>)> {
        Ok((self.kind.as_rust()?, unsafe { CMapStringToStringArray::raw_borrow(self.values) }?.as_rust()?))
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CInjectionRequestOperations {
    pub operations: *const *const CInjectionRequestOperation,
    pub count: libc::c_int,
}

impl Drop for CInjectionRequestOperations {
    fn drop(&mut self) {
        let _ = unsafe {
            for e in Box::from_raw(::std::slice::from_raw_parts_mut(
                self.operations as *mut *mut CInjectionRequestOperation,
                self.count as usize,
            )).iter() {
                let _ = CInjectionRequestOperation::drop_raw_pointer(*e).unwrap();
            }
        };
    }
}

impl CReprOf<Vec<(hermes::InjectionKind, HashMap<String, Vec<String>>)>> for CInjectionRequestOperations {
    fn c_repr_of(input: Vec<(hermes::InjectionKind, HashMap<String, Vec<String>>)>) -> Result<Self> {
        Ok(Self {
            count: input.len() as libc::c_int,
            operations: Box::into_raw(
                input
                    .into_iter()
                    .map(|e| CInjectionRequestOperation::c_repr_of(e).map(|c| c.into_raw_pointer()))
                    .collect::<Result<Vec<*const CInjectionRequestOperation>>>()
                    .context("Could not convert map to C Repr")?
                    .into_boxed_slice(),
            ) as *const *const CInjectionRequestOperation,
        })
    }
}

impl AsRust<Vec<(hermes::InjectionKind, HashMap<String, Vec<String>>)>> for CInjectionRequestOperations {
    fn as_rust(&self) -> Result<Vec<(hermes::InjectionKind, HashMap<String, Vec<String>>)>> {
        let mut result = Vec::with_capacity(self.count as usize);

        for e in unsafe { slice::from_raw_parts(self.operations, self.count as usize) } {
            result.push(unsafe { CInjectionRequestOperation::raw_borrow(*e) }?.as_rust()?);
        }

        Ok(result)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CInjectionRequestMessage {
    operations: *const CInjectionRequestOperations,
    lexicon: *const CMapStringToStringArray,
    cross_language: *const libc::c_char,
    id: *const libc::c_char,
}

impl Drop for CInjectionRequestMessage {
    fn drop(&mut self) {
        let _ = unsafe { CInjectionRequestOperations::drop_raw_pointer(self.operations) };
        let _ = unsafe { CMapStringToStringArray::drop_raw_pointer(self.lexicon) };
        take_back_nullable_c_string!(self.cross_language);
        take_back_nullable_c_string!(self.id);
    }
}

impl CReprOf<hermes::InjectionRequest> for CInjectionRequestMessage {
    fn c_repr_of(input: hermes::InjectionRequest) -> Result<Self> {
        Ok(Self {
            operations: CInjectionRequestOperations::c_repr_of(input.operations)?.into_raw_pointer(),
            lexicon: CMapStringToStringArray::c_repr_of(input.lexicon)?.into_raw_pointer(),
            cross_language: convert_to_nullable_c_string!(input.cross_language),
            id: convert_to_nullable_c_string!(input.id),
        })
    }
}

impl AsRust<hermes::InjectionRequest> for CInjectionRequestMessage {
    fn as_rust(&self) -> Result<hermes::InjectionRequest> {
        let operations = unsafe { CInjectionRequestOperations::raw_borrow(self.operations) }?.as_rust()?;
        let lexicon = unsafe { CMapStringToStringArray::raw_borrow(self.lexicon) }?.as_rust()?;
        Ok(hermes::InjectionRequest {
            operations,
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

impl Drop for CInjectionStatusMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.last_injection_date);
    }
}

impl CReprOf<hermes::InjectionStatus> for CInjectionStatusMessage {
    fn c_repr_of(status: hermes::InjectionStatus) -> Result<Self> {
        let last_injection_date_str = status.last_injection_date.map(|d| d.to_rfc3339());

        Ok(Self {
            last_injection_date: convert_to_nullable_c_string!(last_injection_date_str),
        })
    }
}

impl AsRust<hermes::InjectionStatus> for CInjectionStatusMessage {
    fn as_rust(&self) -> Result<hermes::InjectionStatus> {
        let last_injection_date = create_optional_rust_string_from!(self.last_injection_date);
        let last_injection_date = if let Some(date_str) = last_injection_date {
            Some(date_str.parse()?)
        } else {
            None
        };

        Ok(hermes::InjectionStatus { last_injection_date })
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use spectral::prelude::*;
    use super::*;

    fn round_trip_test<T, U>(input: T) where T: Clone + PartialEq + ::std::fmt::Debug, U: CReprOf<T> + AsRust<T> {
        let c = U::c_repr_of(input.clone()).expect("could not convert to c_repr");

        let result = c.as_rust().expect("could not convert back to rust");

        assert_that!(result).is_equal_to(input);
    }

    #[test]
    fn round_trip_intent_not_recognized() {
        round_trip_test::<_, CIntentNotRecognizedMessage>(hermes::IntentNotRecognizedMessage {
            site_id: "siteid".into(),
            custom_data: Some("custom".into()),
            session_id: "session id".into(),
            input: Some("some text".into()),
        });

        round_trip_test::<_, CIntentNotRecognizedMessage>(hermes::IntentNotRecognizedMessage {
            site_id: "siteid".into(),
            custom_data: None,
            session_id: "session id".into(),
            input: None,
        });

    }

    #[test]
    fn round_trip_session_started() {
        round_trip_test::<_, CSessionStartedMessage>(hermes::SessionStartedMessage {
            site_id: "siteid".into(),
            custom_data: Some("custom".into()),
            session_id: "session id".into(),
            reactivated_from_session_id: Some("other session id".into()),
        });

        round_trip_test::<_, CSessionStartedMessage>(hermes::SessionStartedMessage {
            site_id: "siteid".into(),
            custom_data: None,
            session_id: "session id".into(),
            reactivated_from_session_id: None,
        })
    }

    #[test]
    fn round_trip_session_ended() {
        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage {
            site_id: "siteid".into(),
            custom_data: Some("custom".into()),
            session_id: "session id".into(),
            termination: hermes::SessionTerminationType::Nominal,
        });

        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage {
            site_id: "siteid".into(),
            custom_data: None,
            session_id: "session_id".into(),
            termination: hermes::SessionTerminationType::Error { error: "this is my error".into() },
        })
    }

    #[test]
    fn round_trip_session_queued() {
        round_trip_test::<_, CSessionQueuedMessage>(hermes::SessionQueuedMessage {
            site_id: "siteid".into(),
            custom_data: Some("custom".into()),
            session_id: "session id".into(),
        });

        round_trip_test::<_, CSessionQueuedMessage>(hermes::SessionQueuedMessage {
            site_id: "siteid".into(),
            custom_data: None,
            session_id: "session_id".into(),
        })
    }

    #[test]
    fn round_trip_start_session() {
        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage {
            init: hermes::SessionInit::Notification {
                text: "text".into()
            },
            custom_data: Some("thing".into()),
            site_id: Some("site".into()),
        });

        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage {
            init: hermes::SessionInit::Action {
                intent_filter: Some(vec!["filter1".into(), "filter2".into()]),
                text: Some("text".into()),
                can_be_enqueued: true,
                send_intent_not_recognized: false,
            },
            custom_data: Some("thing".into()),
            site_id: Some("site".into()),
        });

        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage {
            init: hermes::SessionInit::Action {
                intent_filter: None,
                text: None,
                can_be_enqueued: false,
                send_intent_not_recognized: true
            },
            custom_data: None,
            site_id: None,
        });
    }

    #[test]
    fn round_trip_continue_session() {
        round_trip_test::<_, CContinueSessionMessage>(hermes::ContinueSessionMessage {
            session_id: "my session id".into(),
            text: "some text".into(),
            intent_filter: Some(vec!["filter1".into(), "filter2".into()]),
            custom_data: Some("foo bar".into()),
            send_intent_not_recognized: true,
        });

        round_trip_test::<_, CContinueSessionMessage>(hermes::ContinueSessionMessage {
            session_id: "my session id".into(),
            text: "some text".into(),
            intent_filter: None,
            custom_data: None,
            send_intent_not_recognized: false,
        });

        round_trip_test::<_, CContinueSessionMessage>(hermes::ContinueSessionMessage {
            session_id: "my session id".into(),
            text: "some text".into(),
            intent_filter: Some(vec![]),
            custom_data: Some("".into()),
            send_intent_not_recognized: true,
        });
    }

    #[test]
    fn round_trip_end_session() {
        round_trip_test::<_, CEndSessionMessage>(hermes::EndSessionMessage {
            session_id: "my session id".into(),
            text: Some("some text".into()),
        });

        round_trip_test::<_, CEndSessionMessage>(hermes::EndSessionMessage {
            session_id: "my session id".into(),
            text: None,
        });
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

    #[test]
    fn round_trip_injection_request_operation() {
        round_trip_test::<_, CInjectionRequestOperation>(
            (hermes::InjectionKind::Add, HashMap::new())
        );

        let mut test_map = HashMap::new();
        test_map.insert("hello".into(), vec!["hello".to_string(), "world".to_string()]);
        test_map.insert("foo".into(), vec!["bar".to_string(), "baz".to_string()]);

        round_trip_test::<_, CInjectionRequestOperation>(
            (hermes::InjectionKind::Add, test_map)
        );
    }

    #[test]
    fn round_trip_injection_request_operations() {
        round_trip_test::<_, CInjectionRequestOperations>(
            vec![]
        );

        let mut test_map = HashMap::new();
        test_map.insert("hello".into(), vec!["hello".to_string(), "world".to_string()]);
        test_map.insert("foo".into(), vec!["bar".to_string(), "baz".to_string()]);

        round_trip_test::<_, CInjectionRequestOperations>(
            vec![
                (hermes::InjectionKind::Add, HashMap::new()),
                (hermes::InjectionKind::Add, test_map)
            ]
        );
    }

    #[test]
    fn round_trip_injection_request() {
        let mut injections = HashMap::new();
        injections.insert("hello".into(), vec!["hello".to_string(), "world".to_string()]);
        injections.insert("foo".into(), vec!["bar".to_string(), "baz".to_string()]);

        let mut lexicon = HashMap::new();
        lexicon.insert("this".into(), vec!["is ".to_string(), "a".to_string(), "lexicon".to_string()]);
        lexicon.insert("baz".into(), vec!["bar".to_string(), "foo".to_string()]);

        round_trip_test::<_, CInjectionRequestMessage>(
            hermes::InjectionRequest {
                cross_language: Some("en".to_string()),
                operations: vec![
                    (hermes::InjectionKind::Add, HashMap::new()),
                    (hermes::InjectionKind::Add, injections)
                ],
                lexicon,
                id: Some("some id".to_string()),
            }
        );
    }

    #[test]
    fn round_injection_status() {
        round_trip_test::<_, CInjectionStatusMessage>(hermes::InjectionStatus {
            last_injection_date: Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
        });
    }

    #[test]
    fn round_trip_token_confidence() {
        round_trip_test::<_, CAsrTokenConfidence>(hermes::AsrTokenConfidence {
            value: "hello world".into(),
            confidence: 0.98,
            range_start: 4,
            range_end: 9,
        });
    }

    #[test]
    fn round_trip_token_confidence_array() {
        round_trip_test::<_, CAsrTokenConfidenceArray>(
            vec![]
        );

        round_trip_test::<_, CAsrTokenConfidenceArray>(
            vec![
                hermes::AsrTokenConfidence {
                    value: "hello".to_string(), confidence: 0.98, range_start: 1, range_end: 4
                },
                hermes::AsrTokenConfidence {
                    value: "world".to_string(), confidence: 0.73, range_start: 5, range_end: 9
                },
            ]
        );
    }
}
