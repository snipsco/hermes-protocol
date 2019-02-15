import { terminationType, terminationType_legacy } from '../enums'

export type SessionEndedMessage = {
    sessionId: string,
    siteId: string,
    customData?: string,
    termination: {
        reason: terminationType,
        error?: string
    }
}

export type SessionEndedMessageLegacy = {
    session_id: string,
    site_id: string,
    custom_data?: string,
    termination: {
        termination_type: terminationType_legacy,
        data?: string
    }
}
