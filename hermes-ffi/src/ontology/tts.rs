use std::ptr::null;
use std::slice;

use failure::Fallible;
use failure::ResultExt;
use ffi_utils::*;

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
    use super::super::tests::round_trip_test;
    use super::*;
    use hermes::hermes_utils::Example;

    #[test]
    fn round_trip_register_sound() {
        round_trip_test::<_, CRegisterSoundMessage>(hermes::RegisterSoundMessage::minimal_example());
        round_trip_test::<_, CRegisterSoundMessage>(hermes::RegisterSoundMessage::full_example());
    }
}
