use ::std::ffi::CString;
use ::std::ptr::null;

use errors::*;

use ::libc;
use ::snips_queries_ontology::ffi::{CIntentClassifierResult, CSlot, CSlotList};

#[repr(C)]
#[derive(Debug)]
pub struct CTextCapturedMessage {
    pub text: *const libc::c_char,
    pub likelihood: f32,
    pub seconds: f32,
}

impl CTextCapturedMessage {
    pub fn from(input: ::TextCapturedMessage) -> Result<Self> {
        Ok(Self {
            text: CString::new(input.text)?.into_raw(),
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
    pub likelihood: f32,
    pub seconds: f32,
}

impl CNluQueryMessage {
    pub fn from(input: ::NluQueryMessage) -> Result<Self> {
        Ok(Self {
            text: CString::new(input.text)?.into_raw(),
            likelihood: input.likelihood.unwrap_or(0.0),
            seconds: input.seconds.unwrap_or(0.0),
        })
    }
}

impl Drop for CNluQueryMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.text as *mut libc::c_char) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlotQueryMessage {
    pub text: *const libc::c_char,
    pub likelihood: f32,
    pub seconds: f32,
    pub intent_name: *const libc::c_char,
    pub slot_name: *const libc::c_char,
}

impl CNluSlotQueryMessage {
    pub fn from(input: ::NluSlotQueryMessage) -> Result<Self> {
        Ok(Self {
            text: CString::new(input.text)?.into_raw(),
            likelihood: input.likelihood,
            seconds: input.seconds,
            intent_name: CString::new(input.intent_name)?.into_raw(),
            slot_name: CString::new(input.slot_name)?.into_raw(),
        })
    }
}

impl Drop for CNluSlotQueryMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.text as *mut libc::c_char) };
        let _ = unsafe { CString::from_raw(self.intent_name as *mut libc::c_char) };
        let _ = unsafe { CString::from_raw(self.slot_name as *mut libc::c_char) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CPlayFileMessage {
    pub file_path: *const libc::c_char,
}

impl CPlayFileMessage {
    pub fn from(input: ::PlayFileMessage) -> Result<Self> {
        Ok(Self {
            file_path: CString::new(input.file_path)?.into_raw(),
        })
    }
}

impl Drop for CPlayFileMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.file_path as *mut libc::c_char) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CPlayBytesMessage {
    pub id: *const libc::c_char,
    pub wav_bytes: *const u8,
    pub wav_bytes_len: libc::size_t,
}

impl CPlayBytesMessage {
    pub fn from(input: ::PlayBytesMessage) -> Result<Self> {
        Ok(Self {
            id: CString::new(input.id)?.into_raw(),
            wav_bytes_len: input.wav_bytes.len(),
            wav_bytes: Box::into_raw(input.wav_bytes.into_boxed_slice()) as *const u8,
        })
    }
}

impl Drop for CPlayBytesMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.id as *mut libc::c_char) };
        let _ = unsafe { Box::from_raw(self.wav_bytes as *mut u8) };
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
            id: CString::new(input.id)?.into_raw(),
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
            text: CString::new(input.text)?.into_raw(),
            lang: if let Some(s) = input.lang { CString::new(s)?.into_raw() } else { null() },
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
    pub slot: Option<Box<CSlot>>,
}

impl CSlotMessage {
    pub fn from(input: ::SlotMessage) -> Result<Self> {
        Ok(Self {
            slot: if let Some(s) = input.slot { Some(Box::new(CSlot::from(s)?)) } else { None },
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CIntentNotRecognizedMessage {
    pub text: *const libc::c_char,
}

impl CIntentNotRecognizedMessage {
    pub fn from(input: ::IntentNotRecognizedMessage) -> Result<Self> {
        Ok(Self {
            text: CString::new(input.text)?.into_raw(),
        })
    }
}

impl Drop for CIntentNotRecognizedMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.text as *mut libc::c_char) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CIntentMessage {
    pub input: *const libc::c_char,
    pub intent: Option<Box<CIntentClassifierResult>>,
    pub slots: Option<Box<CSlotList>>,
}

impl CIntentMessage {
    pub fn from(input: ::IntentMessage) -> Result<Self> {
        Ok(Self {
            input: CString::new(input.input)?.into_raw(),
            intent: Some(Box::new(CIntentClassifierResult::from(input.intent)?)),
            slots: if let Some(slots) = input.slots {
                Some(Box::new(CSlotList::from(slots)?))
            } else { None },
        })
    }
}

impl Drop for CIntentMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.input as *mut libc::c_char) };
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
            error: CString::new(input.error)?.into_raw(),
            context: if let Some(s) = input.context { CString::new(s)?.into_raw() } else { null() },
        })
    }
}

impl Drop for CErrorMessage {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.error as *mut libc::c_char) };
        let _ = unsafe { CString::from_raw(self.context as *mut libc::c_char) };
    }
}

