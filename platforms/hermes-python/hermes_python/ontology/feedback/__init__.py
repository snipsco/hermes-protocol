# -*- coding: utf-8 -*-
from __future__ import absolute_import
from __future__ import unicode_literals

from typing import Optional, Text
from hermes_python.ffi.ontology.feedback import CSiteMessage


class SiteMessage(object):
    def __init__(self, site_id, session_id=None):
        # type: (Text, Optional[Text]) -> None
        self.site_id = site_id
        self.session_id = session_id

    def into_c_repr(self):
        return CSiteMessage.build(self.site_id, self.session_id)

    @classmethod
    def from_c_repr(cls, c_repr):
        site_id = c_repr.site_id.decode('utf-8')
        session_id = c_repr.session_id.decode('utf-8') if c_repr.session_id else None
        return cls(site_id, session_id)
