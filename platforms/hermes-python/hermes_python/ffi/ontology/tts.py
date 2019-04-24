# -*- coding: utf-8 -*-
from __future__ import absolute_import
from __future__ import unicode_literals

from ctypes import Structure, POINTER, pointer, c_char_p, c_uint8, c_int


class CRegisterSoundMessage(Structure):
    _fields_ = [("sound_id", c_char_p),
                ("wav_sound", POINTER(c_uint8)),
                ("wav_sound_len", c_int)]

    @classmethod
    def from_repr(cls, repr):
        sound_id = c_char_p(repr.sound_id.encode('utf-8'))

        wav_length = len(repr.wav_sound)
        wav_bytes = (c_uint8 * wav_length)()
        wav_bytes[:] = repr.wav_sound

        return cls(sound_id, wav_bytes, wav_length)
