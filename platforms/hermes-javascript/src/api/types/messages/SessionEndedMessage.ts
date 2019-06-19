import { terminationType, component } from '../enums'

export interface SessionEndedMessage {
    /** Id of the session that ended. */
    sessionId: string
    /** Id of the site where the session took place. */
    siteId: string
    /** Custom data provided in the StartSessionMessage or a ContinueSessionMessage. */
    customData?: string
    /** Information about the session termination. */
    termination: {
        /** Reason of the termination. */
        reason: terminationType,
        /** If there was an error, the error description. */
        error?: string
        /** If there was a timeout, the component that timeouted. */
        component?: component
    }
}
