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
import ai.snips.hermes.IntentAlternative
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
import ai.snips.hermes.StartSessionMessage
import ai.snips.hermes.TextCapturedMessage
import ai.snips.nlu.ontology.Slot
import ai.snips.nlu.ontology.ffi.CSlot
import ai.snips.nlu.ontology.ffi.readRangeTo
import ai.snips.nlu.ontology.ffi.readSlotValue
import ai.snips.nlu.ontology.ffi.readString
import ai.snips.nlu.ontology.ffi.toPointer
import com.sun.jna.Memory
import com.sun.jna.Pointer
import com.sun.jna.Structure

import kotlin.reflect.jvm.jvmErasure

// helper function, enabling CStruct constructor inside the generic api
inline fun <T, reified U : CStruct<T>> factory(p: Pointer?) = U::class
        .constructors
        .firstOrNull {
            it.parameters.size == 1 && it.parameters.first().type.jvmErasure == Pointer::class
        }!!
        .call(p)

abstract class CStruct<T>(p: Pointer?) : Structure(p) {
    abstract class CReprOf<T> {
        abstract fun cReprOf(input: T): CStruct<T>
        // TODO: make cReprOf implementation work instead of abstract class
        // fun cReprOf(input: T): CStruct<T> = factory<T, CStruct<T>>(null).assign(input)
    }

    abstract fun asJava(): T
    abstract fun assign(input: T): CStruct<T>
}

class CArray<T>(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        inline fun <T, reified U : CStruct<T>> cReprOf(input: List<T>) = CArray<T>(null).assign<U>(input)
    }

    @JvmField
    var entry: Pointer? = null
    @JvmField
    var size: Int = -1

    init {
        read()
    }

    override fun getFieldOrder() = listOf("entry", "size")

    inline fun <reified U : CStruct<T>> asJava(): List<T> = if (size > 0) {
        (factory<T, U>(entry).toArray(size) as Array<U>).map { it.asJava() }
    } else {
        listOf()
    }

    inline fun <reified U : CStruct<T>> assign(list: List<T>) = this.apply {
        size = list.size
        entry = if (size > 0) {
            val ref = factory<T, U>(null)
            val cArray: Array<U> = ref.toArray(list.size) as Array<U>
            list.forEachIndexed { i, token -> cArray[i].assign(token).apply { write() } }
            ref.pointer
        } else null
    }
}

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

class CContinueSessionMessage(p: Pointer?) : CStruct<ContinueSessionMessage>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<ContinueSessionMessage>() {
        @JvmStatic
        override fun cReprOf(input: ContinueSessionMessage): CStruct<ContinueSessionMessage> = CContinueSessionMessage(null).assign(input)
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

    override fun asJava(): ContinueSessionMessage = ContinueSessionMessage(
            sessionId = session_id.readString(),
            text = text.readString(),
            intentFilter = intent_filter?.toStringList() ?: listOf(),
            customData = custom_data?.readString(),
            slot = slot?.readString(),
            sendIntentNotRecognized = send_intent_not_recognized == 1.toByte()
    )

    override fun assign(input: ContinueSessionMessage): CStruct<ContinueSessionMessage> = this.apply {
        session_id = input.sessionId.toPointer()
        text = input.text.toPointer()
        intent_filter = if (input.intentFilter.isEmpty()) null else CStringArray.fromStringList(input.intentFilter)
        custom_data = input.customData?.toPointer()
        slot = input.slot?.toPointer()
        send_intent_not_recognized = if (input.sendIntentNotRecognized) 1 else 0
    }
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
        fun cReprOf(@Suppress("UNUSED_PARAMETER") slot: Slot) = CNluSlot(null).apply {
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

    fun asJava() = nlu_slot!!.toSlot()
}

// TODO: this struct needs to be optimised
class CNluSlotArray(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun cReprOf(@Suppress("UNUSED_PARAMETER") list: List<Slot>) = CNluSlotArray(null).apply {
            count = list.size
            entries = if (count > 0) {
                throw java.lang.RuntimeException("Converter for CNluSlotArray not existing yet...")
            } else null
        }
    }

    @JvmField
    var entries: CNluSlot? = null
    @JvmField
    var count: Int = -1

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("entries", "count")

    fun asJava(): List<Slot> = if (count > 0) {
        (entries!!.toArray(count) as Array<CNluSlot>).map { it.asJava() }
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

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("intent_name", "confidence_score")

    fun toIntentClassifierResult() = IntentClassifierResult(intentName = intent_name!!.readString(),
                                                            confidenceScore = confidence_score!!)
}

class CNluIntentAlternative(p: Pointer?) : CStruct<IntentAlternative>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<IntentAlternative>() {
        @JvmStatic
        override fun cReprOf(input: IntentAlternative): CStruct<IntentAlternative> = CNluIntentAlternative(null).assign(input)
    }

