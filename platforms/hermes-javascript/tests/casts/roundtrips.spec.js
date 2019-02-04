const ref = require('ref')
const Int64 = require('node-int64')
const {
    Dialog,
    Injection
} = require('../../dist')
const {
    Casteable,
    StringArray,
    StartSessionMessage,
    IntentMessage,
    InjectionRequestMessage,
    PlayBytesMessage
} = require('../../dist/casts')
const {
    CContinueSessionMessage,
    CEndSessionMessage,
    CSessionStartedMessage,
    CIntentMessage,
    CIntentNotRecognizedMessage,
    CSessionQueuedMessage,
    CSessionEndedMessage,
    CSessionTermination,
    CInjectionRequestMessage,
    CSiteMessage,
    CInjectionStatusMessage,
    CPlayFinishedMessage
} = require('../../dist/ffi/typedefs')

// Log segfaults
const SegfaultHandler = require('segfault-handler')
SegfaultHandler.registerHandler('crash.log')

// Rust round trip tests
const rustRoundTrip = require('./rustTestsWrapper').call()

// Serialize <-> Deserialize helper
function roundTrip({ data, MessageClass = Casteable, forgeType, forgeOptions = {}, roundTripOptions = {}, FFIFunctionName }) {
    try {
        const pojo = new MessageClass(data)
        expect(pojo).toEqual(data)
        const cStructPointer = pojo.forge(forgeType, forgeOptions).ref()
        const StructType = forgeType || pojo.type
        let roundTrip
        if(FFIFunctionName) {
            const mutableReference = ref.NULL_POINTER.ref()
            rustRoundTrip(FFIFunctionName, cStructPointer, mutableReference)
            const rustAllocatedPointer = ref.reinterpret(mutableReference.deref(), StructType.size)
            const struct = StructType.get(rustAllocatedPointer)
            roundTrip = new MessageClass(struct.ref(), roundTripOptions)
        } else {
            roundTrip = new MessageClass(cStructPointer, roundTripOptions)
        }
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
            data: {
                session_init: {
                    init_type: Dialog.enums.initType.action,
                    value: {
                        text: 'toto',
                        intent_filter: ['intent', 'filter'],
                        can_be_enqueued: 1,
                        send_intent_not_recognized: 0
                    }
                },
                custom_data: 'customThing',
                site_id: 'siteId'
            },
            MessageClass: StartSessionMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_start_session'
        })
        //  Notification
         roundTrip({
            data: {
                session_init: {
                    init_type: Dialog.enums.initType.notification,
                    value: 'notification'
                },
                custom_data: 'customThing',
                site_id: 'siteId'
            },
            MessageClass: StartSessionMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_start_session'
        })
    })
    it('ContinueSessionMessage', () => {
        roundTrip({
            data: {
                session_id: 'Session id',
                text: 'Session resumed',
                custom_data: 'customThing',
                send_intent_not_recognized: 1,
                intent_filter: ['intent1', 'intent2']
            },
            forgeType: CContinueSessionMessage,
            forgeOptions: {
                intent_filter: intents => new StringArray(intents).forge()
            },
            roundTripOptions: {
                intent_filter: intents => new StringArray(intents)._array
            },
            FFIFunctionName: 'hermes_ffi_test_round_trip_continue_session'
        })
    })
    it('EndSessionMessage', () => {
       roundTrip({
           data: {
                session_id: 'Session id',
                text: 'Session ended'
           },
           forgeType: CEndSessionMessage,
           FFIFunctionName: 'hermes_ffi_test_round_trip_end_session'
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
                    confidence_score: 0.5
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
                    confidence_score: 0.5,
                    raw_value: 'vert',
                    value: {
                        value_type: Dialog.enums.slotType.custom,
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
                    confidence_score: 0.5
                },
                asr_tokens: null,
                slots: [
                    {
                        confidence_score: 0.5,
                        raw_value: 'un',
                        value: {
                            value: 1.2,
                            value_type: Dialog.enums.slotType.number
                        },
                        range_start: 11,
                        range_end: 13,
                        entity: 'snips/number',
                        slot_name: 'firstTerm'
                    },
                    {
                        confidence_score: 0.5,
                        raw_value: 'un',
                        value: {
                            value: 1.5,
                            value_type: Dialog.enums.slotType.number
                        },
                        range_start: 19,
                        range_end: 21,
                        entity: 'snips/number',
                        slot_name: 'secondTerm'
                    },
                    {
                        confidence_score: 0.5,
                        raw_value: 'un',
                        value: {
                            value: new Int64(101),
                            value_type: Dialog.enums.slotType.ordinal
                        },
                        range_start: 19,
                        range_end: 21,
                        entity: 'snips/ordinal',
                        slot_name: 'secondTerm'
                    }
                ]
            },
            MessageClass: IntentMessage,
            forgeType: CIntentMessage
        })
        roundTrip({
            data: {
                session_id: 'fad16235-2b00-48fb-8684-d729284686f5',
                custom_data: null,
                site_id: 'default',
                input: 'what will be the weather in parish this sunday',
                asr_tokens: null,
                intent: {
                    intent_name: 'davidsnips:WeatherForecast',
                    confidence_score: 0.6884455680847168
                },
                slots: [{
                    confidence_score: 1.0,
                    raw_value: 'this sunday',
                    value: {
                        value_type: Dialog.enums.slotType.instantTime,
                        value: {
                            value: '2019-01-06 00:00:00 +01:00',
                            grain: 4,
                            precision: 1
                        }
                    },
                    range_start: 35,
                    range_end: 46,
                    entity: 'snips/datetime',
                    slot_name: 'forecast_datetime'
                }]
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
                custom_data: null,
                confidence_score: 0.5
            },
            forgeType: CIntentNotRecognizedMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_intent_not_recognized'
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
            forgeType: CSessionStartedMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_session_started'
        })
    })
    it('SessionQueuedMessage', () => {
        roundTrip({
            data: {
                session_id: 'Session id',
                custom_data: 'Custom data',
                site_id: 'Site id'
            },
            forgeType: CSessionQueuedMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_session_queued'
        })
    })
    it('SessionEndedMessage', () => {
        roundTrip({
            data: {
                session_id: 'Session id',
                custom_data: 'Custom data',
                termination: {
                    termination_type: Dialog.enums.terminationType.error,
                    data: 'Error message'
                },
                site_id: 'Site id'
            },
            forgeType: CSessionEndedMessage,
            forgeOptions: {
                termination: termination =>
                    new Casteable(termination).forge(CSessionTermination)
            },
            FFIFunctionName: 'hermes_ffi_test_round_trip_session_ended'
        })
    })
    it('InjectionRequestMessage', () => {
        roundTrip({
            data: {
                cross_language: '123',
                id: '456',
                lexicon: {
                    films : [
                        'The Wolf of Wall Street',
                        'The Lord of the Rings'
                    ]
                },
                operations: [
                    {
                        kind: Injection.enums.injectionKind.add,
                        values: {
                            films : [
                                'The Wolf of Wall Street',
                                'The Lord of the Rings'
                            ]
                        }
                    }
                ]
            },
            MessageClass: InjectionRequestMessage,
            forgeType: CInjectionRequestMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_injection_request'
        })
    })
    it('[norust] SiteMessage', () => {
        roundTrip({
            data: {
                site_id: 'default',
                session_id: 'session id'
            },
            forgeType: CSiteMessage
        })
    })
    it('[norust] InjectionStatus', () => {
        roundTrip({
            data: {
                last_injection_date: '2018-12-10T11:14:08.468Z'
            },
            forgeType: CInjectionStatusMessage
        })
    })
    it('[norust] PlayBytesMessage', () => {
        const wavBuffer = Buffer.from([0x00, 0x01, 0x02, 0x03])
        roundTrip({
            data: {
                id: 'ABCDEF',
                site_id: 'default',
                wav_bytes: wavBuffer,
                wav_bytes_len:  wavBuffer.length,
            },
            MessageClass: PlayBytesMessage
        })
    })
    it('[norust] PlayFinishedMessage', () => {
        roundTrip({
            data: {
                id: 'ABCDEF',
                site_id: 'default'
            },
            forgeType: CPlayFinishedMessage
        })
    })
})
