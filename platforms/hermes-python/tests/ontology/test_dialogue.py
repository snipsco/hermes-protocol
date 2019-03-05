from __future__ import unicode_literals
import hermes_python

import pytest


def test_slot_access_dot_notation():
    from hermes_python.ontology.dialogue import SlotsList
    slots = hermes_python.ontology.dialogue.SlotMap({"test_slot": SlotsList()})
    assert type(slots.test_slot) is SlotsList


def test_slot_access_dict_notation():
    from hermes_python.ontology.dialogue import SlotsList
    slots = hermes_python.ontology.dialogue.SlotMap({"test_slot": SlotsList()})
    assert type(slots["test_slot"]) is SlotsList



def test_unseen_slot_access_1():
    slots = hermes_python.ontology.dialogue.SlotMap({})
    assert slots.unseen_slot.first() is None


def test_unseen_slot_access_2():
    slots = hermes_python.ontology.dialogue.SlotMap({})
    assert len(slots.unseen_slot) == 0


def test_unseen_slot_access_3():
    slots = hermes_python.ontology.dialogue.SlotMap({})
    assert slots.unseen_slot.all() is None