    @JvmField
    var intent_name: Pointer? = null
    @JvmField
    var slots: CNluSlotArray? = null
    @JvmField
    var confidence_score: Float? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("intent_name", "slots", "confidence_score")

    override fun asJava() = IntentAlternative(
            intentName = intent_name?.readString(),
            confidenceScore = confidence_score!!,
            slots = slots?.asJava() ?: listOf())

    override fun assign(input: IntentAlternative): CStruct<IntentAlternative> = this.apply {
        intent_name = input.intentName?.toPointer()
        confidence_score = input.confidenceScore
        slots = CNluSlotArray.cReprOf(input.slots)
    }
}

class CIntentMessage(p: Pointer?) : CStruct<IntentMessage>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<IntentMessage>() {
        @JvmStatic
        override fun cReprOf(input: IntentMessage): CStruct<IntentMessage> = CIntentMessage(null).assign(input)
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
    var alternatives: CArray<IntentAlternative>? = null
    @JvmField
    var asr_tokens: CAsrTokenDoubleArray? = null
    @JvmField
    var asr_confidence: Pointer? = null // Pointer to a Float (it's optional in rust)

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("session_id", "custom_data", "site_id", "input", "intent", "slots", "alternatives", "asr_tokens", "asr_confidence")

    override fun asJava() = IntentMessage(
            sessionId = session_id.readString(),
            customData = custom_data?.readString(),
            siteId = site_id.readString(),
            input = input.readString(),
            intent = intent!!.toIntentClassifierResult(),
            slots = slots?.asJava() ?: listOf(),
            alternatives = alternatives?.asJava<CNluIntentAlternative>() ?: listOf(),
            asrConfidence = if (asr_confidence != null) {
                asr_confidence!!.getFloat(0)
            } else null,
            asrTokens = asr_tokens?.asJava()?.toMutableList() ?: mutableListOf())

    override fun assign(input_: IntentMessage): CStruct<IntentMessage> = this.apply {
        session_id = input_.sessionId.toPointer()
        custom_data = input_.customData?.toPointer()
        site_id = input_.siteId.toPointer()
        input = input_.input.toPointer()
        intent = CNluIntentClassifierResult.fromIntentClassifierResult(input_.intent)
        slots = CNluSlotArray.cReprOf(input_.slots)
        alternatives = CArray.cReprOf<IntentAlternative, CNluIntentAlternative>(input_.alternatives)
        asr_tokens = CAsrTokenDoubleArray.cReprOf(input_.asrTokens)
        asr_confidence = if (input_.asrConfidence != null) {
            Memory(Pointer.SIZE.toLong()).apply { write(0, floatArrayOf(input_.asrConfidence.toFloat()), 0, 1) }
        } else null
    }
}

class CIntentNotRecognizedMessage(p: Pointer?) : CStruct<IntentNotRecognizedMessage>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<IntentNotRecognizedMessage>() {
        @JvmStatic
        override fun cReprOf(input: IntentNotRecognizedMessage): CStruct<IntentNotRecognizedMessage> = CIntentNotRecognizedMessage(null).assign(input)
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
    var alternatives: CArray<IntentAlternative>? = null
    @JvmField
    var confidence_score: Float? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("site_id", "session_id", "input", "custom_data", "alternatives", "confidence_score")

    override fun asJava() = IntentNotRecognizedMessage(
            siteId = site_id.readString(),
            sessionId = session_id.readString(),
            input = input?.readString(),
            customData = custom_data?.readString(),
            alternatives = alternatives?.asJava<CNluIntentAlternative>() ?: listOf(),
            confidenceScore = confidence_score!!)

    override fun assign(input_: IntentNotRecognizedMessage): CStruct<IntentNotRecognizedMessage> = this.apply {
        site_id = input_.siteId.toPointer()
        session_id = input_.sessionId.toPointer()
        input = input_.input?.toPointer()
        custom_data = input_.customData?.toPointer()
        alternatives = CArray.cReprOf<IntentAlternative, CNluIntentAlternative>(input_.alternatives)
        confidence_score = input_.confidenceScore
    }
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

class CSessionEndedMessage(p: Pointer?) : CStruct<SessionEndedMessage>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<SessionEndedMessage>() {
        @JvmStatic
        override fun cReprOf(input: SessionEndedMessage): CStruct<SessionEndedMessage> =  CSessionEndedMessage(null).assign(input)
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

    override fun asJava(): SessionEndedMessage = SessionEndedMessage(
            sessionId = session_id.readString(),
            customData = custom_data?.readString(),
            siteId = site_id.readString(),
            termination = termination!!.toSessionTermination()
    )

