import { initType, initType_legacy } from '../enums'

export type StartSessionMessage = {
    init: {
        type: initType,
        text?: string,
        intentFilter?: string[],
        canBeEnqueued?: boolean,
        sendIntentNotRecognized?: boolean
    },
    customData?: string,
    siteId?: string
}

export type StartSessionMessageLegacy = {
    session_init: {
        init_type: initType_legacy,
        value: string | {
            text?: string,
            intent_filter?: string[],
            can_be_enqueued?: boolean,
            send_intent_not_recognized?: boolean
        }
    },
    custom_data?: string,
    site_id?: string
}
