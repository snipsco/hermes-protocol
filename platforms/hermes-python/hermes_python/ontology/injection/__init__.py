from typing import Optional, Text, List, Mapping
from ...ffi.ontology.injection import InjectionKind, CInjectionRequestMessage, CInjectionResetCompleteMessage, CInjectionResetRequestMessage, CInjectionCompleteMessage


class InjectionStatusMessage(object):
    def __init__(self, last_injection_date):
        self.last_injection_date = last_injection_date

    @classmethod
    def from_c_repr(cls, c_repr):
        last_injection_date = c_repr.last_injection_date.decode('utf-8')
        return cls(last_injection_date)


class InjectionRequestMessage(object):
    def __init__(self, operations, lexicon=dict(), cross_language=None, id=None):
        # type: (List[InjectionRequestOperation], Mapping[Text, List[Text]], Optional[Text], Optional[Text]) -> None
        """
        :param operations: List of operations to execute in the order of the list on a model
        :type operations: List[InjectionRequestOperation]
        :param lexicon: List of pre-computed prononciations to add in a model
        :type lexicon: Mapping[Text, List[Text]]
        :param cross_language: Language for cross-language G2P
        :type cross_language: Optional[Text]
        :param id: The id of the `InjectionRequestMessage` that was processed
        :type id: Optional[Text]
        """
        self.operations = operations
        self.lexicon = lexicon
        self.cross_language = cross_language
        self.id = id

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        number_of_operations = c_repr.operations.contents.count
        c_operations_array_repr = c_repr.operations.contents.operations
        operations = [InjectionRequestOperation.from_c_repr(c_operations_array_repr[i].contents) for i in
                      range(number_of_operations)]

        lexicon = c_repr.lexicon.contents.into_repr()

        cross_language = c_repr.id.decode('utf-8') if c_repr.cross_language else None
        id = c_repr.id.decode('utf-8') if c_repr.id else None
        return cls(operations, lexicon, cross_language, id)

    def into_c_repr(self):
        return CInjectionRequestMessage.from_repr(self)


class InjectionRequestOperation(object):
    def __init__(self, values):
        self.values = values

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        kind = InjectionKind(c_repr.kind)
        if kind == InjectionKind.ADD_FROM_VANILLA:
            return AddFromVanillaInjectionRequest.from_c_repr(c_repr)
        elif kind == InjectionKind.ADD:
            return AddInjectionRequest.from_c_repr(c_repr)
        else:
            raise ("Unknown injection kind")


class AddInjectionRequest(InjectionRequestOperation):
    def __init__(self, values):
        # type:(Mapping[Text, List[Text]]) -> None
        self.kind = InjectionKind.ADD
        self.values = values

    @classmethod
    def from_c_repr(cls, c_repr):
        values = c_repr.values.contents.into_repr()
        return cls(values)


class AddFromVanillaInjectionRequest(InjectionRequestOperation):
    def __init__(self, values):
        self.kind = InjectionKind.ADD_FROM_VANILLA
        self.values = values

    @classmethod
    def from_c_repr(cls, c_repr):
        values = c_repr.values.contents.into_repr()
        return cls(values)


class InjectionCompleteMessage(object):
    def __init__(self, request_id):
        """
        :param request_id: The id of the injection request that just completed.
        :type request_id: Text
        """
        # type: (Text)
        self.request_id = request_id

    def __eq__(self, other):
        return other.__dict__ == self.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        request_id = c_repr.request_id.decode('utf-8')
        return cls(request_id)

    def into_c_repr(self):
        return CInjectionCompleteMessage.from_repr(self)



class InjectionResetRequestMessage(object):
    def __init__(self, request_id):
        """
        :param request_id: The id of the injection reset request.
        :type request_id: Text
        """
        # type: (Text)
        self.request_id = request_id

    def __eq__(self, other):
        return other.__dict__ == self.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        request_id = c_repr.request_id.decode('utf-8')
        return cls(request_id)

    def into_c_repr(self):
        return CInjectionResetRequestMessage.from_repr(self)


class InjectionResetCompleteMessage(object):
    def __init__(self, request_id):
        """
        :param request_id: The id of the injection reset request that just completed.
        :type request_id: Text
        """
        # type: (Text)
        self.request_id = request_id

    def __eq__(self, other):
        return other.__dict__ == self.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        request_id = c_repr.request_id.decode('utf-8')
        return cls(request_id)

    def into_c_repr(self):
        return CInjectionResetCompleteMessage.from_repr(self)


