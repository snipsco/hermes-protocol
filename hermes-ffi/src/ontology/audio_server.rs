 use failure::Fallible;
use std::slice;
use ffi_utils::{AsRust, CReprOf, RawPointerConverter};
use failure::ResultExt;

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
    pub fn from(input: hermes::PlayBytesMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::PlayBytesMessage> for CPlayBytesMessage {
    fn c_repr_of(input: hermes::PlayBytesMessage) -> Fallible<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
            wav_bytes_len: input.wav_bytes.len() as libc::c_int,
            wav_bytes: Box::into_raw(input.wav_bytes.into_boxed_slice()) as *const u8,
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::PlayBytesMessage> for CPlayBytesMessage {
    fn as_rust(&self) -> Fallible<hermes::PlayBytesMessage> {
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
    pub fn from(input: hermes::AudioFrameMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::AudioFrameMessage> for CAudioFrameMessage {
    fn c_repr_of(input: hermes::AudioFrameMessage) -> Fallible<Self> {
        Ok(Self {
            wav_frame_len: input.wav_frame.len() as libc::c_int,
            wav_frame: Box::into_raw(input.wav_frame.into_boxed_slice()) as *const u8,
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::AudioFrameMessage> for CAudioFrameMessage {
    fn as_rust(&self) -> Fallible<hermes::AudioFrameMessage> {
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
    pub fn from(input: hermes::PlayFinishedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::PlayFinishedMessage> for CPlayFinishedMessage {
    fn c_repr_of(input: hermes::PlayFinishedMessage) -> Fallible<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::PlayFinishedMessage> for CPlayFinishedMessage {
    fn as_rust(&self) -> Fallible<hermes::PlayFinishedMessage> {
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
