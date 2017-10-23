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
        convert_to_c_string_result!($string)?
    };
}

macro_rules! convert_to_c_string_result {
    ($string:expr) => {
        CString::new($string).chain_err(||"Could not convert String to C Repr").map(|s| s.into_raw())
    };
}

macro_rules! convert_to_c_array_string {
    ($string_vec:expr) => {
        Box::into_raw(Box::new(CArrayString::from($string_vec).chain_err(|| "Could not convert Vector of Strings to C Repr")?)) as *const CArrayString
    }
}
macro_rules! convert_to_nullable_c_array_string {
    ($opt:expr) => {
        if let Some(s) = $opt {
            convert_to_c_array_string!(s)
        } else  {
            null()
        }
    }
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

macro_rules! take_back_c_array_string {
    ($pointer:expr) => {{ let _ = unsafe { Box::from_raw($pointer as *mut CArrayString) };}}
}

macro_rules! take_back_nullable_c_array_string {
    ($pointer:expr) => {
        if !$pointer.is_null() {
           take_back_c_array_string!($pointer)
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CArrayString {
    pub data: *const *const libc::c_char,
    pub size: libc::c_int, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl CArrayString {
    pub fn from(input: Vec<String>) -> Result<Self> {
        Ok(Self {
            size: input.len() as libc::c_int,
            data: Box::into_raw(input.into_iter()
                .map(|s| convert_to_c_string_result!(s))
                .collect::<Result<Vec<_>>>()?
                .into_boxed_slice()) as *const *const libc::c_char,
        })
    }
}

impl Drop for CArrayString {
    fn drop(&mut self) {
        let _ = unsafe {
            let y = Box::from_raw(slice::from_raw_parts_mut(self.data as *mut *mut libc::c_char, self.size as usize));
            for p in y.into_iter() {
                CString::from_raw(*p);
            }
        };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSiteMessage {
    pub site_id: *const libc::c_char,
    pub session_id: *const libc::c_char, // Nullable
}

impl CSiteMessage {
    pub fn from(input: hermes::TextCapturedMessage) -> Result<Self> {
        Ok(Self {
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
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
pub struct CTextCapturedMessage {
    pub text: *const libc::c_char,
    pub likelihood: f32,
    pub seconds: f32,
    pub site_id: *const libc::c_char,
    pub session_id: *const libc::c_char, // Nullable
}

impl CTextCapturedMessage {
    pub fn from(input: hermes::TextCapturedMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            likelihood: input.likelihood,
            seconds: input.seconds,
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl Drop for CTextCapturedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.text);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluQueryMessage {
    pub input: *const libc::c_char,
    pub intent_filter: *const CArrayString, // Nullable 
    pub id: *const libc::c_char, // Nullable
    pub session_id: *const libc::c_char, // Nullable
}

impl CNluQueryMessage {
    pub fn from(input: hermes::NluQueryMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            intent_filter: convert_to_nullable_c_array_string!(input.intent_filter),
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl Drop for CNluQueryMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_array_string!(self.intent_filter);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlotQueryMessage {
    pub input: *const libc::c_char,
    pub intent_name: *const libc::c_char,
    pub slot_name: *const libc::c_char,
    pub id: *const libc::c_char, // Nullable
    pub session_id: *const libc::c_char, // Nullable
}

impl CNluSlotQueryMessage {
    pub fn from(input: hermes::NluSlotQueryMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            intent_name: convert_to_c_string!(input.intent_name),
            slot_name: convert_to_c_string!(input.slot_name),
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
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
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CPlayBytesMessage {
    pub id: *const libc::c_char,
    pub wav_bytes: *const u8,
    pub wav_bytes_len: libc::c_int, // Note: we can't use `libc_size_t` because JNA doesn't it
    pub site_id: *const libc::c_char,
    pub session_id: *const libc::c_char, // Nullable
}

impl CPlayBytesMessage {
    pub fn from(input: hermes::PlayBytesMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
            wav_bytes_len: input.wav_bytes.len() as libc::c_int,
            wav_bytes: Box::into_raw(input.wav_bytes.into_boxed_slice()) as *const u8,
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl Drop for CPlayBytesMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.id);
        let _ = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.wav_bytes as *mut u8, self.wav_bytes_len as usize)) };
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CAudioFrameMessage {
    pub wav_frame: *const u8,
    pub wav_frame_len: libc::c_int, // Note: we can't use `libc_size_t` because JNA doesn't it
    pub site_id: *const libc::c_char,
}

impl CAudioFrameMessage {
    pub fn from(input: hermes::AudioFrameMessage) -> Result<Self> {
        Ok(Self {
            wav_frame_len: input.wav_frame.len() as libc::c_int,
            wav_frame: Box::into_raw(input.wav_frame.into_boxed_slice()) as *const u8,
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl Drop for CAudioFrameMessage {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.wav_frame as *mut u8, self.wav_frame_len as usize)) };
        take_back_c_string!(self.site_id);
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct CPlayFinishedMessage {
    pub id: *const libc::c_char,
    pub site_id: *const libc::c_char,
    pub session_id: *const libc::c_char, // Nullable
}

impl CPlayFinishedMessage {
    pub fn from(input: hermes::PlayFinishedMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl Drop for CPlayFinishedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.id);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSayMessage {
    pub text: *const libc::c_char,
    pub lang: *const libc::c_char, // Nullable
    pub id: *const libc::c_char, // Nullable
    pub site_id: *const libc::c_char,
    pub session_id: *const libc::c_char, // Nullable
}

impl CSayMessage {
    pub fn from(input: hermes::SayMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            lang: convert_to_nullable_c_string!(input.lang),
            id: convert_to_nullable_c_string!(input.id),
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

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
    pub id: *const libc::c_char, // Nullable
    pub session_id: *const libc::c_char, // Nullable
}

impl CSayFinishedMessage {
    pub fn from(input: hermes::SayMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
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
    pub id: *const libc::c_char, // Nullable
    pub input: *const libc::c_char,
    pub intent_name: *const libc::c_char,
    pub slot: *const CSlot, // Nullable
    pub session_id: *const libc::c_char, // Nullable
}

impl CNluSlotMessage {
    pub fn from(input: hermes::NluSlotMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            input: convert_to_c_string!(input.input),
            intent_name: convert_to_c_string!(input.intent_name),
            slot: if let Some(s) = input.slot {
                Box::into_raw(Box::new(CSlot::from(s).chain_err(|| "Could not transform Slot into C Repr")?)) as *const CSlot
            } else {
                null()
            },
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl Drop for CNluSlotMessage {
    fn drop(&mut self) {
        if !self.slot.is_null() {
            take_back_nullable_c_string!(self.id);
            take_back_c_string!(self.input);
            take_back_c_string!(self.intent_name);
            if !self.slot.is_null() {
                let _ = unsafe { Box::from_raw(self.slot as *mut CSlot) };
            }
            take_back_nullable_c_string!(self.session_id);
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluIntentNotRecognizedMessage {
    pub input: *const libc::c_char,
    pub id: *const libc::c_char, // Nullable
    pub session_id: *const libc::c_char, // Nullable
}

impl CNluIntentNotRecognizedMessage {
    pub fn from(input: hermes::NluIntentNotRecognizedMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
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
pub struct CNluIntentMessage {
    pub id: *const libc::c_char, // Nullable
    pub input: *const libc::c_char,
    pub intent: *const CIntentClassifierResult,
    pub slots: *const CSlotList, // Nullable
    pub session_id: *const libc::c_char, //Nullable
}

impl CNluIntentMessage {
    pub fn from(input: hermes::NluIntentMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            input: convert_to_c_string!(input.input),
            intent: Box::into_raw(Box::new(CIntentClassifierResult::from(input.intent).chain_err(|| "Could not transform IntentClassifierResult into C Repr")?)),
            slots: if let Some(slots) = input.slots {
                Box::into_raw(Box::new(CSlotList::from(slots).chain_err(|| "Could not transform Slot list into C Repr")?)) as *const CSlotList
            } else {
                null()
            },
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl Drop for CNluIntentMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.input);
        let _ = unsafe { Box::from_raw(self.intent as *mut CIntentClassifierResult) };
        if !self.slots.is_null() {
            let _ = unsafe { Box::from_raw(self.slots as *mut CSlotList) };
        }
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CIntentMessage {
    pub session_id: *const libc::c_char,
    pub custom_data: *const libc::c_char, // Nullable
    pub site_id: *const libc::c_char,

    pub input: *const libc::c_char,
    pub intent: *const CIntentClassifierResult,
    pub slots: *const CSlotList, // Nullable
}

impl CIntentMessage {
    pub fn from(input: hermes::IntentMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
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
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
        take_back_c_string!(self.input);
        let _ = unsafe { Box::from_raw(self.intent as *mut CIntentClassifierResult) };
        if !self.slots.is_null() {
            let _ = unsafe { Box::from_raw(self.slots as *mut CSlotList) };
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum CSessionInitType {
    Action = 1,
    Notification = 2,
}

impl CSessionInitType {
    pub fn from(slot_value: &hermes::SessionInit) -> Self {
        match slot_value {
            &hermes::SessionInit::Notification { .. } => CSessionInitType::Notification,
            &hermes::SessionInit::Action { .. } => CSessionInitType::Action,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct CActionSessionInit {
    text: *const libc::c_char, // Nullable
    intent_filter: *const CArrayString, // Nullable
    can_be_enqueued: libc::c_int,
}

impl CActionSessionInit {
    pub fn new(text: Option<String>, intent_filter: Option<Vec<String>>, can_be_enqueued: bool) -> Result<Self> {
        Ok(Self {
            text: convert_to_nullable_c_string!(text),
            intent_filter: convert_to_nullable_c_array_string!(intent_filter),
            can_be_enqueued: if can_be_enqueued { 1 } else { 0 }
        })
    }
}

impl Drop for CActionSessionInit {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.text);
        take_back_nullable_c_array_string!(self.intent_filter);
    }
}



#[repr(C)]
#[derive(Debug)]
pub struct CSessionInit {
    init_type: CSessionInitType,
    /**
     * Points to either a *const char, a *const CActionSessionInit
     */
    value: *const libc::c_void,
}

impl CSessionInit {
    fn from(init: hermes::SessionInit) -> Result<Self> {
        let init_type = CSessionInitType::from(&init);
        let value: *const libc::c_void = match init {
            hermes::SessionInit::Action { text, intent_filter, can_be_enqueued } => {
                Box::into_raw(Box::new(CActionSessionInit::new(text, intent_filter, can_be_enqueued)?)) as _
            },
            hermes::SessionInit::Notification { text } => {
                convert_to_c_string!(text) as _
            }
        };
        Ok(Self { init_type, value })
    }
}

impl Drop for CSessionInit {
    fn drop(&mut self) {
        match self.init_type {
            CSessionInitType::Action => {
                take_back_c_string!(self.value);
            },
            CSessionInitType::Notification => unsafe {
                Box::from_raw(self.value as *mut CActionSessionInit);
            },
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

impl CStartSessionMessage {
    pub fn from(input: hermes::StartSessionMessage) -> Result<Self> {
        Ok(Self {
            init: CSessionInit::from(input.init)?,
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_nullable_c_string!(input.site_id),
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
    pub custom_data: *const libc::c_char, // Nullable
    pub site_id: *const libc::c_char,
    pub reactivated_from_session_id: *const libc::c_char, // Nullable
}

impl CSessionStartedMessage {
    pub fn from(input: hermes::SessionStartedMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
            reactivated_from_session_id: convert_to_nullable_c_string!(input.reactivated_from_session_id),
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
    pub custom_data: *const libc::c_char, // Nullable
    pub site_id: *const libc::c_char,
}

impl CSessionQueuedMessage {
    pub fn from(input: hermes::SessionQueuedMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
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
    pub intent_filter: *const CArrayString, // Nullable 
}

impl CContinueSessionMessage {
    pub fn from(input: hermes::ContinueSessionMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            text: convert_to_c_string!(input.text),
            intent_filter: convert_to_nullable_c_array_string!(input.intent_filter),
        })
    }
}

impl Drop for CContinueSessionMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_c_string!(self.text);
        take_back_nullable_c_array_string!(self.intent_filter);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CEndSessionMessage {
    pub session_id: *const libc::c_char,
    pub text: *const libc::c_char, // Nullable
}

impl CEndSessionMessage {
    pub fn from(input: hermes::EndSessionMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            text: convert_to_nullable_c_string!(input.text),
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
pub enum CSessionTerminationType {
    Nominal = 1,
    SiteUnavailable = 2,
    AbortedByUser = 3,
    IntentNotRecognized = 4,
    Timeout = 5,
    Error = 6,
}

impl CSessionTerminationType {
    fn from(termination_type: &hermes::SessionTerminationType) -> CSessionTerminationType {
        match termination_type {
            &hermes::SessionTerminationType::Nominal => CSessionTerminationType::Nominal,
            &hermes::SessionTerminationType::SiteUnavailable => CSessionTerminationType::SiteUnavailable,
            &hermes::SessionTerminationType::AbortedByUser => CSessionTerminationType::AbortedByUser,
            &hermes::SessionTerminationType::IntentNotRecognized => CSessionTerminationType::IntentNotRecognized,
            &hermes::SessionTerminationType::Timeout => CSessionTerminationType::Timeout,
            &hermes::SessionTerminationType::Error { .. } => CSessionTerminationType::Error,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionTermination {
    termination_type: CSessionTerminationType,
    data: *const libc::c_char, // Nullable,
}

impl CSessionTermination {
    fn from(termination: ::hermes::SessionTerminationType) -> Result<Self> {
        let termination_type = CSessionTerminationType::from(&termination);
        let data: *const libc::c_char = match termination {
            ::hermes::SessionTerminationType::Error { error } => convert_to_c_string!(error),
            _ => null(),
        };
        Ok(Self { termination_type, data })
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
    pub custom_data: *const libc::c_char, // Nullable
    pub termination: CSessionTermination,
    pub site_id: *const libc::c_char,
}

impl CSessionEndedMessage {
    pub fn from(input: hermes::SessionEndedMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            termination: CSessionTermination::from(input.termination)?,
            site_id: convert_to_c_string!(input.site_id),
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
    pub session_id: *const libc::c_char, // Nullable
    pub error: *const libc::c_char,
    pub context: *const libc::c_char, // Nullable
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
