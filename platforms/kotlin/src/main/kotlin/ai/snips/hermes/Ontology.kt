package ai.snips.hermes

import ai.snips.queries.ontology.IntentClassifierResult
import ai.snips.queries.ontology.Slot

data class IntentMessage(val input: String, val intent: IntentClassifierResult?, val slots: List<Slot>)
data class IntentNotRecognizedMessage(val input: String, val id: String?)
