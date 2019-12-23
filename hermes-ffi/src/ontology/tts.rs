use std::slice;

use failure::Fallible;
use failure::ResultExt;
use ffi_utils::*;
use ffi_utils_derive::{AsRust, CReprOf};

use hermes::*;

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(SayMessage)]
pub struct CSayMessage {
    pub text: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub lang: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub id: *const libc::c_char,
    pub site_id: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub session_id: *const libc::c_char,
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
#[derive(Debug, CReprOf, AsRust)]
#[target_type(SayFinishedMessage)]
pub struct CSayFinishedMessage {
    /// Nullable
    #[nullable]
    pub id: *const libc::c_char,
    /// Nullable
    #[nullable]
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

impl Drop for CSayFinishedMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CRegisterSoundMessage {
    pub sound_id: *const libc::c_char,
    pub wav_sound: *const u8,
    // Note: we can't use `libc::size_t` because it's not supported by JNA
    pub wav_sound_len: libc::c_int,
}

unsafe impl Sync for CRegisterSoundMessage {}

impl CReprOf<hermes::RegisterSoundMessage> for CRegisterSoundMessage {
    fn c_repr_of(input: hermes::RegisterSoundMessage) -> Fallible<Self> {
        Ok(Self {
            wav_sound_len: input.wav_sound.len() as libc::c_int,
            wav_sound: Box::into_raw(input.wav_sound.into_boxed_slice()) as *const u8,
            sound_id: convert_to_c_string!(input.sound_id),
        })
    }
}

impl AsRust<hermes::RegisterSoundMessage> for CRegisterSoundMessage {
    fn as_rust(&self) -> Fallible<hermes::RegisterSoundMessage> {
        Ok(hermes::RegisterSoundMessage {
            wav_sound: unsafe { slice::from_raw_parts(self.wav_sound as *const u8, self.wav_sound_len as usize) }
                .to_vec(),
            sound_id: create_rust_string_from!(self.sound_id),
        })
    }
}

impl Drop for CRegisterSoundMessage {
    fn drop(&mut self) {
        let _ = unsafe {
            Box::from_raw(slice::from_raw_parts_mut(
                self.wav_sound as *mut u8,
                self.wav_sound_len as usize,
            ))
        };
        take_back_c_string!(self.sound_id);
    }
}

#[cfg(test)]
mod tests {
    use hermes::hermes_utils::Example;

    use super::*;
    use super::super::tests::round_trip_test;

    #[test]
    fn round_trip_register_sound() {
        round_trip_test::<_, CRegisterSoundMessage>(hermes::RegisterSoundMessage::minimal_example());
        round_trip_test::<_, CRegisterSoundMessage>(hermes::RegisterSoundMessage::full_example());
    }
}
