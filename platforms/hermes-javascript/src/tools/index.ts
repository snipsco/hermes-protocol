import Int64 from 'node-int64'

 /**
 * Prevents the process from terminating.
 */
export function keepAlive(timer = 20) {
    return setInterval(() => {}, timer)
}

/**
 * Stops the keepAlive loop.
 */
export function killKeepAlive(keepAliveRef: NodeJS.Timeout) {
    clearInterval(keepAliveRef)
}

/**
 * Generic C struct to JS object casting method.
 */
export function cast(struct, customKeysCasting = {}) {

    // console.log('before ', struct)

    if(struct instanceof Buffer) {
        struct = struct.deref()
    }
    const obj = { ...struct.toObject() }

    // console.log('after ', obj)

    const keys = Object.keys(obj)
    for(let key of keys) {

        // console.log('key: ', key)

        const value = obj[key]

        try {

            const ref = value && value.ref && value.ref()
            const valueType = ref && ref.type.name

            if(value instanceof Buffer && value.isNull() || ref && ref.isNull()) {
                obj[key] = null
                continue
            }

            if(customKeysCasting[key]) {
                obj[key] = customKeysCasting[key](value, struct)
                continue
            }

            // console.log('value: ', value)
            // console.log('valueType:', valueType)

            if(!ref) {
                continue
            } else if(valueType === 'StructType*' || valueType === 'StructType') {
                // console.log('beforeStructTypeCall ', key, ' > ', valueType)
                obj[key] = module.exports.cast(value)
            } else if(valueType === 'char*' || valueType === 'string') {
                obj[key] = value.readCString()
            } else if(valueType === 'int64') {
                obj[key] = new Int64(value)
            } else {
                obj[key] = value.deref()
            }
        } catch (error) {
            // eslint-disable-next-line
            console.error(error)
            obj[key] = null
        }

        // console.log(key, ' -> ', obj[key])
    }
    return obj
}