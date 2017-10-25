package ai.snips.hermes.ffi

import ai.snips.hermes.IntentMessage
import ai.snips.hermes.SessionStartedMessage
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
            intent = intent?.toIntentClassifierResult(),
            slots = slots?.toSlotList() ?: listOf())
}

class SessionStartedMessage(p: Pointer) : Structure(p), Structure.ByReference {
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
