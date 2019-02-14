export namespace DialogTypes {

    /* Enums */

    export enum grain {
        year = 'Year',
        quarter = 'Quarter',
        month = 'Month',
        week = 'Week',
        day = 'Day',
        hour = 'Hour',
        minute = 'Minute',
        second = 'Second'
    }

    export enum precision {
        approximate = 'Approximate',
        exact = 'Exact'
    }

    export enum initType {
        action = 'action',
        notification = 'notification'
    }

    export enum terminationType {
        nominal = 'nominal',
        siteUnavailable = 'siteUnavailable',
        abortedByUser = 'abortedByUser',
        intentNotRecognized = 'intentNotRecognized',
        timeout = 'timeout',
        error = 'error'
    }

    export enum slotType {
        custom = 'Custom',
        number = 'Number',
        ordinal = 'Ordinal',
        instantTime = 'InstantTime',
        timeInterval = 'TimeInterval',
        amountOfMoney = 'AmountOfMoney',
        temperature = 'Temperature',
        duration = 'Duration',
        percentage = 'Percentage',
        musicAlbum = 'MusicAlbum',
        musicArtist = 'MusicArtist',
        musicTrack = 'MusicTrack'
    }

    // Legacy

    export enum grain_legacy {
        year = 0,
        quarter,
        month,
        week,
        day,
        hour,
        minute,
        second
    }

    export enum precision_legacy {
        approximate = 0,
        exact
    }

    export enum initType_legacy {
        action = 1,
        notification
    }

    export enum terminationType_legacy {
        nominal = 1,
        unavailable,
        abortedByUser,
        intentNotRecognized,
        timeout,
        error
    }

    export enum slotType_legacy {
        custom = 1,
        number,
        ordinal,
        instantTime,
        timeInterval,
        amountOfMoney,
        temperature,
        duration,
        percentage,
        musicAlbum,
        musicArtist,
        musicTrack
    }

    /* Messages */

    export type StartSessionMessage = {
        init: {
            type: initType,
            text?: string,
            intentFilter?: string[],
            canBeEnqueued?: boolean,
            sendIntentNotRecognized?: false
        },
        customData?: string,
        siteId?: string
    } | {
        session_init: {
            init_type: initType_legacy,
            value: string | {
                text?: string,
                intent_filter?: string[],
                can_be_enqueued?: boolean,
                send_intent_not_recognized?: false
            }
        },
        custom_data?: string,
        site_id?: string
    }

    export type ContinueSessionMessage = {
        sessionId: string,
        text?: string,
        intentFilter?: string[],
        customData?: string,
        sendIntentNotRecognized?: boolean
    } | {
        session_id: string,
        text?: string,
        intent_filter?: string[],
        custom_data?: string,
        send_intent_not_recognized?: boolean
    }

    export type EndSessionMessage = {
        sessionId: string,
        text?: string
    } | {
        session_id: string,
        text?: string
    }

    export type SessionStartedMessage = {
        sessionId: string,
        siteId: string
        customData?: string
    } & {
        session_id: string,
        site_id: string
        custom_data?: string
    }

    export type SessionEndedMessage = {
        sessionId: string,
        siteId: string,
        customData?: string,
        termination: {
            reason: DialogTypes.terminationType,
            error?: string
        }
    } & {
        session_id: string,
        site_id: string,
        custom_data?: string,
        termination: {
            termination_type: DialogTypes.terminationType_legacy,
            data?: string
        }
    }

    export type SessionQueuedMessage = {
        sessionId: string,
        siteId: string,
        customData?: string
    } & {
        session_id: string,
        site_id: string,
        custom_data?: string
    }

    export type IntentNotRecognizedMessage = {
        sessionId: string,
        siteId: string,
        input: string,
        confidence_score: number
        customData?: string,
    } & {
        session_id: string,
        site_id: string,
        input: string,
        confidence_score: number
        custom_data?: string,
    }

    export type IntentMessage = {
        sessionId: string,
        siteId: string,
        input: string,
        customData?: string,
        intent: {
            intentName: string,
            confidenceScore: number
        },
        asrTokens: [
            {
                value: string,
                confidence: number,
                rangeStart: number,
                rangeEnd: number,
                time: {
                    start: number,
                    end: number
                }
            }[]
        ],
        slots: {
            confidenceScore: number,
            rawValue: string,
            range: {
                start: number,
                end: number
            },
            entity: string,
            slotName: string,
            value: {
                kind: DialogTypes.slotType,
                // Wildcard
                value: any
            }
        }[]
    } & {
        session_id: string,
        custom_data?: string,
        site_id: string,
        input: string,
        intent: {
            intent_name: string,
            confidence_score: number
        },
        asr_tokens: [
            {
                value: string,
                confidence: number,
                range_start: number,
                range_end: number,
                time: {
                    start: number,
                    end: number
                }
            }[]
        ],
        slots: {
            confidence_score: number,
            raw_value: string,
            range_start: number,
            range_end: number
            entity: string,
            slot_name: string,
            value: {
                value_type: DialogTypes.slotType_legacy,
                // Wildcard
                value: any
            }
        }[]
    }

    export type publishMessagesList = {
        start_session: StartSessionMessage,
        continue_session: ContinueSessionMessage,
        end_session: EndSessionMessage
    }
    export type subscribeMessagesList = {
        intents: IntentMessage,
        intent_not_recognized: IntentNotRecognizedMessage,
        session_ended: SessionEndedMessage,
        session_queued: SessionQueuedMessage,
        session_started: SessionStartedMessage
    } & {
        // Workaround for intents that have a dynamic key
        [key: string]: IntentMessage
    }
}
