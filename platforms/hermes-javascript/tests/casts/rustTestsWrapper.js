const path = require('path')
const ffi = require('ffi')
const ref = require('ref')

module.exports.library = libraryPath => ffi.Library(libraryPath, {
    // Round trips
    hermes_ffi_test_round_trip_start_session: [ 'int', [ 'void *', 'void *' ]],
    hermes_ffi_test_round_trip_continue_session: [ 'int', [ 'void *', 'void *' ]],
    hermes_ffi_test_round_trip_end_session: [ 'int', [ 'void *', 'void *' ]],
    hermes_ffi_test_round_trip_intent_not_recognized: [ 'int', [ 'void *', 'void *' ]],
    hermes_ffi_test_round_trip_session_started: [ 'int', [ 'void *', 'void *' ]],
    hermes_ffi_test_round_trip_session_queued: [ 'int', [ 'void *', 'void *' ]],
    hermes_ffi_test_round_trip_session_ended: [ 'int', [ 'void *', 'void *' ]],
    hermes_ffi_test_round_trip_injection_request: [ 'int', [ 'void *', 'void *' ]],

    // Error handling
    hermes_ffi_test_get_last_error: [ 'int', [ 'char **' ]],
})

module.exports.call = function(libraryPath = path.resolve(__dirname, '../../../../target/release/libhermes_ffi_test')) {
    const library = module.exports.library(libraryPath)
    return function(funName, ...args) {
        const result = library[funName](...args)
        if(result === 0)
            return
        const errorRef = ref.alloc('char **')
        library['hermes_ffi_test_get_last_error'](errorRef)
        let errorMessage = 'Error while calling function ' + funName + '\n'
        errorMessage += errorRef.deref().readCString()
        throw new Error(errorMessage)
    }
}