    override fun assign(input: SessionEndedMessage): CStruct<SessionEndedMessage> = this.apply {
        session_id = input.sessionId.toPointer()
        custom_data = input.customData?.toPointer()
        termination = CSessionTermination.fromSessionTermination(input.termination)
        site_id = input.siteId.toPointer()
    }
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

class CMapStringToStringArrayEntry(p: Pointer?) : Structure(p), Structure.ByReference {
    companion object {
        @JvmStatic
        fun fromMapEntry(entry: Map.Entry<String, List<String>>) = CMapStringToStringArrayEntry(null).apply {
            assignFromMapEntry(entry)
        }
    }

    fun assignFromMapEntry(entry: Map.Entry<String, List<String>>) {
        key = entry.key.toPointer()
        value = CStringArray.fromStringList(entry.value)
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
            entries = if (map.isNotEmpty()) {
                val cMapStringToStringArrayEntryRef = CMapStringToStringArrayEntry(null)
                val cMapStringToStringArray: Array<CMapStringToStringArrayEntry> = cMapStringToStringArrayEntryRef.toArray(count) as Array<CMapStringToStringArrayEntry>

                map.entries.forEachIndexed {i, entry ->
                    cMapStringToStringArray[i].assignFromMapEntry(entry)
                }

                cMapStringToStringArrayEntryRef
            } else null
        }
    }

    @JvmField
    var entries: CMapStringToStringArrayEntry? = null

    @JvmField
    var count: Int = -1

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("entries", "count")

    fun toMap() = if (count > 0) {
        (entries!!.toArray(count) as Array<CMapStringToStringArrayEntry>).map { it.toPair() }.toMap()
    } else mapOf()

}

class CInjectionRequestOperation(p: Pointer?) : CStruct<InjectionOperation>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<InjectionOperation>() {
        const val KIND_ADD = 1
        const val KIND_ADD_FROM_VANILLA = 2

        @JvmStatic
        override fun cReprOf(input: InjectionOperation) = CInjectionRequestOperation(null).assign(input)
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

    override fun asJava(): InjectionOperation = InjectionOperation(
            kind = when (kind) {
                KIND_ADD -> Add
                else -> throw RuntimeException("unknown injection kind $kind")
            },
            values = values?.toMap()?.toMutableMap() ?: mutableMapOf()
    )

    override fun assign(input: InjectionOperation) = this.apply {
        values = CMapStringToStringArray.fromMap(input.values)
        kind = when (input.kind) {
            InjectionKind.Add -> KIND_ADD
            InjectionKind.AddFromVanilla -> KIND_ADD_FROM_VANILLA
        }
    }
}

class CInjectionRequestMessage(p: Pointer?) : CStruct<InjectionRequestMessage>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<InjectionRequestMessage>() {
        @JvmStatic
        override fun cReprOf(input: InjectionRequestMessage) = CInjectionRequestMessage(null).assign(input)
    }

    @JvmField
    var operations: CArray<InjectionOperation>? = null
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

    override fun asJava(): InjectionRequestMessage = InjectionRequestMessage(
            operations = operations!!.asJava<CInjectionRequestOperation>(),
            lexicon = lexicon!!.toMap().toMutableMap(),
            crossLanguage = cross_language?.readString(),
            id = id?.readString()
    )

    override fun assign(input: InjectionRequestMessage): CStruct<InjectionRequestMessage> = this.apply {
        operations = CArray.cReprOf<InjectionOperation, CInjectionRequestOperation>(input.operations)
        lexicon = CMapStringToStringArray.fromMap(input.lexicon)
        cross_language = input.crossLanguage?.toPointer()
        id = input.id?.toPointer()
    }
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

class CAsrToken(p: Pointer?) : CStruct<AsrToken>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<AsrToken>() {
        override fun cReprOf(input: AsrToken) = CAsrToken(null).assign(input)
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

    override fun asJava(): AsrToken = AsrToken(
            value = value.readString(),
            confidence = confidence!!,
            range = AsrTokenRange(range_start, range_end),
            time = time!!.toAsrDecodingDuration()
    )

    override fun assign(input: AsrToken): CStruct<AsrToken> = this.apply {
        value = input.value.toPointer()
        confidence = input.confidence
        range_start = input.range.start
        range_end = input.range.end
        time = CAsrDecodingDuration.fromAsrDecodingDuration(input.time)
    }
}

class CAsrTokenDoubleArray(p: Pointer?) : CStruct<List<List<AsrToken>>>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<List<List<AsrToken>>>()  {
        @JvmStatic
        override fun cReprOf(input: List<List<AsrToken>>) = CAsrTokenDoubleArray(null).assign(input)
    }

