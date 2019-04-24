from typing import Text, ByteString
from hermes_python.ffi.ontology.tts import CRegisterSoundMessage

class RegisterSoundMessage(object):
    def __init__(self, sound_id, wav_sound):
        # type:(Text, ByteString) -> None
        self.sound_id = sound_id
        self.wav_sound = wav_sound

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        sound_id = c_repr.sound_id.decode('utf-8')
        nb = c_repr.wav_sound_len
        wav_sound = bytearray([c_repr.wav_sound[i] for i in range(nb)])
        return cls(sound_id, wav_sound)

    def into_c_repr(self):
        return CRegisterSoundMessage.from_repr(self)

