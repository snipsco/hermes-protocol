const ref = require('ref')
const array = require('ref-array')
const Casteable = require('./Casteable')
const { CStringArray } = require('../ffi/typedefs')

const StringArrayType = array(ref.types.CString)

class StringArray extends Casteable {
    constructor(arg) {
        super({})
        if(arg instanceof Buffer) {
            this._array = []
            const cArray = ref.get(arg)
            const stringArray = new StringArrayType(cArray.data)
            for(let i = 0; i < cArray.size; i++) {
                this._array.push(stringArray[i])
            }
        } else {
            this._array = arg
        }
    }
    forge() {
        return new CStringArray({
            data: new StringArrayType(this._array).buffer,
            size: this._array.length
        }).ref()
    }
}

module.exports = StringArray