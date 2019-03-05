import { initType } from '../enums'

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
