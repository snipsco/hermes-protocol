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

    // Error handling
    hermes_ffi_test_get_last_error: [ 'int', [ 'char **' ]],
})

module.exports.call = function(libraryPath = path.resolve(__dirname, '../../libhermes_ffi_test')) {
    return function(funName, ...args) {
        const result = module.exports.library(libraryPath)[funName](...args)
        if(result === 0)
            return
        const errorRef = ref.alloc('char **')
        module.exports.library(libraryPath)['hermes_ffi_test_get_last_error'](errorRef)
        let errorMessage = 'Error while calling function ' + funName + '\n'
        errorMessage += errorRef.deref().readCString(0)
        throw new Error(errorMessage)
    }
}