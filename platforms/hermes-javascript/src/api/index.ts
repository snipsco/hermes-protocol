import ref from 'ref'
import Dialog from './dialog'
import Injection from './injection'
import Feedback from './feedback'
import Audio from './audio'
import Tts from './tts'
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
 *
 * **Important: do not instanciate this class more than once!**
 */
export class Hermes {

    /**
     * Create a new Hermes instance that connects to the underlying event bus.
     *
     * **Important: Each call to this function will open the hermes shared library
     * and bind hermes-javascript to it. It is an expensive operation.**
     *
     * @param options - Options used to instanciate a new Hermes object.
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
     *
     * @returns A Dialog instance, reused from a previous call if possible.
     */
    dialog() {
        return this._getOrCreateSubset<Dialog>('dialog', Dialog)
    }
    /**
     * Return an Injection instance used to interact with the vocabulary injection API.
     *
     * @returns An Injection instance, reused from a previous call if possible.
     */
    injection() {
        return this._getOrCreateSubset<Injection>('injection', Injection)
    }
    /**
     * Return a Feedback object instance used to interact with the audio feedback API.
     *
     * @returns An Feedback instance, reused from a previous call if possible.
     */
    feedback() {
        return this._getOrCreateSubset<Feedback>('feedback', Feedback)
    }
    /**
     * Return a Tts object instance used to interact with the text to speech API.
     *
     * @returns An Tts instance, reused from a previous call if possible.
     */
    tts() {
        return this._getOrCreateSubset<Tts>('tts', Tts)
    }

    /**
     * @experimental
     *
     * **Warning: Experimental, use at your own risk!**
     *
     * Returns an Audio object instance used to interact with the audio playback API.
     *
     * @returns An Audio instance, reused from a previous call if possible.
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