package ai.snips.hermes.ffi

import ai.snips.hermes.ContinueSessionMessage
import ai.snips.hermes.EndSessionMessage
import ai.snips.hermes.IntentMessage
import ai.snips.hermes.SessionEndedMessage
import ai.snips.hermes.SessionInit
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
import ai.snips.queries.ontology.ffi.CIntentClassifierResult
import ai.snips.queries.ontology.ffi.CSlots
import ai.snips.queries.ontology.ffi.readString
import com.sun.jna.Pointer
import com.sun.jna.Structure


class CArrayString(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField
    var data: Pointer? = null
    @JvmField
    var size: Int = -1

    override fun getFieldOrder() = listOf("data", "size")

    fun toStringList() = if (size > 0) {
        data!!.getPointerArray(0, size).map { it.readString() }
    } else listOf<String>()
}

class CActionSessionInit(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField
    var text: Pointer? = null
    @JvmField
    var intent_filter: Pointer? = null
    @JvmField
    var can_be_enqueued: Byte = -1

    override fun getFieldOrder() = listOf("text", "intent_filter", "can_be_enqueued")

    fun toSessionInit() = SessionInit.Action(
            text = text?.readString(),
            intentFilter = intent_filter?.let { CArrayString(it).toStringList() } ?: listOf(),
            canBeEnqueued = can_be_enqueued == 1.toByte()
    )
}

class CSessionInit : Structure(), Structure.ByValue {
    companion object {
        const val ACTION = 1
        const val NOTIFICATION = 2
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

class CStartSessionMessage(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField
    var init: CActionSessionInit? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var site_id: Pointer? = null

    override fun getFieldOrder() = listOf("init", "custom_data", "site_id")

    fun toStartSessionMessage() = StartSessionMessage(
            init = init!!.toSessionInit(),
            customData = custom_data?.readString(),
            siteId = site_id?.readString()
    )
}

class CContinueSessionMessage(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var text: Pointer? = null
    @JvmField
    var intent_filter: Pointer? = null

    override fun getFieldOrder() = listOf("session_id", "text", "intent_filter")

    fun toContinueSessionMessage() = ContinueSessionMessage(
            sessionId = session_id.readString(),
            text = text.readString(),
            intentFilter = intent_filter?.let { CArrayString(it).toStringList() } ?: listOf()
    )
}

class CEndSessionMessage(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var text: Pointer? = null

    override fun getFieldOrder() = listOf("session_id", "text", "intent_filter")

    fun toEndSessionMessage() = EndSessionMessage(
            sessionId = session_id.readString(),
            text = text?.readString()
    )
}


class CIntentMessage(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
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
    var intent: CIntentClassifierResult? = null
    @JvmField
    var slots: CSlots? = null

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
    init {
        read()
    }

    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var site_id: Pointer? = null
    @JvmField
    var reactivated_from_session_id: Pointer? = null

    override fun getFieldOrder() = listOf("session_id", "custom_data", "site_id", "reactivated_from_session_id")

    fun toSessionStartedMessage() = SessionStartedMessage(
            sessionId = session_id.readString(),
            customData = custom_data?.readString(),
            siteId = site_id.readString(),
            reactivatedFromSessionId = reactivated_from_session_id?.readString())
}

class CSessionQueuedMessage(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var site_id: Pointer? = null

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
    init {
        read()
    }

    @JvmField
    var session_id: Pointer? = null
    @JvmField
    var custom_data: Pointer? = null
    @JvmField
    var termination: CSessionTermination? = null
    @JvmField
    var site_id: Pointer? = null

    override fun getFieldOrder() = listOf("session_id", "custom_data", "termination", "site_id")

    fun toSessionEndedMessage() = SessionEndedMessage(
            sessionId = session_id.readString(),
            customData = custom_data?.readString(),
            siteId = site_id.readString(),
            termination = termination!!.toSessionTermination())
}
