// we use a lot of snake case property names in this file to match the C headers, lets tell the compiler not to worry
@file:Suppress("PropertyName")

package ai.snips.hermes.ffi

import ai.snips.hermes.AsrDecodingDuration
import ai.snips.hermes.AsrToken
import ai.snips.hermes.AsrTokenRange
import ai.snips.hermes.ContinueSessionMessage
import ai.snips.hermes.DialogueConfigureIntent
import ai.snips.hermes.DialogueConfigureMessage
import ai.snips.hermes.EndSessionMessage
import ai.snips.hermes.HermesComponent
import ai.snips.hermes.HermesComponent.Asr
import ai.snips.hermes.HermesComponent.AudioServer
import ai.snips.hermes.HermesComponent.ClientApp
import ai.snips.hermes.HermesComponent.Dialogue
import ai.snips.hermes.HermesComponent.Hotword
import ai.snips.hermes.HermesComponent.Injection
import ai.snips.hermes.HermesComponent.Nlu
import ai.snips.hermes.HermesComponent.Tts
import ai.snips.hermes.InjectionKind
import ai.snips.hermes.InjectionKind.Add
import ai.snips.hermes.InjectionOperation
import ai.snips.hermes.InjectionRequestMessage
import ai.snips.hermes.InjectionCompleteMessage
import ai.snips.hermes.InjectionResetCompleteMessage
import ai.snips.hermes.InjectionResetRequestMessage
import ai.snips.hermes.IntentClassifierResult
import ai.snips.hermes.IntentMessage
import ai.snips.hermes.IntentNotRecognizedMessage
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
import ai.snips.hermes.Slot
import ai.snips.hermes.StartSessionMessage
import ai.snips.hermes.TextCapturedMessage
import ai.snips.nlu.ontology.ffi.CSlot
import ai.snips.nlu.ontology.ffi.readRangeTo
import ai.snips.nlu.ontology.ffi.readSlotValue
import ai.snips.nlu.ontology.ffi.readString
import ai.snips.nlu.ontology.ffi.toPointer
import com.sun.jna.Memory
import com.sun.jna.Pointer
import com.sun.jna.Structure


class CHermesComponent {
    companion object {
        private const val NONE = -1
        private const val AUDIO_SERVER = 1
        private const val HOTWORD = 2
        private const val ASR = 3
        private const val NLU = 4
        private const val DIALOGUE = 5
        private const val TTS = 6
        private const val INJECTION = 7
        private const val CLIENT_APP = 8

        fun fromHermesComponent(component: HermesComponent?) : Int = when (component) {
            HermesComponent.AudioServer -> AUDIO_SERVER
            HermesComponent.Hotword -> HOTWORD
            HermesComponent.Asr -> ASR
            HermesComponent.Nlu -> NLU
            HermesComponent.Dialogue -> DIALOGUE
            HermesComponent.Tts -> TTS
            HermesComponent.Injection -> INJECTION
            HermesComponent.ClientApp -> CLIENT_APP
            null -> NONE
        }

        fun toHermesComponent(component: Int?) : HermesComponent? = when (component) {
            null, CHermesComponent.NONE -> null
            CHermesComponent.AUDIO_SERVER -> AudioServer
            CHermesComponent.HOTWORD -> Hotword
            CHermesComponent.ASR -> Asr
            CHermesComponent.NLU -> Nlu
            CHermesComponent.DIALOGUE -> Dialogue
            CHermesComponent.TTS -> Tts
            CHermesComponent.INJECTION -> Injection
            CHermesComponent.CLIENT_APP -> ClientApp
            else -> throw IllegalArgumentException("got unexpected component type $component")
        }
    }
}

class CStringArray(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
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
        @JvmStatic
        fun fromActionSessionInit(actionSessionInit: SessionInit.Action) = CActionSessionInit(null).apply {
            text = actionSessionInit.text?.toPointer()
            intent_filter = if (actionSessionInit.intentFilter.isEmpty()) null else CStringArray.fromStringList(actionSessionInit.intentFilter)
            can_be_enqueued = if (actionSessionInit.canBeEnqueued) 1 else 0
            send_intent_not_recognized = if (actionSessionInit.sendIntentNotRecognized) 1 else 0
        }
    }

    @JvmField
    var text: Pointer? = null
    @JvmField
    var intent_filter: CStringArray? = null
    @JvmField
    var can_be_enqueued: Byte = -1
    @JvmField
    var send_intent_not_recognized: Byte = -1

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("text", "intent_filter", "can_be_enqueued", "send_intent_not_recognized")

    fun toSessionInit() = SessionInit.Action(
            text = text?.readString(),
            intentFilter = intent_filter?.toStringList() ?: listOf(),
            canBeEnqueued = can_be_enqueued == 1.toByte(),
            sendIntentNotRecognized = send_intent_not_recognized == 1.toByte()
    )
}

