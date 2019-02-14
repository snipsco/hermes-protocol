import ApiSubset from '../ApiSubset'
import {
    InjectionRequestMessage
} from '../../casts'
import {
    CInjectionRequestMessage,
    CInjectionStatusMessage
} from '../../ffi/typedefs'
import {
    InjectionTypes
} from '../types'

export default class Injection extends ApiSubset {

    constructor(protocolHandler, call, options) {
        super(protocolHandler, call, options, 'hermes_protocol_handler_injection_facade')
    }

    publishEvents = {
        injection_request: {
            fullEventName: 'hermes_injection_publish_injection_request',
            messageClass: InjectionRequestMessage,
            forgedStruct: CInjectionRequestMessage
        },
        injection_status_request: {
            fullEventName: 'hermes_injection_publish_injection_status_request',
            forgedStruct: null
        }
    }
    publishMessagesList: InjectionTypes.publishMessagesList

    subscribeEvents = {
        injection_status: {
            fullEventName: 'hermes_injection_subscribe_injection_status',
            dropEventName: 'hermes_drop_injection_status_message',
            messageStruct: CInjectionStatusMessage
        }
    }
    subscribeMessagesList: InjectionTypes.subscribeMessagesList

    destroy () {
        this.call('hermes_drop_injection_facade', this.facade)
    }

    static enums = {
        injectionKind: InjectionTypes.injectionKind,
        legacy: {
            injectionKind: InjectionTypes.injectionKind_legacy
        }
    }
}