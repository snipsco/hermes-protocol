package ai.snips.hermes.ffi

import ai.snips.hermes.ContinueSessionMessage
import ai.snips.hermes.EndSessionMessage
import ai.snips.hermes.InjectionKind.Add
import ai.snips.hermes.InjectionOperation
import ai.snips.hermes.InjectionRequestMessage
import ai.snips.hermes.IntentMessage
import ai.snips.hermes.SayFinishedMessage
import ai.snips.hermes.SayMessage
import ai.snips.hermes.SessionEndedMessage
import ai.snips.hermes.SessionInit
import ai.snips.hermes.SessionInit.Action
import ai.snips.hermes.SessionInit.Notification
import ai.snips.hermes.SessionInit.Type
import ai.snips.hermes.SessionQueuedMessage
import ai.snips.hermes.SessionStartedMessage
import ai.snips.hermes.SessionTermination
import ai.snips.hermes.SessionTermination.AbortedByUser
import ai.snips.hermes.SessionTermination.Error
import ai.snips.hermes.SessionTermination.IntenNotRecognized
import ai.snips.hermes.SessionTermination.Nominal
import ai.snips.hermes.SessionTermination.SiteUnAvailable
import ai.snips.hermes.SessionTermination.Timeout
import ai.snips.hermes.StartSessionMessage
import ai.snips.nlu.ontology.ffi.CIntentClassifierResult
import ai.snips.nlu.ontology.ffi.CSlots
import ai.snips.nlu.ontology.ffi.readString
import ai.snips.nlu.ontology.ffi.toPointer
import com.sun.jna.Memory
import com.sun.jna.Pointer
import com.sun.jna.Structure

class CStringArray(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        fun fromStringList(list: List<String>) = CStringArray(null).apply {
            size = list.size
            data = if (size > 0)
                Memory(Pointer.SIZE * list.size.toLong()).apply {
                    list.forEachIndexed { i, s ->
                        this.setPointer(i.toLong() * Pointer.SIZE, s.toPointer())
                    }
                }
            else null
        }
    }

    @JvmField
    var data: Pointer? = null
    @JvmField
    var size: Int = -1

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("data", "size")

    fun toStringList() = if (size > 0) {
        data!!.getPointerArray(0, size).map { it.readString() }
    } else listOf<String>()
}

class CActionSessionInit(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        fun fromActionSessionInit(actionSessionInit: SessionInit.Action) = CActionSessionInit(null).apply {
            text = actionSessionInit.text?.toPointer()
            intent_filter = if (actionSessionInit.intentFilter.isEmpty()) null else CStringArray.fromStringList(actionSessionInit.intentFilter)
            can_be_enqueued = if (actionSessionInit.canBeEnqueued) 1 else 0
        }
    }

    @JvmField
    var text: Pointer? = null
    @JvmField
    var intent_filter: CStringArray? = null
    @JvmField
    var can_be_enqueued: Byte = -1

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("text", "intent_filter", "can_be_enqueued")

    fun toSessionInit() = SessionInit.Action(
            text = text?.readString(),
            intentFilter = intent_filter?.toStringList() ?: listOf(),
            canBeEnqueued = can_be_enqueued == 1.toByte()
    )
}

class CSessionInit : Structure(), Structure.ByValue {
    companion object {
        const val ACTION = 1
        const val NOTIFICATION = 2

        fun fromSessionInit(sessionInit: SessionInit) = CSessionInit().apply {
            when (sessionInit.type) {
                Type.ACTION -> {
                    init_type = ACTION
                    value = CActionSessionInit.fromActionSessionInit(sessionInit as Action).apply { write() }.pointer
                }
                Type.NOTIFICATION -> {
                    init_type = NOTIFICATION
                    value = (sessionInit as Notification).text.toPointer()
                }
            }
        }
    }

    @JvmField
    var init_type: Int? = null
    @JvmField
    var value: Pointer? = null

    override fun getFieldOrder() = listOf("init_type", "value")

    fun toSessionInit(): SessionInit = when (init_type) {
        ACTION -> CActionSessionInit(value!!).toSessionInit()
        NOTIFICATION -> SessionInit.Notification(text = value.readString())
        else -> throw IllegalArgumentException("unknown value type $init_type")
    }
}

class CStartSessionMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        fun fromStartSessionMessage(message: StartSessionMessage) = CStartSessionMessage(null).apply {
            init = CSessionInit.fromSessionInit(message.init)
            custom_data = message.customData?.toPointer()
            site_id = message.siteId?.toPointer()
        }
    }

    @JvmField
    var init: CSessionInit? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var site_id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("init", "custom_data", "site_id")

    fun toStartSessionMessage() = StartSessionMessage(
            init = init!!.toSessionInit(),
            customData = custom_data?.readString(),
            siteId = site_id?.readString()
    )
}

class CContinueSessionMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        fun fromContinueSessionMessage(continueSessionMessage: ContinueSessionMessage) = CContinueSessionMessage(null).apply {
            session_id = continueSessionMessage.sessionId.toPointer()
            text = continueSessionMessage.text.toPointer()
            intent_filter = if (continueSessionMessage.intentFilter.isEmpty()) null else CStringArray.fromStringList(continueSessionMessage.intentFilter)
        }
    }

    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var text: Pointer? = null
    @JvmField
    var intent_filter: CStringArray? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("session_id", "text", "intent_filter")

    fun toContinueSessionMessage() = ContinueSessionMessage(
            sessionId = session_id.readString(),
            text = text.readString(),
            intentFilter = intent_filter?.toStringList() ?: listOf()
    )
}

class CEndSessionMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        fun fromEndSessionMessage(endSessionMessage: EndSessionMessage) = CEndSessionMessage(null).apply {
            session_id = endSessionMessage.sessionId.toPointer()
            text = endSessionMessage.text?.toPointer()
        }
    }

    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var text: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("session_id", "text")

    fun toEndSessionMessage() = EndSessionMessage(
            sessionId = session_id.readString(),
            text = text?.readString()
    )
}


class CIntentMessage(p: Pointer) : Structure(p), Structure.ByReference {
    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var site_id: Pointer? = null
    @JvmField
    var input: Pointer? = null
    @JvmField
    var intent: CIntentClassifierResult? = null
    @JvmField
    var slots: CSlots? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("session_id", "custom_data", "site_id", "input", "intent", "slots")

    fun toIntentMessage() = IntentMessage(
            sessionId = session_id.readString(),
            customData = custom_data?.readString(),
            siteId = site_id.readString(),
            input = input.readString(),
            intent = intent!!.toIntentClassifierResult(),
            slots = slots?.toSlotList() ?: listOf())
}

class CSessionStartedMessage(p: Pointer) : Structure(p), Structure.ByReference {
    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var site_id: Pointer? = null
    @JvmField
    var reactivated_from_session_id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("session_id", "custom_data", "site_id", "reactivated_from_session_id")

    fun toSessionStartedMessage() = SessionStartedMessage(
            sessionId = session_id.readString(),
            customData = custom_data?.readString(),
            siteId = site_id.readString(),
            reactivatedFromSessionId = reactivated_from_session_id?.readString())
}

class CSessionQueuedMessage(p: Pointer) : Structure(p), Structure.ByReference {
    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var site_id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("session_id", "custom_data", "site_id")

    fun toSessionQueuedMessage() = SessionQueuedMessage(
            sessionId = session_id.readString(),
            customData = custom_data?.readString(),
            siteId = site_id.readString())
}

class CSessionTermination : Structure(), Structure.ByValue {
    companion object {
        const val NOMINAL = 1
        const val SITE_UNAVAILABLE = 2
        const val ABORTED_BY_USER = 3
        const val INTENT_NOT_RECOGNIZED = 4
        const val TIMEOUT = 5
        const val ERROR = 6
    }

    @JvmField
    var termination_type: Int? = null
    @JvmField
    var data: Pointer? = null

    override fun getFieldOrder() = listOf("termination_type", "data")

    fun toSessionTermination(): SessionTermination = when (termination_type!!) {
        NOMINAL -> Nominal
        SITE_UNAVAILABLE -> SiteUnAvailable
        ABORTED_BY_USER -> AbortedByUser
        INTENT_NOT_RECOGNIZED -> IntenNotRecognized
        TIMEOUT -> Timeout
        ERROR -> Error(error = data.readString())
        else -> throw IllegalArgumentException("unknown value type $data")
    }
}

class CSessionEndedMessage(p: Pointer) : Structure(p), Structure.ByReference {
    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var termination: CSessionTermination? = null
    @JvmField
    var site_id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("session_id", "custom_data", "termination", "site_id")

    fun toSessionEndedMessage() = SessionEndedMessage(
            sessionId = session_id.readString(),
            customData = custom_data?.readString(),
            siteId = site_id.readString(),
            termination = termination!!.toSessionTermination())
}