class CSessionInit : Structure(), Structure.ByValue {
    companion object {
        const val ACTION = 1
        const val NOTIFICATION = 2

        @JvmStatic
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
        @JvmStatic
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
        @JvmStatic
        fun fromContinueSessionMessage(continueSessionMessage: ContinueSessionMessage) = CContinueSessionMessage(null).apply {
            session_id = continueSessionMessage.sessionId.toPointer()
            text = continueSessionMessage.text.toPointer()
            intent_filter = if (continueSessionMessage.intentFilter.isEmpty()) null else CStringArray.fromStringList(continueSessionMessage.intentFilter)
            custom_data = continueSessionMessage.customData?.toPointer()
            slot = continueSessionMessage.slot?.toPointer()
            send_intent_not_recognized = if (continueSessionMessage.sendIntentNotRecognized) 1 else 0
        }
    }

    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var text: Pointer? = null
    @JvmField
    var intent_filter: CStringArray? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var slot: Pointer? = null
    @JvmField
    var send_intent_not_recognized: Byte = -1

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("session_id", "text", "intent_filter", "custom_data", "slot", "send_intent_not_recognized")

    fun toContinueSessionMessage() = ContinueSessionMessage(
            sessionId = session_id.readString(),
            text = text.readString(),
            intentFilter = intent_filter?.toStringList() ?: listOf(),
            customData = custom_data?.readString(),
            slot = slot?.readString(),
            sendIntentNotRecognized = send_intent_not_recognized == 1.toByte()
    )
}

class CEndSessionMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
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

class CNluSlot(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromSlot(@Suppress("UNUSED_PARAMETER") slot: Slot) = CNluSlot(null).apply {
            nlu_slot = null
            throw java.lang.RuntimeException("Converter for CSlot not existing yet...")
        }
    }

    @JvmField
    var nlu_slot: CSlot? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("nlu_slot")

    fun toSlot() = Slot(
            rawValue = nlu_slot!!.raw_value.readString(),
            value = nlu_slot!!.value.readSlotValue(),
            range = nlu_slot!!.range_start.readRangeTo(nlu_slot!!.range_end),
            entity = nlu_slot!!.entity.readString(),
            slotName = nlu_slot!!.slot_name.readString(),
            confidenceScore = nlu_slot!!.confidence_score
    )
}

class CNluSlotArray(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromSlotList(list: List<Slot>) = CNluSlotArray(null).apply {
            count = list.size
            entries = if (count > 0)
                Memory(Pointer.SIZE * list.size.toLong()).apply {
                    list.forEachIndexed { i, e ->
                        this.setPointer(i.toLong() * Pointer.SIZE, CNluSlot.fromSlot(e).pointer)
                    }
                }
            else null
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

    fun toSlotList(): List<Slot> = if (count > 0) {
        entries!!.getPointerArray(0, count).map { CNluSlot(it).toSlot() }
    } else listOf()
}

class CNluIntentClassifierResult : Structure(), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromIntentClassifierResult(intentClassifierResult: IntentClassifierResult) =
                CNluIntentClassifierResult().apply {
                    intent_name = intentClassifierResult.intentName.toPointer()
                    confidence_score = intentClassifierResult.confidenceScore
                }
    }

    @JvmField
    var intent_name: Pointer? = null
    @JvmField
    var confidence_score: Float? = null

    override fun getFieldOrder() = listOf("intent_name", "confidence_score")

    fun toIntentClassifierResult() = IntentClassifierResult(intentName = intent_name!!.readString(),
                                                            confidenceScore = confidence_score!!)
}

class CIntentMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromIntentMessage(message: IntentMessage) = CIntentMessage(null).apply {
            session_id = message.sessionId.toPointer()
            custom_data = message.customData?.toPointer()
            site_id = message.siteId.toPointer()
            input = message.input.toPointer()
            intent = CNluIntentClassifierResult.fromIntentClassifierResult(message.intent)
            slots = CNluSlotArray.fromSlotList(message.slots)
            asr_tokens = CAsrTokenDoubleArray.fromAsrTokenDoubleList(message.asrTokens)
            asr_confidence = message.asrConfidence ?: -1.0f
        }
    }

    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var site_id: Pointer? = null
    @JvmField
    var input: Pointer? = null
    @JvmField
    var intent: CNluIntentClassifierResult? = null
    @JvmField
    var slots: CNluSlotArray? = null
    @JvmField
    var asr_tokens: CAsrTokenDoubleArray? = null
    @JvmField
    var asr_confidence: Float? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("session_id", "custom_data", "site_id", "input", "intent", "slots", "asr_tokens", "asr_confidence")

    fun toIntentMessage() = IntentMessage(
            sessionId = session_id.readString(),
            customData = custom_data?.readString(),
            siteId = site_id.readString(),
            input = input.readString(),
            intent = intent!!.toIntentClassifierResult(),
            slots = slots?.toSlotList() ?: listOf(),
            asrConfidence = if(asr_confidence?.let { it in 0.0..1.0 } == true) asr_confidence else null,
            asrTokens = asr_tokens?.toAsrTokenDoubleList()?.toMutableList() ?: mutableListOf())
}

class CIntentNotRecognizedMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromIntentNotRecognizedMessage(message: IntentNotRecognizedMessage) = CIntentNotRecognizedMessage(null).apply {
            site_id = message.siteId.toPointer()
            session_id = message.sessionId.toPointer()
            input = message.input?.toPointer()
            custom_data = message.customData?.toPointer()
            confidence_score = message.confidenceScore
        }
    }

    @JvmField
    var site_id: Pointer? = null
    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var input: Pointer? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var confidence_score: Float? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("site_id", "session_id", "input", "custom_data", "confidence_score")

    fun toIntentNotRecognizedMessage() = IntentNotRecognizedMessage(
            siteId = site_id.readString(),
            sessionId = session_id.readString(),
            input = input?.readString(),
            customData = custom_data?.readString(),
            confidenceScore = confidence_score!!)
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

        @JvmStatic
        fun fromSessionTermination(input: SessionTermination) = when(input) {
                is Timeout -> CSessionTermination().apply {
                    termination_type = TIMEOUT
                    data = null
                    component = CHermesComponent.fromHermesComponent(input.component)
                }
                is Error -> CSessionTermination().apply {
                    termination_type = ERROR
                    data = input.error.toPointer()
                    component = null
                }
                else -> CSessionTermination().apply {
                    termination_type = when(input) {
                        is Nominal -> NOMINAL
                        is SiteUnAvailable -> SITE_UNAVAILABLE
                        is AbortedByUser -> ABORTED_BY_USER
                        is IntenNotRecognized -> INTENT_NOT_RECOGNIZED
                        else -> throw IllegalArgumentException("got unexpected termination type $input")
                    }
                    data = null
                    component = null
                }
        }
    }

    @JvmField
    var termination_type: Int? = null
    @JvmField
    var data: Pointer? = null
    @JvmField
    var component: Int? = null

    override fun getFieldOrder() = listOf("termination_type", "data", "component")

    fun toSessionTermination(): SessionTermination = when (termination_type!!) {
        NOMINAL -> Nominal
        SITE_UNAVAILABLE -> SiteUnAvailable
        ABORTED_BY_USER -> AbortedByUser
        INTENT_NOT_RECOGNIZED -> IntenNotRecognized
        TIMEOUT -> Timeout(component = CHermesComponent.toHermesComponent(component))
        ERROR -> Error(error = data.readString())
        else -> throw IllegalArgumentException("unknown value type $data")
    }
}

class CSessionEndedMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromSessionEndedMessage(input: SessionEndedMessage) = CSessionEndedMessage(null).apply {
            session_id = input.sessionId.toPointer()
            custom_data = input.customData?.toPointer()
            termination = CSessionTermination.fromSessionTermination(input.termination)
            site_id = input.siteId.toPointer()
        }
    }

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
        @JvmStatic
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
        @JvmStatic
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
        @JvmStatic
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
        const val KIND_ADD_FROM_VANILLA = 2

        @JvmStatic
        fun fromInjectionOperation(input: InjectionOperation) = CInjectionRequestOperation(null).apply {
            values = CMapStringToStringArray.fromMap(input.values)
            kind = when (input.kind) {
                InjectionKind.Add -> KIND_ADD
                InjectionKind.AddFromVanilla -> KIND_ADD_FROM_VANILLA
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
            values = values?.toMap()?.toMutableMap() ?: mutableMapOf()
    )
}

