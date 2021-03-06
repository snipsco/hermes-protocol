from __future__ import unicode_literals
import hermes_python

from hermes_python.ontology.dialogue import DialogueConfiguration


def test_slot_access_dot_notation():
    from hermes_python.ontology.nlu import SlotsList
    slots = hermes_python.ontology.dialogue.SlotMap({"test_slot": SlotsList()})
    assert type(slots.test_slot) is SlotsList


def test_slot_access_dict_notation():
    from hermes_python.ontology.nlu import SlotsList
    slots = hermes_python.ontology.dialogue.SlotMap({"test_slot": SlotsList()})
    assert type(slots["test_slot"]) is SlotsList


def test_helper_method_access():
    from hermes_python.ontology.slot import SlotValue, CustomValue
    from hermes_python.ontology.nlu import SlotsList, NluSlot, SlotMap

    custom_value_slot = CustomValue("Hello world")
    nlu_slot = NluSlot(SlotValue(1, custom_value_slot), "test", [], "test", "test_slot", 0, 0, [])
    slots = SlotMap({"test_slot": SlotsList()})

    slots.test_slot.append(nlu_slot)

    assert type(slots.test_slot.first()) is CustomValue


def test_unseen_slot_access_1():
    slots = hermes_python.ontology.dialogue.SlotMap({})
    assert len(slots) == 0
    assert len(slots.unseen_slots) == 0
    assert len(slots) == 0
    assert slots.unseen_slot.first() is None
    assert slots.unseen_slot.all() is None


def test_unseen_slot_access_2():
    slots = hermes_python.ontology.dialogue.SlotMap({})
    assert len(slots) == 0
    assert len(slots.unseen_slot) == 0
    assert len(slots) == 0


def test_unseen_slot_access_3():
    slots = hermes_python.ontology.dialogue.SlotMap({})
    assert slots.unseen_slot.all() is None


def test_unseen_slot_acces_dict_notation():
    slots = hermes_python.ontology.dialogue.SlotMap({})
    assert len(slots['unseen_slot']) == 0
    assert slots['unseen_slot'].first() is None
    assert slots['unseen_slot'].all() is None


def test_slot_map_items_iteration():
    from hermes_python.ontology.nlu import SlotsList
    slots = hermes_python.ontology.dialogue.SlotMap({"test_slot": SlotsList()})

    for slot, slot_value_list in slots.items():
        assert slot == "test_slot"
        assert len(slot_value_list) == 0