class CSayMessage(p: Pointer) : Structure(p), Structure.ByReference {
    @JvmField
    var text: Pointer? = null
    @JvmField
    var lang: Pointer? = null
    @JvmField
    var id: Pointer? = null
    @JvmField
    var site_id: Pointer? = null
    @JvmField
    var session_id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("text", "lang", "id", "site_id", "session_id")

    fun toSayMessage() = SayMessage(
            text = text.readString(),
            lang = lang?.readString(),
            id = id?.readString(),
            siteId = site_id.readString(),
            sessionId = session_id?.readString()
    )
}

class CSayFinishedMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        fun fromSayFinishedMessage(sayFinishedMessage: SayFinishedMessage) = CSayFinishedMessage(null).apply {
            id = sayFinishedMessage.id?.toPointer()
            session_id = sayFinishedMessage.sessionId?.toPointer()
        }
    }

    @JvmField
    var id: Pointer? = null

    @JvmField
    var session_id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("id", "session_id")

    fun toSayFinishedMessage() = SayFinishedMessage(
            id = id?.readString(),
            sessionId = session_id?.readString()
    )
}

class CMapStringToStringArrayEntry(p: Pointer?) : Structure(p), Structure.ByValue {
    companion object {
        fun fromMapEntry(entry: Map.Entry<String, List<String>>) = CMapStringToStringArrayEntry(null).apply {
            key = entry.key.toPointer()
            value = CStringArray.fromStringList(entry.value)
            write()
        }

    }


    @JvmField
    var key: Pointer? = null

    @JvmField
    var value: CStringArray? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("key", "value")

    fun toPair() = key.readString() to (value?.toStringList() ?: listOf())
}

class CMapStringToStringArray(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        fun fromMap(map: Map<String, List<String>>) = CMapStringToStringArray(null).apply {
            count = map.size
            entries = if (map.isNotEmpty()) Memory(Pointer.SIZE * map.size.toLong()).apply {
                map.entries.forEachIndexed { i, e ->
                    this.setPointer(i.toLong() * Pointer.SIZE, CMapStringToStringArrayEntry.fromMapEntry(e).pointer)
                }
            } else null
        }
    }

    @JvmField
    var entries: Pointer? = null

    @JvmField
    var count: Int = -1


    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("entries", "count")

    fun toMap() = if (count > 0) {
        entries!!.getPointerArray(0, count).map { CMapStringToStringArrayEntry(it).toPair() }.toMap()
    } else mapOf()

}

class CInjectionRequestOperation(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        const val KIND_ADD = 1

        fun fromInjectionOperation(input: InjectionOperation) = CInjectionRequestOperation(null).apply {
            values = CMapStringToStringArray.fromMap(input.values)
            kind = when (input.kind) {
                Add -> KIND_ADD
            }
            write()
        }
    }

    @JvmField
    var values: CMapStringToStringArray? = null

    @JvmField
    var kind: Int = -1

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("values", "kind")

    fun toInjectionOperation() = InjectionOperation(
            kind = when (kind) {
                KIND_ADD -> Add
                else -> throw RuntimeException("unknown injection kind $kind")
            },
            values = values?.toMap() ?: mapOf()

    )
}

class CInjectionRequestOperations(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        fun fromInjectionOperationsList(input: List<InjectionOperation>) = CInjectionRequestOperations(null).apply {
            count = input.size
            operations = if(input.isNotEmpty()) Memory(Pointer.SIZE * input.size.toLong()).apply {
                input.forEachIndexed { i, o ->
                    this.setPointer(i.toLong() * Pointer.SIZE, CInjectionRequestOperation.fromInjectionOperation(o).pointer.share(0))
                }
            } else null
        }
    }

    @JvmField
    var operations: Pointer? = null

    @JvmField
    var count: Int = -1

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("operations", "count")

    fun toList() = if (count > 0) {
        operations!!.getPointerArray(0, count).map { CInjectionRequestOperation(it).toInjectionOperation() }
    } else listOf()
}

class CInjectionRequestMessage(p: Pointer?) : Structure(p), Structure.ByReference {

    companion object {
        fun fromInjectionRequest(input: InjectionRequestMessage) = CInjectionRequestMessage(null).apply {
            operations = CInjectionRequestOperations.fromInjectionOperationsList(input.operations)
            lexicon = CMapStringToStringArray.fromMap(input.lexicon)
        }
    }

    @JvmField
    var operations: CInjectionRequestOperations? = null

    @JvmField
    var lexicon: CMapStringToStringArray? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("operations", "lexicon")

    fun toInjectionRequestMessage() = InjectionRequestMessage(
            operations = operations!!.toList(),
            lexicon = lexicon!!.toMap()
    )
}
