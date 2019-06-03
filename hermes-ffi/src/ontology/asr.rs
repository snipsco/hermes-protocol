use std::ptr::null;
use std::slice;

use failure::Fallible;
use failure::ResultExt;
use ffi_utils::*;

#[repr(C)]
#[derive(Debug)]
pub struct CAsrStartListeningMessage {
    pub site_id: *const libc::c_char,
    /// Nullable
    pub session_id: *const libc::c_char,
    pub start_signal_ms: i64, // -1 mean None
}

unsafe impl Sync for CAsrStartListeningMessage {}

impl CAsrStartListeningMessage {
    pub fn from(input: hermes::AsrStartListeningMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::AsrStartListeningMessage> for CAsrStartListeningMessage {
    fn c_repr_of(input: hermes::AsrStartListeningMessage) -> Fallible<Self> {
        Ok(Self {
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
            start_signal_ms: input.start_signal_ms.unwrap_or(-1),
        })
    }
}

impl AsRust<hermes::AsrStartListeningMessage> for CAsrStartListeningMessage {
    fn as_rust(&self) -> Fallible<hermes::AsrStartListeningMessage> {
        Ok(hermes::AsrStartListeningMessage {
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_optional_rust_string_from!(self.session_id),
            start_signal_ms: if self.start_signal_ms == -1 {
                None
            } else {
                Some(self.start_signal_ms)
            },
        })
    }
}

impl Drop for CAsrStartListeningMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CTextCapturedMessage {
    pub text: *const libc::c_char,
    /// Nullable
    pub tokens: *const CAsrTokenArray,
    pub likelihood: f32,
    pub seconds: f32,
    pub site_id: *const libc::c_char,
    /// Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CTextCapturedMessage {}

impl CTextCapturedMessage {
    pub fn from(input: hermes::TextCapturedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::TextCapturedMessage> for CTextCapturedMessage {
    fn c_repr_of(input: hermes::TextCapturedMessage) -> Fallible<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            tokens: if let Some(tokens) = input.tokens {
                CAsrTokenArray::c_repr_of(tokens)?.into_raw_pointer()
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
    fn as_rust(&self) -> Fallible<hermes::TextCapturedMessage> {
        Ok(hermes::TextCapturedMessage {
            text: create_rust_string_from!(self.text),
            likelihood: self.likelihood,
            tokens: match unsafe { self.tokens.as_ref() } {
                Some(tokens) => Some(unsafe { CAsrTokenArray::raw_borrow(tokens)? }.as_rust()?),
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
        let _ = unsafe { CAsrTokenArray::drop_raw_pointer(self.tokens) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CAsrDecodingDuration {
    pub start: f32,
    pub end: f32,
}

impl CReprOf<hermes::AsrDecodingDuration> for CAsrDecodingDuration {
    fn c_repr_of(input: hermes::AsrDecodingDuration) -> Fallible<Self> {
        Ok(Self {
            start: input.start,
            end: input.end,
        })
    }
}

impl AsRust<hermes::AsrDecodingDuration> for CAsrDecodingDuration {
    fn as_rust(&self) -> Fallible<hermes::AsrDecodingDuration> {
        Ok(hermes::AsrDecodingDuration {
            start: self.start,
            end: self.end,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CAsrToken {
    pub value: *const libc::c_char,
    pub confidence: f32,
    pub range_start: i32,
    pub range_end: i32,
    pub time: CAsrDecodingDuration,
}

impl CReprOf<hermes::AsrToken> for CAsrToken {
    fn c_repr_of(input: hermes::AsrToken) -> Fallible<Self> {
        Ok(Self {
            value: convert_to_c_string!(input.value),
            confidence: input.confidence,
            range_start: input.range_start as i32,
            range_end: input.range_end as i32,
            time: CAsrDecodingDuration::c_repr_of(input.time)?,
        })
    }
}

impl AsRust<hermes::AsrToken> for CAsrToken {
    fn as_rust(&self) -> Fallible<hermes::AsrToken> {
        Ok(hermes::AsrToken {
            value: create_rust_string_from!(self.value),
            confidence: self.confidence,
            range_start: self.range_start as usize,
            range_end: self.range_end as usize,
            time: self.time.as_rust()?,
        })
    }
}

impl Drop for CAsrToken {
    fn drop(&mut self) {
        take_back_c_string!(self.value);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CAsrTokenArray {
    pub entries: *const *const CAsrToken,
    pub count: libc::c_int,
}

impl CReprOf<Vec<hermes::AsrToken>> for CAsrTokenArray {
    fn c_repr_of(input: Vec<hermes::AsrToken>) -> Fallible<Self> {
        let array = Self {
            count: input.len() as _,
            entries: Box::into_raw(
                input
                    .into_iter()
                    .map(|e| CAsrToken::c_repr_of(e).map(RawPointerConverter::into_raw_pointer))
                    .collect::<Fallible<Vec<_>>>()
                    .context("Could not convert map to C Repr")?
                    .into_boxed_slice(),
            ) as *const *const _,
        };
        Ok(array)
    }
}

impl AsRust<Vec<hermes::AsrToken>> for CAsrTokenArray {
    fn as_rust(&self) -> Fallible<Vec<hermes::AsrToken>> {
        let mut result = Vec::with_capacity(self.count as usize);
        for e in unsafe { slice::from_raw_parts(self.entries, self.count as usize) } {
            result.push(unsafe { CAsrToken::raw_borrow(*e) }?.as_rust()?);
        }
        Ok(result)
    }
}

impl Drop for CAsrTokenArray {
    fn drop(&mut self) {
        unsafe {
            let tokens = Box::from_raw(std::slice::from_raw_parts_mut(
                self.entries as *mut *mut CAsrToken,
                self.count as usize,
            ));
            for e in tokens.iter() {
                let _ = CAsrToken::drop_raw_pointer(*e);
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CAsrTokenDoubleArray {
    pub entries: *const *const CAsrTokenArray,
    pub count: libc::c_int,
}

impl CReprOf<Vec<Vec<hermes::AsrToken>>> for CAsrTokenDoubleArray {
    fn c_repr_of(input: Vec<Vec<hermes::AsrToken>>) -> Fallible<Self> {
        let array = Self {
            count: input.len() as _,
            entries: Box::into_raw(
                input
                    .into_iter()
                    .map(|e| CAsrTokenArray::c_repr_of(e).map(RawPointerConverter::into_raw_pointer))
                    .collect::<Fallible<Vec<_>>>()
                    .context("Could not convert map to C Repr")?
                    .into_boxed_slice(),
            ) as *const *const _,
        };
        Ok(array)
    }
}

impl AsRust<Vec<Vec<hermes::AsrToken>>> for CAsrTokenDoubleArray {
    fn as_rust(&self) -> Fallible<Vec<Vec<hermes::AsrToken>>> {
        let mut result = Vec::with_capacity(self.count as usize);

        for e in unsafe { slice::from_raw_parts(self.entries, self.count as usize) } {
            result.push(unsafe { CAsrTokenArray::raw_borrow(*e) }?.as_rust()?);
        }
        Ok(result)
    }
}

impl Drop for CAsrTokenDoubleArray {
    fn drop(&mut self) {
        unsafe {
            let tokens = Box::from_raw(std::slice::from_raw_parts_mut(
                self.entries as *mut *mut CAsrTokenArray,
                self.count as usize,
            ));

            for e in tokens.iter() {
                let _ = CAsrTokenArray::drop_raw_pointer(*e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::round_trip_test;
    use super::*;

    #[test]
    fn round_trip_asr_token() {
        round_trip_test::<_, CAsrToken>(hermes::AsrToken {
            value: "hello world".into(),
            confidence: 0.98,
            range_start: 4,
            range_end: 9,
            time: hermes::AsrDecodingDuration { start: 0.0, end: 5.0 },
        });
    }

    #[test]
    fn round_trip_asr_token_array() {
        round_trip_test::<_, CAsrTokenArray>(vec![]);

        round_trip_test::<_, CAsrTokenArray>(vec![
            hermes::AsrToken {
                value: "hello".to_string(),
                confidence: 0.98,
                range_start: 1,
                range_end: 4,
                time: hermes::AsrDecodingDuration { start: 0.0, end: 5.0 },
            },
            hermes::AsrToken {
                value: "world".to_string(),
                confidence: 0.73,
                range_start: 5,
                range_end: 9,
                time: hermes::AsrDecodingDuration { start: 0.0, end: 5.0 },
            },
        ]);
    }

    #[test]
    fn round_trip_asr_token_double_array() {
        round_trip_test::<_, CAsrTokenDoubleArray>(vec![]);

        round_trip_test::<_, CAsrTokenDoubleArray>(vec![
            vec![
                hermes::AsrToken {
                    value: "hello".to_string(),
                    confidence: 0.98,
                    range_start: 1,
                    range_end: 4,
                    time: hermes::AsrDecodingDuration { start: 0.0, end: 5.0 },
                },
                hermes::AsrToken {
                    value: "world".to_string(),
                    confidence: 0.73,
                    range_start: 5,
                    range_end: 9,
                    time: hermes::AsrDecodingDuration { start: 0.0, end: 5.0 },
                },
            ],
            vec![],
            vec![hermes::AsrToken {
                value: "yop".to_string(),
                confidence: 0.97,
                range_start: 5,
                range_end: 1,
                time: hermes::AsrDecodingDuration { start: 1.0, end: 4.5 },
            }],
        ]);
    }
}
