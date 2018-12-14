const ref = require('ref')
const Dialog = require('./dialog')
const Injection = require('./injection')
const Feedback = require('./feedback')
const { MqttOptions } = require('../casts')
const { call } = require('../ffi/bindings')

/**
 * List of API subsets that will be used to auto-generate getters.
 */
const API_SUBSETS = {
    dialog: Dialog,
    injection: Injection,
    feedback: Feedback
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
     * @param {*} options.username Username used when connecting to the broker.
     * @param {*} options.password Password used when connecting to the broker.
     * @param {*} options.tls_hostname Hostname to use for the TLS configuration. If set, enables TLS.
     * @param {*} options.tls_ca_file CA files to use if TLS is enabled.
     * @param {*} options.tls_ca_path CA paths to use if TLS is enabled.
     * @param {*} options.tls_client_key Client key to use if TLS is enabled.
     * @param {*} options.tls_client_cert Client cert to use if TLS is enabled.
     * @param {*} options.tls_disable_root_store Boolean indicating if the root store should be disabled if TLS is enabled.
     */
    constructor(options = {}) {

        // Initialize a new Hermes instance.
        this.options = {
            ...Hermes.defaultOptions,
            ...options
        }
        this.listeners = new Map()
        this.activeSubsets = new Map()
        this.call = call(this.options.libraryPath)

        // Allocate the ProtocolHandler double reference
        const protocolHandlerRef = ref.alloc('void **')
        // Allocate mqtt broker options
        const mqttOptions = new MqttOptions({
            broker_address: this.options.address,
            username: this.options.username,
            password: this.options.password,
            tls_hostname: this.options.tls_hostname,
            tls_ca_file: this.options.tls_ca_file,
            tls_ca_path: this.options.tls_ca_path,
            tls_client_key: this.options.tls_client_key,
            tls_client_cert: this.options.tls_client_cert,
            tls_disable_root_store: this.options.tls_disable_root_store
        })
        const mqttOptionsStructPtr = mqttOptions.forge().ref()
        ref._attach(mqttOptionsStructPtr, this)
        // Connect to MQTT with the specified options
        this.call(
            'hermes_protocol_handler_new_mqtt_with_options',
            protocolHandlerRef,
            mqttOptionsStructPtr,
            ref.NULL_POINTER
        )
        this.protocolHandler = protocolHandlerRef.deref()

        // Extra API call to enable logs if needed
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
        this.activeSubsets.forEach(subset => {
            subset.destroy()
        })
        this.call('hermes_destroy_mqtt_protocol_handler', this.protocolHandler)
    }
}

Hermes.defaultOptions = {
    address: 'localhost:1883',
    logs: false
}

module.exports = Hermes