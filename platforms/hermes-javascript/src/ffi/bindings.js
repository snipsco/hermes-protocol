const path = require('path')
const ffi = require('ffi')
const ref = require('ref')

/*****************
   FFI Bindings
 *****************/

module.exports.library = libraryPath => ffi.Library(libraryPath, {

    /* Global */

    hermes_protocol_handler_new_mqtt: [ 'int', [ 'void **', 'string' ]],
    hermes_destroy_mqtt_protocol_handler: [ 'int', [ 'void *' ]],

    /* Utils */

    hermes_enable_debug_logs: [ 'int', []],
    hermes_get_last_error: [ 'int', [ 'char **' ]],

    /* Dialogue */

    // Allocators & destructors

    hermes_protocol_handler_dialogue_facade: [ 'int', [ 'void *', 'void **' ]],
    hermes_drop_dialogue_facade: [ 'int', [ 'void *' ]],

    hermes_drop_continue_session_message: [ 'int', [ 'void *' ]],
    hermes_drop_end_session_message: [ 'int', [ 'void *' ]],
    hermes_drop_start_session_message: [ 'int', [ 'void *' ]],
    hermes_drop_intent_message: [ 'int', [ 'void *' ]],
    hermes_drop_session_ended_message: [ 'int', [ 'void *' ]],
    hermes_drop_session_queued_message: [ 'int', [ 'void *' ]],
    hermes_drop_session_started_message: [ 'int', [ 'void *' ]],
    hermes_drop_intent_not_recognized_message: [ 'int', [ 'void *' ]],

    // Backend API - DO NOT EXPOSE - NO RUST ADAPTER YET

    // hermes_dialogue_backend_publish_intent: [ 'int', [ 'void *', 'void *' ]],
    // hermes_dialogue_backend_publish_session_ended: [ 'int', [ 'void *', 'void *' ]],
    // hermes_dialogue_backend_publish_session_queued: [ 'int', [ 'void *', 'void *' ]],
    // hermes_dialogue_backend_publish_session_started: [ 'int', [ 'void *', 'void *' ]],
    // hermes_dialogue_backend_subscribe_continue_session: [ 'int', [ 'void *', 'void *' ]],
    // hermes_dialogue_backend_subscribe_end_session: [ 'int', [ 'void *', 'void *' ]],
    // hermes_dialogue_backend_subscribe_start_session: [ 'int', [ 'void *', 'void *' ]],

    // Resumes the current session
    hermes_dialogue_publish_continue_session: [ 'int', [ 'void *', 'void *' ]],
    // Ends the current session
    hermes_dialogue_publish_end_session: [ 'int', [ 'void *', 'void *' ]],
    // Programmatically start a new session
    hermes_dialogue_publish_start_session: [ 'int', [ 'void *', 'void *' ]],
    // Callback - Subscribe to intents detected
    hermes_dialogue_subscribe_intent: [ 'int', [ 'void *', 'char *', 'void *' ]],
    hermes_dialogue_subscribe_intents: [ 'int', [ 'void *', 'void *' ]],
    hermes_dialogue_subscribe_intent_not_recognized: [ 'int', [ 'void *', 'void *' ]],
    // Callback - session ended
    hermes_dialogue_subscribe_session_ended: [ 'int', [ 'void *', 'void *' ]],
    // Callback - triggered when the current session in put in the queue
    hermes_dialogue_subscribe_session_queued: [ 'int', [ 'void *', 'void *' ]],
    // Callback - hotword or custom message
    hermes_dialogue_subscribe_session_started: [ 'int', [ 'void *', 'void *' ]],

    /* Injection */

    // Allocators & destructors
    hermes_protocol_handler_injection_facade: [ 'int', [ 'void *', 'void **' ]],
    hermes_drop_injection_facade: [ 'int', [ 'void *' ]],

    // Requests an injection
    hermes_injection_publish_injection_request: [ 'int', [ 'void *', 'void * ']],

    /* Feedback */

    // Allocators & destructors
    hermes_protocol_handler_sound_feedback_facade: [ 'int', [ 'void *', 'void **' ]],
    hermes_drop_sound_feedback_facade: [ 'int', [ 'void *' ]],

    // Turn on / off notification sounds
    hermes_sound_feedback_publish_toggle_on: [ 'int', [ 'void *', 'void *' ]],
    hermes_sound_feedback_publish_toggle_off: [ 'int', [ 'void *', 'void *' ]],

    /* Others */

    // hermes_protocol_handler_tts_backend_facade: [ 'int', [ 'void *', 'void **' ]],
    // hermes_tts_backend_subscribe_say: [ 'int', [ 'void *', 'void *' ]]
})

/**
 * An FFI function call wrapper that throws & returns with the
 * proper error message if an error code is returned by hermes.
 *
 * @param {*} libraryPath
 */
module.exports.call = function(libraryPath = path.resolve(__dirname, '../../libhermes_mqtt_ffi')) {
    const library = module.exports.library(libraryPath)
    return function(funName, ...args) {
        const result = library[funName](...args)
        if(result === 0)
            return
        const errorRef = ref.alloc('char **')
        library['hermes_get_last_error'](errorRef)
        let errorMessage = 'Error while calling function ' + funName + '\n'
        errorMessage += errorRef.deref().readCString()
        throw new Error(errorMessage)
    }
}