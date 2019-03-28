from ctypes import Structure, c_void_p


class CTtsFacade(Structure):
    _fields_ = [("facade", c_void_p)]


class CDialogueFacade(Structure):
    _fields_ = [("facade", c_void_p)]
