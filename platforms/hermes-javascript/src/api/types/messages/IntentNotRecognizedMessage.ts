import { NluSlot } from './IntentMessage'

export interface IntentNotRecognizedMessage {
    /** The current session id. */
    sessionId: string
    /** The site where the user interaction took place. */
    siteId: string
    /** The user input that has generated this event. */
    input: string
    /** The level of confidence in the non-prediction. */
    confidenceScore: number
    /** Array of alternative nlu slots that have lower probability. */
    alternatives?: {
        /**
         * Nullable, name of the intent detected (null = no intent)
         */
        intentName?: string
        /**
         * Nullable
         */
        slots?: NluSlot[]
        /**
         * Between 0 and 1
         */
        confidenceScore: number
    }[],
    /** Custom data provided in the StartSessionMessage or a ContinueSessionMessage. */
    customData?: string
}
