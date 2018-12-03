const Casteable = require('./Casteable')
const SlotsArray = require('./SlotsArray')
const AsrTokensDoubleArray = require('./AsrTokensDoubleArray')
const { cast } = require('../tools')
const types = require('../ffi/typedefs')

class IntentMessage extends Casteable {
    constructor(args) {
        super(args)
        this.type = types.CIntentMessage
    }

    fromBuffer(buffer) {
        return cast(buffer, {
            slots: slots => new SlotsArray(slots)._array,
            asr_tokens: asrTokens => new AsrTokensDoubleArray(asrTokens)._array
        })
    }
    forge() {
        return super.forge(this.type, {
            intent: intent => new Casteable(intent).forge(types.CIntentClassifierResult).ref(),
            slots: slots => new SlotsArray(slots).forge(),
            asr_tokens: asrTokens => new AsrTokensDoubleArray(asrTokens).forge()
        })
    }
}

module.exports = IntentMessage