class CInjectionRequestOperations(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromInjectionOperationsList(input: List<InjectionOperation>) = CInjectionRequestOperations(null).apply {
            count = input.size
            operations = if (input.isNotEmpty()) Memory(Pointer.SIZE * input.size.toLong()).apply {
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
        @JvmStatic
        fun fromInjectionRequest(input: InjectionRequestMessage) = CInjectionRequestMessage(null).apply {
            operations = CInjectionRequestOperations.fromInjectionOperationsList(input.operations)
            lexicon = CMapStringToStringArray.fromMap(input.lexicon)
            cross_language = input.crossLanguage?.toPointer()
            id = input.id?.toPointer()
        }
    }

    @JvmField
    var operations: CInjectionRequestOperations? = null
    @JvmField
    var lexicon: CMapStringToStringArray? = null
    @JvmField
    var cross_language: Pointer? = null
    @JvmField
    var id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("operations", "lexicon", "cross_language", "id")

    fun toInjectionRequestMessage() = InjectionRequestMessage(
            operations = operations!!.toList(),
            lexicon = lexicon!!.toMap().toMutableMap(),
            crossLanguage = cross_language?.readString(),
            id = id?.readString()
    )
}

class CInjectionCompleteMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromInjectionCompleteMessage(input: InjectionCompleteMessage) = CInjectionCompleteMessage(null).apply {
            request_id = input.requestId?.toPointer()
        }
    }

    @JvmField
    var request_id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("request_id")

    fun toInjectionCompleteMessage() = InjectionCompleteMessage (
            requestId = request_id?.readString()
    )
}

class CInjectionResetRequestMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromInjectionResetRequestMessage(input: InjectionResetRequestMessage) = CInjectionResetRequestMessage(null).apply {
            request_id = input.requestId?.toPointer()
        }
    }

    @JvmField
    var request_id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("request_id")

    fun toInjectionResetRequestMessage() = InjectionResetRequestMessage (
            requestId = request_id?.readString()
    )
}

class CInjectionResetCompleteMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromInjectionResetCompleteMessage(input: InjectionResetCompleteMessage) = CInjectionResetCompleteMessage(null).apply {
            request_id = input.requestId?.toPointer()
        }
    }

    @JvmField
    var request_id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("request_id")

    fun toInjectionResetCompleteMessage() = InjectionResetCompleteMessage (
            requestId = request_id?.readString()
    )
}

class CAsrDecodingDuration : Structure(), Structure.ByValue {
    companion object {
        @JvmStatic
        fun fromAsrDecodingDuration(duration: AsrDecodingDuration) = CAsrDecodingDuration().apply {
            start = duration.start
            end = duration.end
        }
    }


    @JvmField
    var start: Float? = null

    @JvmField
    var end: Float? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }


    override fun getFieldOrder() = listOf("start", "end")

    fun toAsrDecodingDuration() = AsrDecodingDuration(start = start!!,
                                                      end = end!!)
}

class CAsrToken(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromAsrToken(token: AsrToken) = CAsrToken(null).apply {
            value = token.value.toPointer()
            confidence = token.confidence
            range_start = token.range.start
            range_end = token.range.end
            time = CAsrDecodingDuration.fromAsrDecodingDuration(token.time)
        }
    }

    @JvmField
    var value: Pointer? = null

    @JvmField
    var confidence: Float? = null

    @JvmField
    var range_start: Int = -1

    @JvmField
    var range_end: Int = -1

    @JvmField
    var time: CAsrDecodingDuration? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("value", "confidence", "range_start", "range_end", "time")

    fun toAsrToken() = AsrToken(
            value = value.readString(),
            confidence = confidence!!,
            range = AsrTokenRange(range_start, range_end),
            time = time!!.toAsrDecodingDuration()
    )

}


