import ref from 'ref'
import { call } from './jsonWrapper'

import {
    Dialog,
    Injection,
    StartSessionMessage,
    ContinueSessionMessage,
    EndSessionMessage,
    IntentMessage,
    IntentNotRecognizedMessage,
    SessionStartedMessage,
    SessionQueuedMessage,
    SessionEndedMessage,
    InjectionRequestMessage,
    RegisterSoundMessage,
    DialogueConfigureMessage,
    TextCapturedMessage,
    component
} from '../../dist'

// Rust round trip tests
const rustRoundTrip = call()

// Serialize <-> Deserialize helper
function roundTrip({ data, FFIFunctionName }) {
    try {
        const mutableReference = ref.alloc('char **')
        rustRoundTrip(FFIFunctionName, JSON.stringify(data), mutableReference)
        const rustAllocatedString = (mutableReference as any).deref().readCString()
        expect(JSON.parse(rustAllocatedString)).toMatchObject(data)
    } catch(error) {
        // eslint-disable-next-line
        console.log(error)
        throw error
    }
}

describe('It should perform json round-trips on messages', () => {
    it('StartSessionMessage', () => {
        const actionStartSessionMessage: StartSessionMessage = {
            init: {
                type: Dialog.enums.initType.action,
                text: 'Hello world!',
                intentFilter: [ 'intent', 'filter' ],
                sendIntentNotRecognized: false,
                canBeEnqueued: true

            },
            customData: 'customThing',
            siteId: 'default'
        }

        const notificationStartSessionMessage: StartSessionMessage = {
            init: {
                type: Dialog.enums.initType.notification,
                text: 'Hello world!'

            },
            customData: 'customThing',
            siteId: 'default'
        }

        // Action
        roundTrip({
            data: actionStartSessionMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_start_session_json'
        })
        //  Notification
         roundTrip({
            data: notificationStartSessionMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_start_session_json'
        })
    })

    it('ContinueSessionMessage', () => {
        const continueSessionMessage: ContinueSessionMessage = {
            sessionId: 'session id',
            text: 'text',
            intentFilter: ['intentA', 'intentB'],
            sendIntentNotRecognized: true,
            customData: 'custom data',
            slot: 'slot'
        }

        roundTrip({
            data: continueSessionMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_continue_session_json'
        })
    })


    it('EndSessionMessage', () => {
        const endSessionMessage: EndSessionMessage = {
            sessionId: 'session id',
            text: 'Hello world.'
        }
        roundTrip({
           data: endSessionMessage,
           FFIFunctionName: 'hermes_ffi_test_round_trip_end_session_json'
        })
    })


    it('IntentMessage(s)', () => {
        const customIntentMessage: IntentMessage = {
            sessionId: '677a2717-7ac8-44f8-9013-db2222f7923d',
            customData: 'customThing',
            siteId: 'default',
            input: 'moi du vert',
            intent: {
                intentName: 'jelb:lightsColor',
                confidenceScore: 0.5
            },
            asrConfidence: 1,
            asrTokens: [
                [{
                    value: 'moi',
                    confidence: 0.5,
                    rangeStart: 0,
                    rangeEnd: 3,
                    time: {
                        start: 0.5,
                        end: 1.0
                    }
                }, {
                    value: 'du',
                    confidence: 0.5,
                    rangeStart: 4,
                    rangeEnd: 6,
                    time: {
                        start: 1.0,
                        end: 1.5
                    }
                }, {
                    value: 'vert',
                    confidence: 0.5,
                    rangeStart: 7,
                    rangeEnd: 11,
                    time: {
                        start: 1.5,
                        end: 2.5
                    }
                }]
            ],
            slots: [{
                confidenceScore: 0.5,
                rawValue: 'vert',
                value: {
                    kind: Dialog.enums.slotType.custom,
                    value: 'vert',
                },
                range: {
                    start: 7,
                    end: 11
                },
                entity: 'Color',
                slotName: 'Color'
            }],
            alternatives: []
        }

        const ordinalAndNumberIntentMessage: IntentMessage = {
            sessionId: '6ce651f7-0aec-4910-bfec-b246ea6ca550',
            customData: 'data',
            siteId: 'default',
            input: 'additionne un plus un',
            intent: {
                intentName: 'jelb:getAddition',
                confidenceScore: 0.5
            },
            asrConfidence: 1,
            asrTokens: [],
            slots: [
                {
                    confidenceScore: 0.5,
                    rawValue: 'un',
                    value: {
                        value: 1,
                        kind: Dialog.enums.slotType.number
                    },
                    alternatives: [],
                    range:{
                        start: 11,
                        end: 13
                    },
                    entity: 'snips/number',
                    slotName: 'firstTerm'
                },
                {
                    confidenceScore: 0.5,
                    rawValue: 'un',
                    value: {
                        value: 1,
                        kind: Dialog.enums.slotType.number
                    },
                    range: {
                        start: 19,
                        end: 21
                    },
                    entity: 'snips/number',
                    slotName: 'secondTerm'
                },
                {
                    confidenceScore: 0.5,
                    rawValue: 'un',
                    value: {
                        value: 101,
                        kind: Dialog.enums.slotType.ordinal
                    },
                    range: {
                        start: 19,
                        end: 21
                    },
                    entity: 'snips/ordinal',
                    slotName: 'thirdTerm'
                }
            ],
            alternatives: [
                {
                    intentName: 'alternativeIntent',
                    confidenceScore: 0.5,
                    slots: [
                        {
                            confidenceScore: 0.5,
                            rawValue: 'un',
                            value: {
                                value: 101,
                                kind: Dialog.enums.slotType.ordinal,
                            },
                            range: {
                                start: 19,
                                end: 21
                            },
                            entity: 'snips/ordinal',
                            slotName: 'thirdTerm'
                        }
                    ]
                }
            ]
        }

        const instantTimeIntentMessage: IntentMessage = {
            sessionId: 'fad16235-2b00-48fb-8684-d729284686f5',
            customData: '',
            siteId: 'default',
            input: 'what will be the weather in parish this sunday',
            asrConfidence: 1,
            asrTokens: [],
            intent: {
                intentName: 'davidsnips:WeatherForecast',
                confidenceScore: 0.5
            },
            slots: [{
                confidenceScore: 1.0,
                rawValue: 'this sunday',
                value: {
                    kind: Dialog.enums.slotType.instantTime,
                    value: '2019-01-06 00:00:00 +01:00',
                    grain: Dialog.enums.grain.day,
                    precision: Dialog.enums.precision.exact
                },
                alternatives: [{
                    kind: Dialog.enums.slotType.instantTime,
                    value: '2019-01-07 00:00:00 +01:00',
                    grain: Dialog.enums.grain.day,
                    precision: Dialog.enums.precision.exact
                }],
                range: {
                    start: 35,
                    end: 46
                },
                entity: 'snips/datetime',
                slotName: 'forecast_datetime',
            }]
        }

        roundTrip({
            data: customIntentMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_intent_json'
        })


        roundTrip({
            data: ordinalAndNumberIntentMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_intent_json'
        })

        roundTrip({
            data: instantTimeIntentMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_intent_json'
        })
    })


    it('IntentNotRecognizedMessage(s)', () => {
        const intentNotRecognizedMessage: IntentNotRecognizedMessage = {
            siteId: 'default',
            sessionId: '6ce651f7-0aec-4910-bfec-b246ea6ca550',
            input: 'additionne un plus un',
            customData: '',
            confidenceScore: 0.5,
            alternatives: [{
                intentName: 'alternativeIntent',
                confidenceScore: 0.5,
                slots: []
            }]
        }
        roundTrip({
            data: intentNotRecognizedMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_intent_not_recognized_json'
        })
    })


    it('SessionStartedMessage', () => {
        const sessionStartedMessage: SessionStartedMessage = {
            sessionId: 'Session id',
            customData: 'Custom data',
            siteId: 'Site id',
            reactivatedFromSessionId: 'reactivated session id'
        }

        roundTrip({
            data: sessionStartedMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_session_started_json'
        })
    })

    it('SessionQueuedMessage', () => {
        const sessionQueuedMessage: SessionQueuedMessage = {
            sessionId: 'Session id',
            customData: 'Custom data',
            siteId: 'Site id'
        }

        roundTrip({
            data: sessionQueuedMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_session_queued_json'
        })
    })


    it('SessionEndedMessage', () => {
        const sessionEndedMessage: SessionEndedMessage = {
            sessionId: 'Session id',
            customData: 'Custom data',
            termination: {
                reason: Dialog.enums.terminationType.error,
                error: 'Error message'
            },
            siteId: 'Site id'
        }

        const sessionEndedMessageComponent: SessionEndedMessage = {
            sessionId: 'Session id',
            customData: 'Custom data',
            termination: {
                reason: Dialog.enums.terminationType.timeout,
                component: component.asr
            },
            siteId: 'Site id'
        }

        roundTrip({
            data: sessionEndedMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_session_ended_json'
        })

        roundTrip({
            data: sessionEndedMessageComponent,
            FFIFunctionName: 'hermes_ffi_test_round_trip_session_ended_json'
        })
    })


    it('InjectionRequestMessage', () => {
        const injectionRequestMessage: InjectionRequestMessage = {
            id: 'abcdef',
            crossLanguage: 'en',
            lexicon: {
                films : [
                    'The Wolf of Wall Street',
                    'The Lord of the Rings'
                ]
            },
            operations: [
                [
                    Injection.enums.injectionKind.add,
                    {
                        films : [
                            [ 'The Wolf of Wall Street', 1 ],
                            [ 'The Lord of the Rings', 1 ]
                        ]
                    }
                ]
            ]
        }

        roundTrip({
            data: injectionRequestMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_injection_request_json'
        })
    })

    it('RegisterSoundMessage', () => {
        const registerSoundMessage: RegisterSoundMessage = {
            soundId: 'sound id',
            wavSound: Buffer.from([0, 1, 2]).toString('base64')
        }

        roundTrip({
            data: registerSoundMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_register_sound_json'
        })
    })

    it('DialogueConfigureMessage', () => {
        const dialogueConfigureMessage: DialogueConfigureMessage = {
            intents: [
                {
                    enable: true,
                    intentId: 'intentId'
                }
            ],
            siteId: 'siteId'
        }

        roundTrip({
            data: dialogueConfigureMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_dialogue_configure_json'
        })
    })

    it('TextCapturedMessage', () => {
        const textCapturedMessage: TextCapturedMessage = {
            likelihood: 0.5,
            seconds: 2,
            sessionId: 'sessionId',
            siteId: 'default',
            text: 'hello',
            tokens: [
                {
                    confidence: 0.5,
                    rangeEnd: 5,
                    rangeStart: 0,
                    time: {
                        start: 0,
                        end: 2
                    },
                    value: 'hello'
                }
            ]
        }

        roundTrip({
            data: textCapturedMessage,
            FFIFunctionName: 'hermes_ffi_test_round_trip_text_captured_json'
        })
    })
})
