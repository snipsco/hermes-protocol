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

    export type publishMessagesList = {
        start_session: StartSessionMessage,
        continue_session: ContinueSessionMessage,
        end_session: EndSessionMessage
    }
}
