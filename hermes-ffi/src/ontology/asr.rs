use failure::Fallible;
use ffi_utils::*;
use ffi_utils_derive::{AsRust, CReprOf};

use hermes::*;

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(AsrStartListeningMessage)]
pub struct CAsrStartListeningMessage {
    pub site_id: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub session_id: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub start_signal_ms: *const i64, // -1 mean None
}

unsafe impl Sync for CAsrStartListeningMessage {}

impl CAsrStartListeningMessage {
    pub fn from(input: hermes::AsrStartListeningMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl Drop for CAsrStartListeningMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
        if !self.start_signal_ms.is_null() {
            let _ = unsafe { Box::from_raw(self.start_signal_ms as *mut i64) };
        }
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(TextCapturedMessage)]
pub struct CTextCapturedMessage {
    pub text: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub tokens: *const CArray<CAsrToken>,
    pub likelihood: f32,
    pub seconds: f32,
    pub site_id: *const libc::c_char,
    /// Nullable
    #[nullable]
    pub session_id: *const libc::c_char,
//    /// Nullable
//    #[nullable]
//    pub speaker_hypotheses: *const CArray<CSpeakerId>,
}

unsafe impl Sync for CTextCapturedMessage {}

impl CTextCapturedMessage {
    pub fn from(input: hermes::TextCapturedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl Drop for CTextCapturedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.text);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
        if !self.tokens.is_null() {
            let _ = unsafe { CArray::<CAsrToken>::drop_raw_pointer(self.tokens) };
        }
        /*
        if !self.speaker_hypotheses.is_null() {
            let _ = unsafe { CSpeakerIdArray::drop_raw_pointer(self.speaker_hypotheses) };
        }
        */
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(AsrDecodingDuration)]
pub struct CAsrDecodingDuration {
    pub start: f32,
    pub end: f32,
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(AsrToken)]
pub struct CAsrToken {
    pub value: *const libc::c_char,
    pub confidence: f32,
    pub range_start: i32,
    pub range_end: i32,
    pub time: CAsrDecodingDuration,
}

impl Drop for CAsrToken {
    fn drop(&mut self) {
        take_back_c_string!(self.value);
    }
}

pub type CAsrTokenArray = CArray<CAsrToken>;
pub type CAsrTokenDoubleArray = CArray<CAsrTokenArray>;

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(SpeakerId)]
pub struct CSpeakerId {
    /// Nullable
    #[nullable]
    pub name: *const libc::c_char,
    pub confidence: f32,
}

impl Drop for CSpeakerId {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.name)
    }
}

#[cfg(test)]
mod tests {
    use hermes::hermes_utils::Example;

    use super::*;
    use super::super::tests::round_trip_test;

    #[test]
    fn round_trip_asr_token() {
        round_trip_test::<_, CAsrToken>(hermes::AsrToken::minimal_example());
        round_trip_test::<_, CAsrToken>(hermes::AsrToken::full_example());
    }

    #[test]
    fn round_trip_asr_token_array() {
        round_trip_test::<_, CAsrTokenArray>(vec![]);

        round_trip_test::<_, CAsrTokenArray>(vec![
            hermes::AsrToken::minimal_example(),
            hermes::AsrToken::minimal_example(),
        ]);
    }

    #[test]
    fn round_trip_asr_token_double_array() {
        round_trip_test::<_, CAsrTokenDoubleArray>(vec![]);

        round_trip_test::<_, CAsrTokenDoubleArray>(vec![
            vec![hermes::AsrToken::minimal_example(), hermes::AsrToken::full_example()],
            vec![],
            vec![hermes::AsrToken::full_example()],
        ]);
    }
}