class CAsrTokenArray(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromAsrTokenList(list: List<AsrToken>) = CAsrTokenArray(null).apply {
            count = list.size
            entries = if (count > 0)
                Memory(Pointer.SIZE * list.size.toLong()).apply {
                    list.forEachIndexed { i, e ->
                        this.setPointer(i.toLong() * Pointer.SIZE, CAsrToken.fromAsrToken(e).apply { write() }.pointer)
                    }
                }
            else null
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

    fun toAsrTokenList(): List<AsrToken> = if (count > 0) {
        entries!!.getPointerArray(0, count).map { CAsrToken(it).toAsrToken() }
    } else listOf()
}

class CAsrTokenDoubleArray(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromAsrTokenDoubleList(list: List<List<AsrToken>>) = CAsrTokenDoubleArray(null).apply {
            count = list.size
            entries = if (count > 0)
                Memory(Pointer.SIZE * list.size.toLong()).apply {
                    list.forEachIndexed { i, e ->
                        this.setPointer(i.toLong() * Pointer.SIZE, CAsrTokenArray.fromAsrTokenList(e).apply { write() }.pointer)
                    }
                }
            else null
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

    fun toAsrTokenDoubleList(): List<List<AsrToken>> = if (count > 0) {
        entries!!.getPointerArray(0, count).map { CAsrTokenArray(it).toAsrTokenList() }
    } else listOf()
}


class CTextCapturedMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromTextCapturedMessage(message:TextCapturedMessage) = CTextCapturedMessage(null).apply {
            text = message.text.toPointer()
            tokens = CAsrTokenArray.fromAsrTokenList(message.tokens)
            likelihood = message.likelihood
            seconds = message.seconds
            site_id = message.siteId.toPointer()
            session_id = message.sessionId?.toPointer()
        }
    }


    @JvmField
    var text: Pointer? = null
    @JvmField
    var tokens: CAsrTokenArray? = null
    @JvmField
    var likelihood: Float? = null
    @JvmField
    var seconds: Float? = null
    @JvmField
    var site_id: Pointer? = null
    @JvmField
    var session_id: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("text", "tokens", "likelihood", "seconds", "site_id", "session_id")

    fun toTextCapturedMessage() = TextCapturedMessage(
            text = text.readString(),
            tokens = tokens?.toAsrTokenList() ?: listOf(),
            likelihood = likelihood!!,
            seconds = seconds!!,
            siteId = site_id.readString(),
            sessionId = session_id?.readString()
    )

}

class CDialogueConfigureIntent(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromDialogueConfigureIntent(dialogueConfigureIntent: DialogueConfigureIntent) = CDialogueConfigureIntent(null).apply {
            intent_id = dialogueConfigureIntent.intentId.toPointer()
            enable = when(dialogueConfigureIntent.enable) {
                true -> 1
                false -> 0
                null -> -1
            }
        }
    }

    @JvmField
    var intent_id: Pointer? = null

    @JvmField
    var enable: Byte = -1

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("intent_id", "enable")

    fun toDialogueConfigureIntent() = DialogueConfigureIntent(
            intentId = intent_id.readString(),
            enable = when(enable) {
                0.toByte() -> false
                1.toByte() -> true
                else -> null
            }
    )
}

class CDialogueConfigureIntentArray(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromDialogueConfigureIntentList(list: List<DialogueConfigureIntent>) = CDialogueConfigureIntentArray(null).apply {
            count = list.size
            entries = if (count > 0)
                Memory(Pointer.SIZE * list.size.toLong()).apply {
                    list.forEachIndexed { i, e ->
                        this.setPointer(i.toLong() * Pointer.SIZE, CDialogueConfigureIntent.fromDialogueConfigureIntent(e).apply { write() }.pointer)
                    }
                }
            else null
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

    fun toDialogueConfigureIntentList(): List<DialogueConfigureIntent> = if (count > 0) {
        entries!!.getPointerArray(0, count).map { CDialogueConfigureIntent(it).toDialogueConfigureIntent() }
    } else listOf()
}

class CDialogueConfigureMessage(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromDialogueConfigureMessage(dialogueConfigureMessage: DialogueConfigureMessage) = CDialogueConfigureMessage(null).apply {
            site_id = dialogueConfigureMessage.siteId?.toPointer()
            intents = CDialogueConfigureIntentArray.fromDialogueConfigureIntentList(dialogueConfigureMessage.intents)
        }
    }

    @JvmField
    var site_id: Pointer? = null

    @JvmField
    var intents: CDialogueConfigureIntentArray? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("site_id", "intents")

    fun toDialogueConfigureMessage() = DialogueConfigureMessage(
            siteId = site_id?.readString(),
            intents = intents?.toDialogueConfigureIntentList() ?: listOf()
    )
}
