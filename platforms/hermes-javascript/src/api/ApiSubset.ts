import ffi from 'ffi'
import ref from 'ref'
import {
    SubscribeEventDescriptor,
    PublishEventDescriptor,
    MessageListener,
    FFIFunctionCall,
    HermesOptions
} from './types'

/* Tools */

const getMetadata = function<T = (SubscribeEventDescriptor | PublishEventDescriptor)>(
    obj: { [key: string]: T },
    eventName: string | number | symbol
) : T {
    if(typeof eventName === 'symbol')
        throw new Error('Symbol not expected')
    let metadata = obj[eventName]
    if(!metadata) {
        const matchingEntry = Object
            .entries(obj)
            .find(([key]) => typeof eventName === 'string' && eventName.startsWith(key))
        if(matchingEntry) {
            metadata = matchingEntry[1]
        } else {
            throw new Error(eventName + ' is not a known event!')
        }
    }
    return metadata
}


/**
 * An abstract Hermes API subset.
 */
export default class ApiSubset {

    protected call: FFIFunctionCall
    public destroy() {}
    private listeners = new Map()
    protected options: HermesOptions
    protected facade: Buffer | null = null
    protected subscribeEvents: { [key: string]: SubscribeEventDescriptor } = {}
    public publishEvents: { [key: string]: PublishEventDescriptor} = {}
    public publishMessagesList: {[key: string]: any} = {}
    public subscribeMessagesList: {[key: string]: any} = {}

    protected constructor(protocolHandler: Buffer, call: FFIFunctionCall, options: HermesOptions, facadeName: string) {
        this.call = call
        this.options = options
        this.listeners = new Map()
        if(facadeName && protocolHandler) {
            const facadeRef = ref.alloc('void **')
            this.call(facadeName, protocolHandler, facadeRef)
            this.facade = facadeRef.deref()
        }
    }

    private makeSubscriptionCallback<T extends keyof this['subscribeMessagesList']>(eventName: T) {
        return ffi.Callback('void', [ ref.coerceType('string') ], (stringifiedJson: string) => {
            try {
                const message = JSON.parse(stringifiedJson)
                const actions = this.listeners.get(eventName)
                actions.forEach(action => action(message))
            } catch (err) {
                // eslint-disable-next-line
                console.error(err)
                throw err
            }
        })
    }

    /**
     * Subscribes a message listener to a given hermes event.
     *
     * @param eventName - The event name to subscribe to.
     * @param listener - A callback triggered when receiving a message.
     * @returns A reference to the listener.
     */
    public on<T extends keyof this['subscribeMessagesList']>(eventName: T, listener: MessageListener<this['subscribeMessagesList'][T]>) {
        const {
            fullEventName,
            additionalArguments
        } = getMetadata(this.subscribeEvents, eventName)
        let listeners = this.listeners.get(eventName)
        if(!listeners) {
            listeners = []
            this.listeners.set(eventName, listeners)
            const callback = this.makeSubscriptionCallback(eventName)
            const args = [
                ...(additionalArguments && additionalArguments(eventName as string) || []),
                callback
            ]
            // Prevent GC
            process.on('exit', function() { callback })
            this.call(fullEventName, this.facade, ...args)
        }
        listeners.push(listener)
        return listener
    }

    /**
     * Add a message listener that will only get called **once** for a given hermes event, then unsubscribe.
     *
     * @param eventName - The event name to subscribe to.
     * @param listener - A callback triggered when receiving a message.
     * @returns A reference to the wrapped listener.
     */
    public once<T extends keyof this['subscribeMessagesList']>(eventName: T, listener: MessageListener<this['subscribeMessagesList'][T]>) {
        const listenerWrapper = (message: this['subscribeMessagesList'][T], ...args: any[]) => {
            this.off(eventName, listenerWrapper)
            listener(message, ...args)
        }
        this.on(eventName, listenerWrapper)
        return listenerWrapper
    }

    /**
     * Removes an existing message listener for a given hermes event.
     *
     * @param eventName - The event name that was subscribed to.
     * @param listener - A reference to the listener callback to remove.
     * @returns True if succeeded, false otherwise.
     */
    public off<T extends keyof this['subscribeMessagesList']>(eventName: T, listener: MessageListener<this['subscribeMessagesList'][T]>) {
        const listeners = this.listeners.get(eventName)
        if(!listeners)
            return false
        const index = listeners.indexOf(listener)
        if(index < 0)
            return false
        listeners.splice(index, 1)
        return true
    }

    /**
     * Publish a message.
     *
     * @param eventName - Name of the publishing event.
     * @param message - Contents of the message.
     */
    public publish<T extends keyof this['publishEvents']>(eventName: T, message?: this['publishMessagesList'][T]) {
        const { fullEventName } = getMetadata(this.publishEvents, eventName)

        if(message) {
            const cStringRef = ref.allocCString(JSON.stringify(message))
            this.call(fullEventName, this.facade, cStringRef)
        } else {
            this.call(fullEventName, this.facade)
        }
    }
}