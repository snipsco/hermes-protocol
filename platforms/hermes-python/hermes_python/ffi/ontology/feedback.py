from ctypes import Structure, c_char_p


class CSiteMessage(Structure):
    _fields_ = [("site_id", c_char_p),
                ("session_id", c_char_p)]

    @classmethod
    def build(cls, site_id, session_id=None):
        site_id = site_id.encode('utf-8')
        session_id = session_id.encode('utf-8') if session_id else None

        return cls(site_id, session_id)

    @classmethod
    def from_repr(cls, repr):
        site_id = repr.site_id
        session_id = repr.session_id

        return cls.build(site_id, session_id)
