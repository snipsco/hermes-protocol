const ffi = require('ffi')
const ref = require('ref')
const { call } = require('../ffi/bindings')
const { Casteable } = require('../casts')

const getMetadata = function(obj, eventName) {
    let metadata = obj[eventName]
    if(!metadata) {
        const matchingEntry = Object
            .entries(obj)
            .find(([key]) => eventName.startsWith(key))
        if(matchingEntry) {
            metadata = matchingEntry[1]
        } else {
            throw new Error(eventName + ' is not a known event!')
        }
    }
    return metadata
}

class ApiSubset {

    constructor(options = {}, facadeName, protocolHandler) {
        this.call = call(options.libraryPath)
        this.listeners = new Map()
        if(facadeName && protocolHandler) {
            const facadeRef = ref.alloc('void **')
            this.call(facadeName, protocolHandler, facadeRef)
            this.facade = facadeRef.deref()
        }
    }

    /**
     * Subscribes a message listener to a given hermes event.
     *
     * @param {*} eventName The event name to subscribe to.
     * @param {*} listener  A callback triggered when receiving a message.
     */
    on(eventName, listener) {
        const {
            messageStruct,
            messageClass,
            dropEventName,
            fullEventName,
            additionalArguments
        } = getMetadata(this.subscribeEvents, eventName)

        let listeners = this.listeners.get(eventName)
        if(!listeners) {
            listeners = []
            this.listeners.set(eventName, listeners)
            const callback = ffi.Callback('void', [ ref.refType(messageStruct) ], data => {
                try {
                    const message = new (messageClass || Casteable)(data)
                    const actions = this.listeners.get(eventName)
                    actions.forEach(action => action(message))
                    this.call(dropEventName, data)
                } catch (err) {
                    // console.error('Error while executing callback for event: ' + fullEventName)
                    // console.error('message:', err.message)
                    console.error(err)
                    throw err
                }
            })
            const args = [
                ...(additionalArguments && additionalArguments(eventName) || []),
                callback
            ]
            // Prevent GC
            process.on('exit', function() { callback })
            this.call(fullEventName, this.facade, ...args)
        }
        listeners.push(listener)
    }

    /**
     * Add a message listener that will only get called **once** for a given hermes event, then unsubscribe.
     * @param {*} eventName The event name to subscribe to.
     * @param {*} listener A callback triggered when receiving a message.
     * @returns {*} The reference to the wrapped listener.
     */

    once(eventName, listener) {
        const listenerWrapper = (...args) => {
            listener(...args)
            this.off(eventName, listenerWrapper)
        }
        this.on(eventName, listenerWrapper)
        return listenerWrapper
    }

    /**
     * Removes an existing message listener for a given hermes event.
     *
     * @param {*} eventName The event name that was subscribed to.
     * @param {*} listener The reference to the listener callback to remove.
     */
    off(eventName, listener) {
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
     */
    publish(eventName, message) {
        const {
            messageClass,
            fullEventName,
            forgedStruct,
            forgeOptions
        } = getMetadata(this.publishEvents, eventName)

        if(message) {
            const cDataRef = new (messageClass || Casteable)(message).forge(forgedStruct, forgeOptions).ref()
            this.call(fullEventName, this.facade, cDataRef)
        } else {
            this.call(fullEventName, this.facade)
        }
    }
}

ApiSubset.prototype.subscribeEvents = {}
ApiSubset.prototype.publishEvents = {}

module.exports = ApiSubset