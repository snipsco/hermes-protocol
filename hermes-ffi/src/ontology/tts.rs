use failure::Fallible;
use failure::ResultExt;
use ffi_utils::{AsRust, CReprOf, RawPointerConverter};
use std::ptr::null;

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
    fn c_repr_of(input: hermes::SayMessage) -> Fallible<Self> {
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
    fn as_rust(&self) -> Fallible<hermes::SayMessage> {
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
    pub fn from(input: hermes::SayMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_say_message(&self) -> Fallible<hermes::SayMessage> {
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
    pub fn from(input: hermes::SayFinishedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_say_finished_message(&self) -> Fallible<hermes::SayFinishedMessage> {
        self.as_rust()
    }
}

impl CReprOf<hermes::SayFinishedMessage> for CSayFinishedMessage {
    fn c_repr_of(input: hermes::SayFinishedMessage) -> Fallible<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::SayFinishedMessage> for CSayFinishedMessage {
    fn as_rust(&self) -> Fallible<hermes::SayFinishedMessage> {
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
