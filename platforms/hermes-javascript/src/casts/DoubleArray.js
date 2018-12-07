const ref = require('ref')
const Casteable = require('./Casteable')

class DoubleArray extends Casteable {
    constructor(arg, properties) {
        super()

        if(!properties) {
            throw new Error('DoubleArray constructor expects a properties argument.')
        }

        const { itemArrayType, refArrayType, cast, forge, countField = 'count', entriesField = 'entries' } = properties
        this.properties = { itemArrayType, refArrayType, cast, forge, countField, entriesField }
        this._itemArrayType = itemArrayType

        if(arg instanceof Buffer) {
            if(arg.length <= 0) {
                this._array = null
                return
            }
            this._array = []
            const doubleArrayStruct = arg.deref()
            const count = doubleArrayStruct[countField]
            const doubleArrayRef = ref.reinterpret(doubleArrayStruct[entriesField], ref.sizeof.pointer * count)
            const doubleArray = new refArrayType(doubleArrayRef)

            for(let i = 0; i < count; i++) {
                const item = doubleArray[i]
                this._array.push(cast(item.deref()))
            }
        } else {
            this._array = arg
        }
    }
    forge() {
        if(!this._array)
            return null

        const { refArrayType, forge, countField, entriesField } = this.properties

        const itemStruct = new this._itemArrayType({
            // .buffer, not .ref() ! We don't want the pointer, just the buffer containing the array.
            [entriesField]: new refArrayType(this._array.map(forge)).buffer,
            [countField]: this._array.length
        }).ref()

        return itemStruct
    }
}

module.exports = DoubleArray