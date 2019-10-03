import {
    InjectionRequestMessage,
    InjectionStatusMessage,
    InjectionCompleteMessage,
    InjectionResetRequestMessage,
    InjectionResetCompleteMessage,
} from './messages'

export namespace InjectionTypes {

    /**
     * The name and type of message that the Injection subset can publish.
     */
    export type publishMessagesList = {
        injection_request: InjectionRequestMessage,
        injection_status_request: null,
        injection_reset_request: InjectionResetRequestMessage
    }

    /**
     * The name and type of message to which the Injection subset can subscribe.
     */
    export type subscribeMessagesList = {
        injection_status: InjectionStatusMessage,
        injection_complete: InjectionCompleteMessage,
        injection_reset_complete: InjectionResetCompleteMessage
    }
}
