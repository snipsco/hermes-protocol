package ai.snips.hermes.ffi

import ai.snips.hermes.IntentMessage
import ai.snips.hermes.IntentNotRecognizedMessage
import ai.snips.queries.ontology.ffi.CIntentClassifierResult
import ai.snips.queries.ontology.ffi.CSlots
import ai.snips.queries.ontology.ffi.readString
import com.sun.jna.Pointer
import com.sun.jna.Structure
import com.sun.jna.toJnaPointer


class CIntentMessage(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField
    var input: Pointer? = null
    @JvmField
    var intent: CIntentClassifierResult? = null
    @JvmField
    var slots: CSlots? = null

    override fun getFieldOrder() = listOf("input", "intent", "slots")

    fun toIntentParserResult() = IntentMessage(
            input = input.readString(),
            intent = intent?.toIntentClassifierResult(),
            slots = slots?.toSlotList() ?: listOf())
}

class CIntentNotRecognizedMessage(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField
    var input: Pointer? = null
    @JvmField
    var id: Pointer? = null

    override fun getFieldOrder() = listOf("input", "id")

    fun toIntentNotRecognizedMessage() = IntentNotRecognizedMessage(input = input.readString(), id = id?.readString())
}
