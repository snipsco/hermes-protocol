const ref = require('ref')
const array = require('ref-array')
const Casteable = require('./Casteable')
const DoubleArray = require('./DoubleArray')
const StringArray = require('./StringArray')
const { cast } = require('../tools')
const {
    CInjectionRequestMessage,
    CMapStringToStringArray,
    CInjectionRequestOperations,
    CInjectionRequestOperation,
    CMapStringToStringArrayEntry
} = require('../ffi/typedefs')

const InjectionRequestOperationsArrayType = array(ref.refType(CInjectionRequestOperation))
const MapStringToStringArrayType = array(ref.refType(CMapStringToStringArrayEntry))

class MapToStringArray extends Casteable {
    constructor(arg){
        super()
        if(arg instanceof Buffer) {
            if(arg.length <= 0) {
                this._array = null
                return null
            }

            const unflattenedDictionary = new DoubleArray(arg, {
                itemArrayType: CMapStringToStringArray,
                refArrayType: MapStringToStringArrayType,
                cast: entry => (
                    cast(entry, {
                        value: value => new StringArray(value)._array
                    })
                )
            })._array

            this._object = unflattenedDictionary.reduce((dict, item) => {
                dict[item.key] = item.value
                return dict
            }, {})
        } else {
            this._object = arg
        }
    }

    forge() {
        const array = Object.entries(this._object).reduce((array, [key, value]) => {
            array.push({
                key,
                value
            })
            return array
        }, [])
        return new DoubleArray(array, {
            itemArrayType: CMapStringToStringArray,
            refArrayType: MapStringToStringArrayType,
            forge: entry => new Casteable(entry).forge(CMapStringToStringArrayEntry, {
                    value: value => new StringArray(value).forge()
            }).ref()
        }).forge()
    }
}

class InjectionRequestOperations extends Casteable {
    constructor(arg) {
        super()
        if(arg instanceof Buffer) {
            if(arg.length <= 0) {
                this._array = null
                return null
            }

            this._array = new DoubleArray(arg, {
                itemArrayType: CInjectionRequestOperations,
                refArrayType: InjectionRequestOperationsArrayType,
                entriesField: 'operations',
                cast: injectionRequestOperation => cast(injectionRequestOperation, {
                    values: values => new MapToStringArray(values)._object
                })
            })._array
        } else {
            this._array = arg
        }
    }

    forge() {
        return new DoubleArray(this._array, {
            itemArrayType: CInjectionRequestOperations,
            refArrayType: InjectionRequestOperationsArrayType,
            entriesField: 'operations',
            forge: injectionRequestOperation => (
                new Casteable(injectionRequestOperation).forge(CInjectionRequestOperation, {
                    values: values => new MapToStringArray(values).forge()
                }).ref()
            )
        }).forge()
    }
}

class InjectionRequestMessage extends Casteable {
    constructor(args) {
        super(args)
        this.type = CInjectionRequestMessage
    }

    fromBuffer(buffer) {
        return cast(buffer, {
            operations: operations => new InjectionRequestOperations(operations)._array,
            lexicon: lexicon => new MapToStringArray(lexicon || {})._object
        })
    }
    forge() {
        return super.forge(CInjectionRequestMessage, {
            operations: operations => operations && new InjectionRequestOperations(operations).forge(),
            lexicon: lexicon => new MapToStringArray(lexicon || {}).forge()
        })
    }
}

module.exports = InjectionRequestMessage
