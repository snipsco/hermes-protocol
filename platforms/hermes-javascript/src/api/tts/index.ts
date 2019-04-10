import ApiSubset from '../ApiSubset'
import {
    FFIFunctionCall, HermesOptions, TtsTypes
} from '../types'

/**
 * The Tts API subset.
 */
export default class Tts extends ApiSubset {

    constructor(protocolHandler: Buffer, call: FFIFunctionCall, options: HermesOptions) {
        super(protocolHandler, call, options, 'hermes_protocol_handler_tts_facade')
    }

    publishEvents = {
        register_sound: {
            fullEventName: 'hermes_tts_publish_register_sound_json'
        }
    }
    publishMessagesList: TtsTypes.publishMessagesList = undefined as any

    destroy () {
        this.call('hermes_drop_tts_facade', this.facade)
    }
}