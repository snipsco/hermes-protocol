import { terminationType } from '../enums'

export type SessionEndedMessage = {
    sessionId: string,
    siteId: string,
    customData?: string,
    termination: {
        reason: terminationType,
        error?: string
    }
}
