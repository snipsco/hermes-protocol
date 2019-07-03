use super::HermesMessage;

/// This message is used to request the audio server to play a wav file
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayBytesMessage {
    /// An id for the request, it will be passed back in the `PlayFinishedMessage`
    pub id: String,
    /// The bytes of the wav to play (should be a regular wav with header)
    /// Note that serde json serialization is provided but in practice most handler impl will want
    /// to avoid the base64 encoding/decoding and give this a special treatment
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub wav_bytes: Vec<u8>,
    /// The site where the bytes should be played
    pub site_id: String,
}

impl<'de> HermesMessage<'de> for PlayBytesMessage {}

/// This message is used to request the audio server to play a part of a sound
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamBytesMessage {
    /// The play request identifier. This identifier will be passed to subsequent chunks along the
    /// chain
    pub stream_id: String,
    /// The bytes of the chunk to play (should be a regular wav with header)
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub bytes: Vec<u8>,
    /// The site where the audio should be played
    pub site_id: String,
    /// The number of the chunk in the chain
    pub chunk_number: u32,
    /// Boolean signaling if this is the last audio chunk of the chain
    pub is_last_chunk: bool,
}

impl<'de> HermesMessage<'de> for StreamBytesMessage {}

/// This message is used for the audio streaming on the snips platform. It is used both for normal
/// streaming and replay streaming.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFrameMessage {
    /// The bytes of the WAV frame (should be a regular WAV with header).
    ///
    /// The WAV should be encoded in S16LE Mono 16000Hz.
    ///
    /// In order to provide full support for the hermes protocol, the WAV should contain some
    /// additional metadata chunks. Note that not all WAV libraries support crafting arbitrary
    /// chunks
    ///
    /// | FourCC |  Type  | Description                                     |
    /// |--------|--------|-------------------------------------------------|
    /// | `time` | U64LE  | Timestamp when the frame was captured           |
    /// | `rpid` | String | *(optional)* replay request_id                  |
    /// | `rprf` | U32LE  | *(optional)* remaining frames in current replay |
    ///
    /// Here's an example header of a frame
    ///
    /// | WAV bytes   | Description                                                               |
    /// |-------------|---------------------------------------------------------------------------|
    /// |`52 49 46 46`| `RIFF`                                                                    |
    /// |`34 02 00 00`| 564(u32) = data is 564 bytes long, [be sure to adjust if using replay](#1)|
    /// |`57 41 56 45`| `WAVE`                                                                    |
    /// |`66 6d 74 20`| `fmt_` (`_` is a space) chunk                                             |
    /// |`10 00 00 00`| 16(u32) = value 16 bytes long                                             |
    /// |`01 00 01 00`| 1(u16) = PCM format, 1(u16) = 1 channel                                   |
    /// |`80 3e 00 00`| 16000(u32) = sample rate is 16000Hz                                       |
    /// |`00 7d 00 00`| 32000(u32) = bitrate is 32000Bps                                          |
    /// |`02 00 10 00`| 2(u16) = [sample alignment](#2) is 2, 16(u16) = 16 bits per sample        |
    /// |`74 69 6d 65`| `time` (snips metadata chunk)                                             |
    /// |`08 00 00 00`| 8(u32) = value for `time` is 8 bytes long                                 |
    /// |`b6 82 8a d4`| timestamp in ms encoded in U64LE (here "1558344008374")                   |
    /// |`6a 01 00 00`| (end of timestamp)                                                        |
    /// |             | **optional section added only in the case of a replay**                   |
    /// |`72 70 69 64`| `rpid` (snips metadata chunk)                                             |
    /// |`04 00 00 00`| 4(u32) = value is 4 bytes long, [may change depending on the id](#3)      |
    /// |`66 6f 6f 6f`| `fooo` (the replay request id), [may need to be padded](#4)               |
    /// |`72 70 69 64`| `rprf` (snips metadata chunk)                                             |
    /// |`04 00 00 00`| 4(u32) = value is 4 bytes long                                            |
    /// |`02 00 00 00`| 2(u32) = 2 replay frames remaining (counting this one)                    |
    /// |             | **end optional section**                                                  |
    /// |`64 61 74 61`| `data` chunk                                                              |
    /// |`00 02 00 00`| 512(u32) = data is 512 bytes long                                         |
    /// |`00 00 00 00`| 256 samples (S16LE)                                                       |
    /// |`00 00 00 00`| ...                                                                       |
    /// |`...`        | ...                                                                       |
    ///
    /// **Notes**
    ///
    /// <a name="1">#1</a>: Data size is  `<size of the wav> - 8` should be 564 for a standard frame
    /// not using replay. A frame with using replay with a 12 char replay request id should have a
    /// data size of 594.
    ///
    /// <a name="2">#2</a>: Sample alignment is `<channel count> x <bytes per sample>`.
    ///
    /// <a name="3">#3</a>: be sure to adjust the `rpid` size to the length of the replay id string,
    /// without any additional padding.
    ///
    /// <a name="4">#4</a>: be sure to add an extra byte (why not `x00`) if the total number of
    /// bytes containing the id is odd. To preserve the file alignment and ensure decodability.
    ///
    /// Note that serde json serialization is provided but in practice most handler impl will want
    /// to avoid the base64 encoding/decoding and give this a special treatment
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub wav_frame: Vec<u8>,
    /// The site this frame originates from
    pub site_id: String,
}

impl<'de> HermesMessage<'de> for AudioFrameMessage {}

/// This message is used to ask the audio server to replay the audio streaming stating a a specific
/// time. The audio server implementation is expected to be able to replay frames from a few seconds
/// in the past. Replayed frames go through the same canal as normal frames and are identified by a
/// special metadata in the INFO chunk
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayRequestMessage {
    /// An id for the request, it will be passed back in the replayed frames headers.
    pub request_id: String,
    /// When to start replay from
    pub start_at_ms: i64,
    /// The site this frame originates from
    pub site_id: String,
}

impl<'de> HermesMessage<'de> for ReplayRequestMessage {}

/// This message is send by the audio server when a wav has finished playing
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayFinishedMessage {
    /// The id of the `PlayBytesMessage` which bytes finished playing
    pub id: String,
    /// The site where the sound was played
    pub site_id: String,
}

/// This message is send by the audio server when a audio stream has finished playing
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamFinishedMessage {
    /// The id of the `StreamBytesMessage` which bytes finished playing
    pub id: String,
    /// The site where the sound was played
    pub site_id: String,
}

impl<'de> HermesMessage<'de> for PlayFinishedMessage {}

fn as_base64<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&base64::encode(bytes))
}

fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}
