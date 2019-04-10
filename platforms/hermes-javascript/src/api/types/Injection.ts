import {
    InjectionRequestMessage,
    InjectionStatusMessage,
} from './messages'

export namespace InjectionTypes {

    /**
     * The name and type of message that the Injection subset can publish.
     */
    export type publishMessagesList = {
        injection_request: InjectionRequestMessage,
        injection_status_request: null
    }

    /**
     * The name and type of message to which the Injection subset can subscribe.
     */
    export type subscribeMessagesList = {
        injection_status: InjectionStatusMessage
    }
}
