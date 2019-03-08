import ApiSubset from '../ApiSubset'
import { FeedbackTypes, FFIFunctionCall, HermesOptions } from '../types'
export default class Feedback extends ApiSubset {

    constructor(protocolHandler: Buffer, call: FFIFunctionCall, options: HermesOptions) {
        super(protocolHandler, call, options, 'hermes_protocol_handler_sound_feedback_facade')
    }

    publishEvents = {
        notification_on: {
            fullEventName: 'hermes_sound_feedback_publish_toggle_on_json'
        },
        notification_off: {
            fullEventName: 'hermes_sound_feedback_publish_toggle_off_json'
        }
    }
    publishMessagesList: FeedbackTypes.publishMessagesList = undefined as any

    destroy () {
        this.call('hermes_drop_sound_feedback_facade', this.facade)
    }
}