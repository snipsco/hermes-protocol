package ai.snips.hermes

import ai.snips.queries.ontology.IntentClassifierResult
import ai.snips.queries.ontology.Slot

data class IntentMessage(val sessionId: String, val customData: String?, val siteId: String, val input: String, val intent: IntentClassifierResult?, val slots: List<Slot>)
data class SessionStartedMessage(val sessionId: String, val customData: String?, val siteId: String, val reactivatedFromSessionId: String?)
data class SessionQueuedMessage(val sessionId: String, val customData: String?, val siteId: String)