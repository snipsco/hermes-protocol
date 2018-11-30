const ref = require('ref')
const array = require('ref-array')
const Casteable = require('./Casteable')
const { cast } = require('../tools')
const types = require('../ffi/typedefs')

const doubleArrayType = array(ref.refType(types.CAsrTokenArray))
const arrayType = array(ref.refType(types.CAsrToken))

class AsrTokensDoubleArrayType extends Casteable {
    constructor(arg) {
        super({})
        if(arg instanceof Buffer) {
            if(arg.length <= 0) {
                this._array = null
                return
            }
            this._array = []
            const doubleArrayStruct = arg.deref()
            const doubleArray = new doubleArrayType(doubleArrayStruct.entries)

            for(let i = 0; i < doubleArrayStruct.count; i++) {
                const asrTokensArray = []
                const arrayStruct = doubleArray[i].deref()
                const array = new arrayType(arrayStruct.entries)
                for(let j = 0; j < arrayStruct.count; j++) {
                    const asrTokenStruct = array[j].deref()
                    const asrToken = cast(asrTokenStruct, {
                        time: cast
                    })
                    asrTokensArray.push(asrToken)
                }
                this._array.push(asrTokensArray)
            }
        } else {
            this._array = arg
        }
    }
    forge() {
        if(!this._array)
            return null

        // We use .buffer for array types, not .ref() !
        // We don't want the pointer, just the buffer containing the array itself.

        const forgeSimpleArray = array => new types.CAsrTokenArray({
            entries: new arrayType(array.map(asrToken =>
                new Casteable(asrToken).forge(types.CAsrToken, {
                    time: timeObj => new Casteable(timeObj).forge(types.CAsrDecodingDuration)
                }).ref()
            )).buffer,
            count: array.length
        }).ref()

        const forgeDoubleArray = array => new types.CAsrTokenDoubleArray({
            entries: new doubleArrayType(array.map(asrTokenArray =>
                forgeSimpleArray(asrTokenArray))).buffer,
            count: array.length
        }).ref()

        return forgeDoubleArray(this._array)
    }
}

module.exports = AsrTokensDoubleArrayType