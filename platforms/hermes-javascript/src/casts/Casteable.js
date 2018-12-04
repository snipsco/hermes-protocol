const { cast } = require('../tools')
const ref = require('ref')

/**
 * An abstract class equipped with generic methods
 * to serialize / deserialize javascript objects
 * and C structures.
 */
class Casteable {
    /**
     * Create a new Casteable class from either a Buffer containing a C structure or a plain old javascript object.
     *
     * @param {*} arg Object or Buffer
     * @param {*} customKeyCasting Keys that need a custom method when casting from a Buffer.
     */
    constructor(arg = {}, customKeyCasting) {
        // 'type' field: C type to cast the Casteable into
        Object.defineProperty(this, 'type', {
            enumerable: false,
            writable: true
        })
        if(arg instanceof Buffer) {
            // If we are creating the Casteable from a Buffer (C struct)
            Object.assign(this, this.fromBuffer(arg, customKeyCasting))
        } else {
            // If we are creating the Casteable from a JS object
            Object.assign(this, arg)
        }
    }

    fromBuffer(buffer, customKeyCasting) {
        return cast(buffer, customKeyCasting)
    }

    /**
     * Forge a C structure from this Casteable.
     *
     * @param {*} type C structure type
     * @param {*} specialFields Fields that need a custom method when casting.
     */
    forge(type, specialFields = {}) {
        if(!type) {
            type = this.type
        }
        const messageStruct = new type
        const keys = Object.keys(this)
        for(let key of keys) {
            const value = this[key]
            if(specialFields[key]) {
                // Custom casting method
                const transformedValue = specialFields[key] && specialFields[key](value)
                messageStruct[key] = transformedValue
            } else if(typeof value === 'string') {
                // Read the char* pointer until a 0x00 byte is found.
                messageStruct[key] = ref.allocCString(value)
            } else if(typeof value !== 'object') {
                // Primitive type.
                messageStruct[key] = value
            } else if (value === null || value === undefined) {
                messageStruct[key] = null
            } else {
                // This is an object, and we expected to have a special field entry for casting it.
                if(!specialFields[key]) {
                    const error = new Error(`Expected specialField entry for [${key}] property (type: ${type} / specialFields: ${JSON.stringify(specialFields)})`)
                    console.error(error)
                    console.error(error.stack)
                }
                messageStruct[key] = ref.NULL
            }
        }
        return messageStruct
    }
}

module.exports = Casteable