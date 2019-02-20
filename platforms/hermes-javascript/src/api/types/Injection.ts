import {
    InjectionRequestMessage,
    InjectionRequestMessageLegacy,
    InjectionStatusMessage,
    InjectionStatusMessageLegacy
} from './messages'
import { HermesAPI } from '.'

export namespace InjectionTypes {
    export type publishMessagesList<API extends HermesAPI = 'json'> = {
        injection_request: API extends 'json' ? InjectionRequestMessage : InjectionRequestMessageLegacy,
        injection_status_request: null
    }

    export type subscribeMessagesList<API extends HermesAPI = 'json'> = {
        injection_status: API extends 'json' ? InjectionStatusMessage : InjectionStatusMessageLegacy
    }
}
