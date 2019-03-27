import {
    IntentMessage,
    IntentNotRecognizedMessage,
    SessionStartedMessage,
} from './messages'

/**
 * An object exposing methods used to configure the current dialog round.
 */
export type FlowContinuation = {
    /**
     * Marks an intent as a possible dialogue continuation for the next dialogue round.
     *
     * @param intentName - The name of the intent.
     * @param action - Function that will be called if the intent gets matched.
     * @param options - Configure the continuation.
     * @param options.slotFiller - Specify a slot name and enables the slot filler that will try to match this slot.
     */
    continue: (intentName: string, action: FlowIntentAction, options?: { slotFiller: string | null }) => FlowActionReturn,
    /**
     * Enables a custom action if the intent is not recognized in the next dialogue round.
     *
     * @param action - Function that will be called if no intents are recognized.
     */
    notRecognized: (action: FlowNotRecognizedAction) => FlowActionReturn,
    /**
     * Marks the dialogue session as completed.
     */
    end: () => void
}

/**
 * An object returned by a dialogue round action function and that configures that next session message.
 */
export type FlowActionReturnData = {
    text?: string,
    customData?: string
}
/**
 * The full return type of a dialogue round action function.
 *
 * Either a full FlowActionReturnData object, a string for the TTS speech or nothing.
 * (or a Promise wrapping these types)
 */
export type FlowActionReturn =
    FlowActionReturnData | string | void |
    Promise<FlowActionReturnData | void | string>

/**
 * A callback for the current round of dialogue that will be run when receiving an intent.
 *
 * @param message - The intent message received this round.
 * @param flow - The object used to configure the next dialogue round.
 */
export type FlowIntentAction = (
    message: IntentMessage,
    flow: FlowContinuation
) => FlowActionReturn

/**
 * A callback for the current round of dialogue that will be run when no intents were matched.
 *
 * @param message - The message received this round.
 * @param flow - The object used to configure the next dialogue round.
 */
export type FlowNotRecognizedAction = (
    message: IntentNotRecognizedMessage,
    flow: FlowContinuation
) => FlowActionReturn

/**
 * A callback for the current round of dialogue that will be run when the session has just been started.
 *
 * @param message - The message received this round.
 * @param flow - The object used to configure the next dialogue round.
 */
export type FlowSessionAction = (
    message: SessionStartedMessage,
    flow: FlowContinuation
) => FlowActionReturn
