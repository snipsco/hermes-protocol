const ref = require('ref')
const Dialog = require('./dialog')
const Injection = require('./injection')
const { call } = require('../ffi/bindings')

/**
 * List of API subsets that will be used to auto-generate getters.
 */
const API_SUBSETS = {
    dialog: Dialog,
    injection: Injection,
}

/**
 * Hermes javascript is an high level API that allows you to
 * subscribe and send Snips messages using the Hermes protocol.
 */
class Hermes {

    /**
     * Create a new Hermes instance that connects to the underlying event bus.
     * @param {*} options.address The bus address *(default localhost:1883)*
     * @param {*} options.logs Enables or Disables stdout logs *(default true)*
     * @param {*} options.libraryPath A custom path for the dynamic hermes ffi library
     */
    constructor(options = {}) {

        // Instance init.
        this.options = {
            ...Hermes.defaultOptions,
            ...options
        }
        this.listeners = new Map()
        this.activeSubsets = new Map()
        this.call = call(this.options.libraryPath)

        // Allocate the ProtocolHandler
        const protocolHandlerRef = ref.alloc('void **')
        this.call('hermes_protocol_handler_new_mqtt', protocolHandlerRef, this.options.address)
        this.protocolHandler = protocolHandlerRef.deref()

        // Enable logs if needed
        if(this.options.logs) {
            this.call('hermes_enable_debug_logs')
        }

        /**
         * Exposes public methods to get the subset api instances.
         */
        Object.entries(API_SUBSETS).forEach(([key, Class]) => {
            this[key] = () => {
                if(!this.activeSubsets.has(key)) {
                    this.activeSubsets.set(key, new Class(this.protocolHandler, this.options))
                }
                return this.activeSubsets.get(key)
            }
        })
    }

    /**
     * Disposes the hermes object and its underlying resources.
     */
    destroy() {
        this.activeSubsets.forEach(value => {
            value.destroy()
        })
        this.call('hermes_destroy_mqtt_protocol_handler', this.protocolHandler)
    }
}

Hermes.defaultOptions = {
    address: 'localhost:1883',
    logs: false
}

module.exports = Hermes