    @JvmField
    var entry: Pointer? = null
    @JvmField
    var size: Int = -1

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("entry", "size")

    override fun asJava(): List<List<AsrToken>> = if (size > 0) {
        (CArray<AsrToken>(entry).toArray(size) as Array<CArray<AsrToken>>).map { it.asJava<CAsrToken>() }
    } else {
        listOf()
    }

    override fun assign(input: List<List<AsrToken>>) = this.apply {
        size = input.size
        entry = if (size > 0) {
            val ref = CArray<AsrToken>(null)
            val cArray: Array<CArray<AsrToken>> = ref.toArray(input.size) as Array<CArray<AsrToken>>
            input.forEachIndexed { i, asrTokenList ->
                cArray[i].assign<CAsrToken>(asrTokenList).apply { write() }
            }
            ref.pointer
        } else null
    }
}


class CTextCapturedMessage(p: Pointer?) : CStruct<TextCapturedMessage>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<TextCapturedMessage>() {
        @JvmStatic
        override fun cReprOf(message: TextCapturedMessage) = CTextCapturedMessage(null).assign(message)
    }


    @JvmField
    var text: Pointer? = null
    @JvmField
    var tokens: Pointer? = null
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

    override fun asJava(): TextCapturedMessage = TextCapturedMessage(
            text = text.readString(),
            tokens = if (tokens != null) {
                CArray<AsrToken>(tokens).asJava<CAsrToken>()
            } else listOf(),
            likelihood = likelihood!!,
            seconds = seconds!!,
            siteId = site_id.readString(),
            sessionId = session_id?.readString()
    )

    override fun assign(input: TextCapturedMessage): CStruct<TextCapturedMessage> = this.apply {
        text = input.text.toPointer()
        tokens = if(input.tokens.isNotEmpty()) {
            CArray.cReprOf<AsrToken, CAsrToken>(input.tokens).apply { write() }.pointer
        } else null
        likelihood = input.likelihood
        seconds = input.seconds
        site_id = input.siteId.toPointer()
        session_id = input.sessionId?.toPointer()
    }
}

class CDialogueConfigureIntent(p: Pointer?) : CStruct<DialogueConfigureIntent>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<DialogueConfigureIntent>() {
        @JvmStatic
        override fun cReprOf(input: DialogueConfigureIntent): CStruct<DialogueConfigureIntent> = CDialogueConfigureIntent(null).assign(input)
    }

    @JvmField
    var intent_id: Pointer? = null

    @JvmField
    var enable: Pointer? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("intent_id", "enable")

    override fun asJava(): DialogueConfigureIntent = DialogueConfigureIntent(
            intentId = intent_id.readString(),
            enable = if (enable != null) {
                when(enable!!.getChar(0).toByte()) {
                    0.toByte() -> false
                    1.toByte() -> true
                    else -> true
                }
            } else null
    )

    override fun assign(input: DialogueConfigureIntent): CStruct<DialogueConfigureIntent> = this.apply {
        intent_id = input.intentId.toPointer()
        enable = when(input.enable) {
            true -> Memory(Pointer.SIZE.toLong()).apply { write(0, byteArrayOf(1.toByte()), 0, 1) }
            false -> Memory(Pointer.SIZE.toLong()).apply { write(0, byteArrayOf(0.toByte()), 0, 1) }
            null -> null
        }

    }
}

class CDialogueConfigureMessage(p: Pointer?) : CStruct<DialogueConfigureMessage>(p), Structure.ByReference {
    companion object: CStruct.CReprOf<DialogueConfigureMessage>() {
        @JvmStatic
        override fun cReprOf(input: DialogueConfigureMessage): CStruct<DialogueConfigureMessage> = CDialogueConfigureMessage(null).assign(input)
    }

    @JvmField
    var site_id: Pointer? = null

    @JvmField
    var intents: CArray<DialogueConfigureIntent>? = null

    // be careful this block must be below the field definition if you don't want the native values read by JNA
    // overridden by the default ones
    init {
        read()
    }

    override fun getFieldOrder() = listOf("site_id", "intents")

    override fun asJava(): DialogueConfigureMessage = DialogueConfigureMessage(
            siteId = site_id?.readString(),
            intents = intents?.asJava<CDialogueConfigureIntent>() ?: listOf()
    )

    override fun assign(input: DialogueConfigureMessage): CStruct<DialogueConfigureMessage> = this.apply {
        site_id = input.siteId?.toPointer()
        intents = CArray.cReprOf<DialogueConfigureIntent, CDialogueConfigureIntent>(input.intents)
    }
}
