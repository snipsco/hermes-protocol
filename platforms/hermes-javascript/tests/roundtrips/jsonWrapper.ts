import path from 'path'
import ffi from 'ffi'
import ref from 'ref'
import { LIB_ENV_FOLDER } from '../constants'

export const library = libraryPath => ffi.Library(libraryPath, {
    // Round trips
    hermes_ffi_test_round_trip_start_session_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_continue_session_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_end_session_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_intent_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_intent_not_recognized_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_session_started_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_session_queued_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_session_ended_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_injection_request_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_register_sound_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_dialogue_configure_json: [ 'int', [ 'string', 'char **' ]],
    hermes_ffi_test_round_trip_text_captured_json: [ 'int', [ 'string', 'char **' ]],

    // Error handling
    hermes_ffi_test_get_last_error: [ 'int', [ 'char **' ]],
})

export const call = function(
    libraryPath = path.join(__dirname, `../../../../target/${LIB_ENV_FOLDER}/libhermes_ffi_test`)
) {
    const library = module.exports.library(libraryPath)
    return function(funName, ...args) {
        const result = library[funName](...args)
        if(result === 0)
            return
        const errorRef = ref.alloc('char **')
        library['hermes_ffi_test_get_last_error'](errorRef)
        let errorMessage = 'Error while calling function ' + funName + '\n'
        errorMessage += (errorRef as any).deref().readCString()
        throw new Error(errorMessage)
    }
}
