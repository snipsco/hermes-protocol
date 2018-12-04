let _keepAlive = true
const keepAliveFunction = function (timer) {
    if(_keepAlive) {
        module.exports.keepAlive(timer)
    }
}

module.exports = {
    /**
     * Prevents the process from terminating.
     */
    keepAlive: function (timer) {
        if(timer) {
            setTimeout(() => keepAliveFunction(timer), timer)
        } else {
            setImmediate(() => keepAliveFunction(timer))
        }
    },
    /**
     * Stops the keepAlive loop.
     */
    killKeepAlive: () => { _keepAlive = false },
    /**
     * Generic C struct to JS object casting method.
     */
    cast: (struct, customKeysCasting = {}) => {

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
                    obj[key] = customKeysCasting[key](value)
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
                    obj[key] = value.readCString(0)
                } else {
                    obj[key] = value.deref()
                }
            } catch (error) {
                console.error(error)
                obj[key] = null
            }

            // console.log(key, ' -> ', obj[key])
        }
        return obj
    }
}