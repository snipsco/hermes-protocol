from __future__ import absolute_import
from __future__ import unicode_literals

from ctypes import c_char_p, c_int32, c_void_p, c_uint8, POINTER, Structure


class CStringArray(Structure):
    _fields_ = [
        ("data", POINTER(c_char_p)),
        ("size", c_int32)
    ]


class CProtocolHandler(Structure):
    _fields_ = [("handler", c_void_p)]


class CMqttOptions(Structure):
    _fields_ = [("broker_address", c_char_p),
                ("username", c_char_p),
                ("password", c_char_p),
                ("tls_hostname", c_char_p),
                ("tls_ca_file", POINTER(CStringArray)),
                ("tls_ca_path", POINTER(CStringArray)),
                ("tls_client_key", c_char_p),
                ("tls_client_cert", c_char_p),
                ("tls_disable_root_store", c_uint8)]

    @classmethod
    def build(cls, broker_address, username, password, tls_hostname, tls_ca_file, tls_ca_path, tls_client_key, tls_client_cert, tls_disable_root_store):
        broker_address = broker_address.encode('utf-8')
        username = username.encode('utf-8') if username else None
        password = password.encode('utf-8') if password else None
        tls_hostname = tls_hostname.encode('utf-8') if tls_hostname else None
        tls_ca_file = tls_ca_file.encode('utf-8') if tls_ca_file else None
        tls_ca_path = tls_ca_path.encode('utf-8') if tls_ca_path else None
        tls_client_key = tls_client_key.encode('utf-8') if tls_client_key else None
        tls_client_cert = tls_client_cert.encode('utf-8') if tls_client_cert else None
        tls_disable_root_store = 1 if tls_disable_root_store else 0  # tls_disable_root_store is a boolean

        return cls(broker_address,
                   username, password,
                   tls_hostname, tls_ca_file, tls_ca_path, tls_client_key, tls_client_cert, tls_disable_root_store)

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr.broker_address,
                         repr.username, repr.password,
                         repr.tls_hostname, repr.tls_ca_file, repr.tls_ca_path, repr.tls_client_key, repr.tls_client_cert, repr.tls_disable_root_store)

