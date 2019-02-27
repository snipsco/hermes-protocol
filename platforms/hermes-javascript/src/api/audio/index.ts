import ref from 'ref'
import ApiSubset from '../ApiSubset'
import {
    AudioTypes
} from '../types'

/**
 * @experimental
 *
 * Warning: Experimental, use at your own risk!
 */
export default class Audio extends ApiSubset {

    constructor(protocolHandler, call, options) {
        super(protocolHandler, call, options, 'hermes_protocol_handler_audio_server_facade')
    }

    publishEvents = {
        play_audio: {
            fullEventName: 'hermes_audio_server_publish_play_bytes_json'
        }
    }
    publishMessagesList: AudioTypes.publishMessagesList

    subscribeEvents = {
        'play_finished/': {
            fullEventName: 'hermes_audio_server_subscribe_play_finished_json',
            additionalArguments: eventName => [
                ref.allocCString(eventName.substring(14))
            ]
        },
        play_finished_all: {
            fullEventName: 'hermes_audio_server_subscribe_all_play_finished_json'
        },
    }
    subscribeMessagesList: AudioTypes.subscribeMessagesList

    destroy () {
        this.call('hermes_drop_audio_server_facade', this.facade)
    }
}
