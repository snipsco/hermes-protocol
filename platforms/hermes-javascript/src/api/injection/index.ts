import ApiSubset from '../ApiSubset'
import {
    InjectionTypes
} from '../types'
import * as enums from '../types/enums'

export default class Injection extends ApiSubset {

    constructor(protocolHandler, call, options) {
        super(protocolHandler, call, options, 'hermes_protocol_handler_injection_facade')
    }

    publishEvents = {
        injection_request: {
            fullEventName: 'hermes_injection_publish_injection_request_json'
        },
        injection_status_request: {
            fullEventName: 'hermes_injection_publish_injection_status_request_json'
        }
    }
    publishMessagesList: InjectionTypes.publishMessagesList

    subscribeEvents = {
        injection_status: {
            fullEventName: 'hermes_injection_subscribe_injection_status_json'
        }
    }
    subscribeMessagesList: InjectionTypes.subscribeMessagesList

    destroy () {
        this.call('hermes_drop_injection_facade', this.facade)
    }

    static enums = {
        injectionKind: enums.injectionKind
    }
}