# -*- coding: utf-8 -*-
from __future__ import absolute_import
from __future__ import unicode_literals
from typing import List
from itertools import groupby
from typing import Optional, Text, Tuple, List, DefaultDict

from hermes_python.ffi.ontology.dialogue import CDialogueConfigureMessage

from .intent import *
from .slot import *
from .session import *


class DialogueConfiguration(object):
    """
    High level representation of DialogueConfigureMessage.
    """

    def __init__(self, site_id=None):
        # type: (Optional[Text]) -> None
        self.default_site_id = site_id
        self.intents = []  # type: List[Tuple[Optional[Text], Text, bool]]

    def disable_intent(self, intent_name, site_id=None):
        # type: (Text, Optional[Text]) -> DialogueConfiguration
        self.intents.append((site_id or self.default_site_id, intent_name, False))
        return self

    def disable_intents(self, intent_filter, site_id=None):
        # type: (List[Text], Optional[Text]) -> DialogueConfiguration
        for intent_name in intent_filter:
            self.disable_intent(intent_name, site_id)
        return self

    def enable_intent(self, intent_name, site_id=None):
        # type: (Text, Optional[Text]) -> DialogueConfiguration
        self.intents.append((site_id or self.default_site_id, intent_name, True))
        return self

    def enable_intents(self, intent_filter, site_id=None):
        # type: (List[Text], Optional[Text]) -> DialogueConfiguration
        for intent_name in intent_filter:
            self.enable_intent(intent_name, site_id)
        return self

    def for_site_id(self, site_id):
        # type: (Text) -> DialogueConfiguration
        self.default_site_id = site_id
        return self

    def build(self):
        # type: () -> List[DialogueConfigureMessage]
        """
        We perform the following :
        [("site_id1", "intent1", False), ("site_id1", "intent1", True), ("site_id2", "intent2", False), ("site_id1", "intent2", True)]
        =>
        [("site_id1", "intent1", [False, True] ),("site_id2", "intent2", False), ("site_id1", "intent2", [True]) ]
        =>
        {"site_id1":[("intent1, [False, True]), ("intent2", [True] )], "site_id2": [("intent2", False)]}
        =>
        {"site_id1": [("intent1", True)], "site_id2" : [("intent2", False)]}
        :return: List[DialogueConfigureMessage]
        """
        transformed_intents = [(site_id, intent_name, [_flag for _site, _intent_name, _flag in list(group)]) for
                               (site_id, intent_name), group in groupby(self.intents, lambda tuple: (
                tuple[0], tuple[1]))]  # Grouping elements by (site_id, intent_name)

        transformed_intents_grouped_by_site_id = defaultdict(list)  # type: DefaultDict[Optional[Text], List[Tuple[Text, List[bool]]]]

        for site_id, group in groupby(transformed_intents, lambda tuple: tuple[0]):
            for _site, _intent_name, _flags in list(group):
                transformed_intents_grouped_by_site_id[site_id].append((_intent_name, _flags))

        transformed_intents_grouped_by_site_id_flattened = {
            site_id: [(intent_name, flags[len(flags) - 1]) for intent_name, flags in value] for
            site_id, value in transformed_intents_grouped_by_site_id.items()}

        return [DialogueConfigureMessage(site_id,
                                         [DialogueConfigureIntent(_intent_name, _flag) for _intent_name, _flag in
                                          value]) for
                site_id, value in transformed_intents_grouped_by_site_id_flattened.items()]


class DialogueConfigureIntent(object):
    def __init__(self, intent_id, enable):
        # type: (Text, bool) -> None
        self.intent_id = intent_id
        self.enable = enable

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        intent_id = c_repr.intent_id.decode('utf-8')
        enable = True if c_repr.enable > 0 else False
        return cls(intent_id, enable)


class DialogueConfigureIntentArray(list):

    @classmethod
    def from_c_repr(cls, c_repr):
        intents_filter_length = c_repr.count
        c_intents_filter_array_repr = c_repr.entries
        return [DialogueConfigureIntent.from_c_repr(c_intents_filter_array_repr[i].contents) for i in
                range(intents_filter_length)]


class DialogueConfigureMessage(object):
    def __init__(self, site_id, intents):
        # type:(Optional[Text], List[DialogueConfigureIntent]) -> None
        self.site_id = site_id
        self.intents = intents

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    def into_c_repr(self):
        return CDialogueConfigureMessage.build(self.site_id, self.intents)

    @classmethod
    def from_c_repr(cls, c_repr):
        site_id = c_repr.site_id.decode('utf-8') if c_repr.site_id else None

        intents_filter_length = c_repr.intents.contents.count
        c_intents_filter_array_repr = c_repr.intents.contents.entries

        intents = [DialogueConfigureIntent.from_c_repr(c_intents_filter_array_repr[i].contents) for i in
                   range(intents_filter_length)]
        return cls(site_id, intents)
