extern crate hermes;
extern crate libc;
extern crate snips_queries_ontology;

use std::ffi::CString;
use std::slice;
use std::ptr::null;

use hermes::{Result, ResultExt};
use snips_queries_ontology::ffi::{CIntentClassifierResult, CSlot, CSlotList};

macro_rules! convert_to_c_string {
    ($string:expr) => {
        CString::new($string).chain_err(||"Could not convert String to C Repr")?.into_raw()
    };
}

macro_rules! convert_to_nullable_c_string {
    ($opt:expr) => {
        if let Some(s) = $opt {
            convert_to_c_string!(s)
        } else  {
            null()
        }
    };
}

macro_rules! take_back_c_string {
    ($pointer:expr) => {{ let _ = unsafe { CString::from_raw($pointer as *mut libc::c_char) }; }};
}

macro_rules! take_back_nullable_c_string {
    ($pointer:expr) => {
        if !$pointer.is_null() {
            take_back_c_string!($pointer)
        }
    };
}

#[repr(C)]
#[derive(Debug)]
pub struct CTextCapturedMessage {
    pub text: *const libc::c_char,
    pub likelihood: f32,
    pub seconds: f32,
}

impl CTextCapturedMessage {
    pub fn from(input: hermes::TextCapturedMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            likelihood: input.likelihood,
            seconds: input.seconds,
        })
    }
}

impl Drop for CTextCapturedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.text);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluQueryMessage {
    pub input: *const libc::c_char,
    pub id: *const libc::c_char,
}

impl CNluQueryMessage {
    pub fn from(input: hermes::NluQueryMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            id: convert_to_nullable_c_string!(input.id),
        })
    }
}

impl Drop for CNluQueryMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_string!(self.id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlotQueryMessage {
    pub input: *const libc::c_char,
    pub id: *const libc::c_char,
    pub intent_name: *const libc::c_char,
    pub slot_name: *const libc::c_char,
}

impl CNluSlotQueryMessage {
    pub fn from(input: hermes::NluSlotQueryMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            id: convert_to_nullable_c_string!(input.id),
            intent_name: convert_to_c_string!(input.intent_name),
            slot_name: convert_to_c_string!(input.slot_name),
        })
    }
}

impl Drop for CNluSlotQueryMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.intent_name);
        take_back_c_string!(self.slot_name);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CPlayBytesMessage {
    pub id: *const libc::c_char,
    pub wav_bytes: *const u8,
    pub wav_bytes_len: libc::c_int, // Note: we can't use `libc_size_t` because JNA doesn't it
}

impl CPlayBytesMessage {
    pub fn from(input: hermes::PlayBytesMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
            wav_bytes_len: input.wav_bytes.len() as libc::c_int,
            wav_bytes: Box::into_raw(input.wav_bytes.into_boxed_slice()) as *const u8,
        })
    }
}

impl Drop for CPlayBytesMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.id);
        let _ = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.wav_bytes as *mut u8, self.wav_bytes_len as usize)) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CPlayFinishedMessage {
    pub id: *const libc::c_char,
}

impl CPlayFinishedMessage {
    pub fn from(input: hermes::PlayFinishedMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
        })
    }
}

impl Drop for CPlayFinishedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSayMessage {
    pub text: *const libc::c_char,
    pub lang: *const libc::c_char,
}

impl CSayMessage {
    pub fn from(input: hermes::SayMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            lang: convert_to_nullable_c_string!(input.lang),
        })
    }
}

impl Drop for CSayMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.text);
        take_back_nullable_c_string!(self.lang);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSlotMessage {
    pub slot: *const CSlot,
}

impl CSlotMessage {
    pub fn from(input: hermes::NluSlotMessage) -> Result<Self> {
        Ok(Self {
            slot: if let Some(s) = input.slot {
                Box::into_raw(Box::new(CSlot::from(s).chain_err(|| "Could not transform Slot into C Repr")?)) as *const CSlot
            } else {
                null()
            },
        })
    }
}

impl Drop for CSlotMessage {
    fn drop(&mut self) {
        if !self.slot.is_null() {
            let _ = unsafe { Box::from_raw(self.slot as *mut CSlot) };
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CIntentNotRecognizedMessage {
    pub input: *const libc::c_char,
    pub id : *const libc::c_char,
}

impl CIntentNotRecognizedMessage {
    pub fn from(input: hermes::NluIntentNotRecognizedMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            id: convert_to_nullable_c_string!(input.id),
        })
    }
}

impl Drop for CIntentNotRecognizedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_string!(self.id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CIntentMessage {
    pub input: *const libc::c_char,
    pub intent: *const CIntentClassifierResult,
    pub slots: *const CSlotList,
}

impl CIntentMessage {
    pub fn from(input: hermes::IntentMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            intent: Box::into_raw(Box::new(CIntentClassifierResult::from(input.intent).chain_err(|| "Could not transform IntentClassifierResult into C Repr")?)),
            slots: if let Some(slots) = input.slots {
                Box::into_raw(Box::new(CSlotList::from(slots).chain_err(|| "Could not transform Slot list into C Repr")?)) as *const CSlotList
            } else {
                null()
            },
        })
    }
}

impl Drop for CIntentMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        let _ = unsafe { Box::from_raw(self.intent as *mut CIntentClassifierResult) };
        if !self.slots.is_null() {
            let _ = unsafe { Box::from_raw(self.slots as *mut CSlotList) };
        }
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
    pub fn from(input: hermes::VersionMessage) -> Result<Self> {
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
    pub error: *const libc::c_char,
    pub context: *const libc::c_char,
}

impl CErrorMessage {
    pub fn from(input: hermes::ErrorMessage) -> Result<Self> {
        Ok(Self {
            error: convert_to_c_string!(input.error),
            context: convert_to_nullable_c_string!(input.context),
        })
    }
}

impl Drop for CErrorMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.error);
        take_back_nullable_c_string!(self.context);
    }
}

