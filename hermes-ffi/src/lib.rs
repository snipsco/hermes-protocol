extern crate hermes;
extern crate libc;
extern crate snips_queries_ontology;

use std::ffi::CString;
use std::slice;
use std::ptr::null;

use hermes::*;

use snips_queries_ontology::ffi::{CIntentClassifierResult, CSlot, CSlotList};

#[repr(C)]
#[derive(Debug)]
pub struct CTextCapturedMessage {
    pub text: *const libc::c_char,
    pub likelihood: f32,
    pub seconds: f32,
}

macro_rules! convert_to_c_string {
    ($string:expr) => {CString::new($string).chain_err(||"Could not convert String to C Repr")?.into_raw()};
}

impl CTextCapturedMessage {
    pub fn from(input: ::TextCapturedMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            likelihood: input.likelihood,
            seconds: input.seconds,
        })
    }
}

impl Drop for CTextCapturedMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.text as *mut libc::c_char) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluQueryMessage {
    pub text: *const libc::c_char,
    pub id: *const libc::c_char,
}

impl CNluQueryMessage {
    pub fn from(input: ::NluQueryMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            id: if let Some(id) = input.id { convert_to_c_string!(id)} else { null() },
        })
    }
}

impl Drop for CNluQueryMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.text as *mut libc::c_char) };
        if !self.id.is_null() {
            let _ = unsafe { CString::from_raw(self.id as *mut libc::c_char) };
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlotQueryMessage {
    pub text: *const libc::c_char,
    pub id: *const libc::c_char,
    pub intent_name: *const libc::c_char,
    pub slot_name: *const libc::c_char,
}

impl CNluSlotQueryMessage {
    pub fn from(input: ::NluSlotQueryMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            id: if let Some(id) = input.id { convert_to_c_string!(id)} else { null() },
            intent_name: convert_to_c_string!(input.intent_name),
            slot_name: convert_to_c_string!(input.slot_name),
        })
    }
}

impl Drop for CNluSlotQueryMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.text as *mut libc::c_char) };
        let _ = unsafe { CString::from_raw(self.intent_name as *mut libc::c_char) };
        let _ = unsafe { CString::from_raw(self.slot_name as *mut libc::c_char) };
        if !self.id.is_null() {
            let _ = unsafe { CString::from_raw(self.id as *mut libc::c_char) };
        }
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
    pub fn from(input: ::PlayBytesMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
            wav_bytes_len: input.wav_bytes.len() as libc::c_int,
            wav_bytes: Box::into_raw(input.wav_bytes.into_boxed_slice()) as *const u8,
        })
    }
}

impl Drop for CPlayBytesMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.id as *mut libc::c_char) };
        let _ = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.wav_bytes as *mut u8, self.wav_bytes_len as usize)) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CPlayFinishedMessage {
    pub id: *const libc::c_char,
}

impl CPlayFinishedMessage {
    pub fn from(input: ::PlayFinishedMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
        })
    }
}

impl Drop for CPlayFinishedMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.id as *mut libc::c_char) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSayMessage {
    pub text: *const libc::c_char,
    pub lang: *const libc::c_char,
}

impl CSayMessage {
    pub fn from(input: ::SayMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            lang: if let Some(s) = input.lang {
                convert_to_c_string!(s)
            } else {
                null()
            },
        })
    }
}

impl Drop for CSayMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.text as *mut libc::c_char) };
        let _ = unsafe { CString::from_raw(self.lang as *mut libc::c_char) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSlotMessage {
    pub slot: *const CSlot,
}

impl CSlotMessage {
    pub fn from(input: ::NluSlotMessage) -> Result<Self> {
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
    pub fn from(input: ::NluIntentNotRecognizedMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            id: if let Some(id) = input.id { convert_to_c_string!(id)} else { null() },
        })
    }
}

impl Drop for CIntentNotRecognizedMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.input as *mut libc::c_char) };
        if !self.id.is_null() {
            let _ = unsafe { CString::from_raw(self.id as *mut libc::c_char) };
        }
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
    pub fn from(input: ::IntentMessage) -> Result<Self> {
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
        let _ = unsafe { CString::from_raw(self.input as *mut libc::c_char) };
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
    pub fn from(input: ::VersionMessage) -> Result<Self> {
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
    pub fn from(input: ::ErrorMessage) -> Result<Self> {
        Ok(Self {
            error: convert_to_c_string!(input.error),
            context: if let Some(s) = input.context {
                convert_to_c_string!(s)
            } else {
                null()
            },
        })
    }
}

impl Drop for CErrorMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.error as *mut libc::c_char) };
        let _ = unsafe { CString::from_raw(self.context as *mut libc::c_char) };
    }
}

