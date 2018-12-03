const {
    Casteable,
    StringArray,
    StartSessionMessage,
    IntentMessage
} = require('../../src/casts')
const {
    CContinueSessionMessage,
    CEndSessionMessage,
    CSessionStartedMessage,
    CIntentMessage,
    CIntentNotRecognizedMessage,
    CSessionQueuedMessage,
    CSessionEndedMessage,
    CSessionTermination
} = require('../../src/ffi/typedefs')

// Log segfaults
const SegfaultHandler = require('segfault-handler')
SegfaultHandler.registerHandler('crash.log')

// Serialize <-> Deserialize helper
function roundTrip({ data, MessageClass = Casteable, forgeType, forgeOptions = {}, roundTripOptions = {} }) {
    try {
        const pojo = new MessageClass(data)
        const cPointer = pojo.forge(forgeType, forgeOptions).ref()
        const roundTrip = new MessageClass(cPointer, roundTripOptions)
        expect(pojo).toEqual(roundTrip)
    } catch(error) {
        console.log(error)
        throw error
    }
}

describe('It should perform casting round-trips on messages', () => {
    it('StartSessionMessage', () => {
        // Action
        roundTrip({
            MessageClass: StartSessionMessage,
            data: {
                session_init: {
                    init_type: 1,
                    value: {
                        text: 'toto',
                        intent_filter: ['intent', 'filter'],
                        can_be_enqueued: 'Y',
                        send_intent_not_recognized: 0
                    }
                },
                custom_data: 'customThing',
                site_id: 'siteId'
            }
        })
         // Notification
         roundTrip({
            MessageClass: StartSessionMessage,
            data: {
                session_init: {
                    init_type: 2,
                    value: 'notification'
                },
                custom_data: 'customThing',
                site_id: 'siteId'
            }
        })
    })
    it('ContinueSessionMessage', () => {
        roundTrip({
            data: {
                session_id: 'Session id',
                text: 'Session resumed',
                custom_data: 'customThing',
                send_intent_not_recognized: 0,
                intent_filter: ['intent1', 'intent2']
            },
            forgeType: CContinueSessionMessage,
            forgeOptions: {
                intent_filter: intents => new StringArray(intents).forge()
            },
            roundTripOptions: {
                intent_filter: intents => new StringArray(intents)._array
            }
        })
    })
    it('EndSessionMessage', () => {
       roundTrip({
           data: {
                session_id: 'Session id',
                text: 'Session ended'
           },
           forgeType: CEndSessionMessage
       })
    })
    it('IntentMessage(s)', () => {
        roundTrip({
            data: {
                session_id: '677a2717-7ac8-44f8-9013-db2222f7923d',
                custom_data: 'customThing',
                site_id: 'default',
                input: 'moi du vert',
                intent: {
                    intent_name: 'jelb:lightsColor',
                    probability: 0.5
                },
                asr_tokens: [
                    [{
                        value: 'moi',
                        confidence: 0.5,
                        range_start: 0,
                        range_end: 3,
                        time: {
                            start: 0.5,
                            end: 1.0
                        }
                    }, {
                        value: 'du',
                        confidence: 0.5,
                        range_start: 4,
                        range_end: 6,
                        time: {
                            start: 1.0,
                            end: 1.5
                        }
                    }, {
                        value: 'vert',
                        confidence: 0.5,
                        range_start: 7,
                        range_end: 11,
                        time: {
                            start: 1.5,
                            end: 2.5
                        }
                    }]
                ],
                slots: [{
                    confidence: 0.5,
                    raw_value: 'vert',
                    value: {
                        value_type: 1,
                        value: 'vert'
                    },
                    range_start: 7,
                    range_end: 11,
                    entity: 'Color',
                    slot_name: 'Color'
                }]
            },
            MessageClass: IntentMessage,
            forgeType: CIntentMessage
        })
        roundTrip({
            data: {
                session_id: '6ce651f7-0aec-4910-bfec-b246ea6ca550',
                custom_data: 'data',
                site_id: 'default',
                input: 'additionne un plus un',
                intent: {
                    intent_name: 'jelb:getAddition',
                    probability: 0.5
                },
                asr_tokens: null,
                slots: [
                    {
                        confidence: 0.5,
                        raw_value: 'un',
                        value: {
                            value: 1.0,
                            value_type: 2
                        },
                        range_start: 11,
                        range_end: 13,
                        entity: 'snips/number',
                        slot_name: 'firstTerm'
                    },
                    {
                        confidence: 0.5,
                        raw_value: 'un',
                        value: {
                            value: 1.0,
                            value_type: 2
                        },
                        range_start: 19,
                        range_end: 21,
                        entity: 'snips/number',
                        slot_name: 'secondTerm'
                    }
                ]
            },
            MessageClass: IntentMessage,
            forgeType: CIntentMessage
        })
    })
    it('IntentNotRecognizedMessage(s)', () => {
        roundTrip({
            data: {
                site_id: 'default',
                session_id: '6ce651f7-0aec-4910-bfec-b246ea6ca550',
                input: 'additionne un plus un',
                custom_data: null
            },
            forgeType: CIntentNotRecognizedMessage
        })
    })
    it('SessionStartedMessage', () => {
        roundTrip({
            data: {
                session_id: 'Session id',
                custom_data: 'Custom data',
                site_id: 'Site id',
                reactivated_from_session_id: 'Reactivated from session id'
            },
            forgeType: CSessionStartedMessage
        })
    })
    it('SessionQueuedMessage', () => {
        roundTrip({
            data: {
                session_id: 'Session id',
                custom_data: 'Custom data',
                site_id: 'Site id'
            },
            forgeType: CSessionQueuedMessage
        })
    })
    it('SessionEndedMessage', () => {
        roundTrip({
            data: {
                session_id: 'Session id',
                custom_data: 'Custom data',
                termination: {
                    termination_type: 1,
                    data: 'Data'
                },
                site_id: 'Site id'
            },
            forgeType: CSessionEndedMessage,
            forgeOptions: {
                termination: termination =>
                    new Casteable(termination).forge(CSessionTermination)
            }
        })
    })
})
