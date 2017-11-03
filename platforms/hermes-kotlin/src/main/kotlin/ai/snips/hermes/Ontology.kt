package ai.snips.hermes

import ai.snips.queries.ontology.IntentClassifierResult
import ai.snips.queries.ontology.Slot

data class IntentMessage(val sessionId: String, val customData: String?, val siteId: String, val input: String, val intent: IntentClassifierResult, val slots: List<Slot>)
data class SessionStartedMessage(val sessionId: String, val customData: String?, val siteId: String, val reactivatedFromSessionId: String?)
data class SessionQueuedMessage(val sessionId: String, val customData: String?, val siteId: String)
data class SessionEndedMessage(val sessionId: String, val customData: String?, val termination: SessionTermination, val siteId: String)

sealed class SessionTermination(val type: SessionTermination.Type) {
    enum class Type {
        NOMINAL,
        SITE_UNAVAILABLE,
        ABORTED_BY_USER,
        INTENT_NOT_RECOGNIZED,
        TIMEOUT,
        ERROR,
    }

    object Nominal : SessionTermination(SessionTermination.Type.NOMINAL)
    object SiteUnAvailable : SessionTermination(SessionTermination.Type.SITE_UNAVAILABLE)
    object AbortedByUser : SessionTermination(SessionTermination.Type.ABORTED_BY_USER)
    object IntenNotRecognized: SessionTermination(SessionTermination.Type.INTENT_NOT_RECOGNIZED)
    object Timeout: SessionTermination(SessionTermination.Type.TIMEOUT)
    data class Error(val error: String) : SessionTermination(SessionTermination.Type.ERROR)
}
