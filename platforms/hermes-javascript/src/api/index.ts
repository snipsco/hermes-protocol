import ref from 'ref'
import Dialog from './dialog'
import Injection from './injection'
import Feedback from './feedback'
import Audio from './audio'
import Tts from './Tts'
import { MqttOptions } from '../casts'
import { call } from '../ffi/bindings'
import ApiSubset from './ApiSubset'
import { HermesOptions, FFIFunctionCall, SubsetConstructor } from './types'

/* Types */

export { Dialog, Injection, Feedback, Audio, Tts }
export { ApiSubset }
export * from './types'

/**
 * Hermes javascript is an high level API that allows you to
 * subscribe and send Snips messages using the Hermes protocol.
 */
export class Hermes {

    /**
     * Create a new Hermes instance that connects to the underlying event bus.
     * @param {*} options The Hermes options object. *(default: {})*
     * @param {*} options.address The bus address *(default localhost:1883)*
     * @param {*} options.logs Enables or Disables stdout logs *(default false)*
     * @param {*} options.libraryPath A custom path for the dynamic Hermes ffi library
     * @param {*} options.username Username used when connecting to the broker.
     * @param {*} options.password Password used when connecting to the broker.
     * @param {*} options.tls_hostname Hostname to use for the TLS configuration. If set, enables TLS.
     * @param {*} options.tls_ca_file CA files to use if TLS is enabled.
     * @param {*} options.tls_ca_path CA paths to use if TLS is enabled.
     * @param {*} options.tls_client_key Client key to use if TLS is enabled.
     * @param {*} options.tls_client_cert Client cert to use if TLS is enabled.
     * @param {*} options.tls_disable_root_store Boolean indicating if the root store should be disabled if TLS is enabled.
     */
    constructor(options: HermesOptions = {}) {

        // Initialize a new Hermes instance.
        this.options = {
            ...Hermes.defaultOptions,
            ...options
        }
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
    }

    /**
     * Return a Dialog instance used to interact with the dialog API.
     */
    dialog() {
        return this._getOrCreateSubset<Dialog>('dialog', Dialog)
    }
    /**
     * Return an Injection instance used to interact with the vocabulary injection API.
     */
    injection() {
        return this._getOrCreateSubset<Injection>('injection', Injection)
    }
    /**
     * Return a Feedback object instance used to interact with the audio feedback API.
     */
    feedback() {
        return this._getOrCreateSubset<Feedback>('feedback', Feedback)
    }
    /**
     * Return a Tts object instance used to interact with the text to speech API.
     */
    tts() {
        return this._getOrCreateSubset<Tts>('tts', Tts)
    }

    /**
     * @experimental
     *
     * Warning: Experimental, use at your own risk!
     *
     * Returns an Audio object instance used to interact with the audio playback API.
     */
    audio() {
        return this._getOrCreateSubset<Audio>('audio', Audio)
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


    // Private //

    private _getOrCreateSubset<T extends ApiSubset>(key: string, Class: SubsetConstructor<T>): T {
        if(!this.activeSubsets.has(key)) {
            this.activeSubsets.set(key, new Class(this.protocolHandler, this.call, this.options))
        }
        return this.activeSubsets.get(key)
    }

    private static defaultOptions = {
        address: 'localhost:1883',
        logs: false
    }

    private options : HermesOptions
    private activeSubsets: Map<string, any> = new Map()
    private call: FFIFunctionCall
    private protocolHandler: Buffer
}