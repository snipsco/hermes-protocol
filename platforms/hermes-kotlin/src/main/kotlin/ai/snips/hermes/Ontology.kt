package ai.snips.hermes

import ai.snips.hermes.SessionInit.Type.ACTION
import ai.snips.hermes.SessionInit.Type.NOTIFICATION
import ai.snips.nlu.ontology.IntentClassifierResult
import ai.snips.nlu.ontology.Slot
import org.parceler.Parcel
import org.parceler.Parcel.Serialization.BEAN
import org.parceler.ParcelConstructor
import org.parceler.ParcelProperty

sealed class SessionInit(val type: SessionInit.Type) {
    enum class Type { ACTION, NOTIFICATION }

    @Parcel(BEAN)
    data class Action @ParcelConstructor constructor(
            @ParcelProperty("text") val text: String?,
            @ParcelProperty("intentFilter") val intentFilter: List<String>,
            @ParcelProperty("canBeEnqueued") val canBeEnqueued: Boolean
    ) : SessionInit(ACTION)

    @Parcel(BEAN)
    data class Notification @ParcelConstructor constructor(
            @ParcelProperty("text") val text: String
    ) : SessionInit(NOTIFICATION)
}

@Parcel(BEAN)
data class StartSessionMessage @ParcelConstructor constructor(
        @ParcelProperty("init") val init: SessionInit,
        @ParcelProperty("customData") val customData: String?,
        @ParcelProperty("siteId") val siteId: String?
)

@Parcel(BEAN)
data class ContinueSessionMessage @ParcelConstructor constructor(
        @ParcelProperty("sessionId") val sessionId: String,
        @ParcelProperty("text") val text: String,
        @ParcelProperty("intentFilter") val intentFilter: List<String>
)

@Parcel(BEAN)
data class EndSessionMessage @ParcelConstructor constructor(
        @ParcelProperty("sessionId") val sessionId: String,
        @ParcelProperty("text") val text: String?
)

@Parcel(BEAN)
data class IntentMessage @ParcelConstructor constructor(
        @ParcelProperty("sessionId") val sessionId: String,
        @ParcelProperty("customData") val customData: String?,
        @ParcelProperty("siteId") val siteId: String,
        @ParcelProperty("input") val input: String,
        @ParcelProperty("intent") val intent: IntentClassifierResult,
        @ParcelProperty("slots") val slots: List<Slot>)

@Parcel(BEAN)
data class SessionStartedMessage @ParcelConstructor constructor(
        @ParcelProperty("sessionId") val sessionId: String,
        @ParcelProperty("customData") val customData: String?,
        @ParcelProperty("siteId") val siteId: String,
        @ParcelProperty("reactivatedFromSessionId") val reactivatedFromSessionId: String?)

@Parcel(BEAN)
data class SessionQueuedMessage @ParcelConstructor constructor(
        @ParcelProperty("sessionId") val sessionId: String,
        @ParcelProperty("customData") val customData: String?,
        @ParcelProperty("siteId") val siteId: String)

@Parcel(BEAN)
data class SessionEndedMessage @ParcelConstructor constructor(
        @ParcelProperty("sessionId") val sessionId: String,
        @ParcelProperty("customData") val customData: String?,
        @ParcelProperty("termination") val termination: SessionTermination,
        @ParcelProperty("siteId") val siteId: String)

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
    object IntenNotRecognized : SessionTermination(SessionTermination.Type.INTENT_NOT_RECOGNIZED)
    object Timeout : SessionTermination(SessionTermination.Type.TIMEOUT)

    @Parcel(BEAN)
    data class Error @ParcelConstructor constructor(
            @ParcelProperty("error") val error: String
    ) : SessionTermination(SessionTermination.Type.ERROR)
}

@Parcel(BEAN)
data class SayMessage @ParcelConstructor constructor(
    @ParcelProperty("text") val text: String,
    @ParcelProperty("lang") val lang: String?,
    @ParcelProperty("id") val id: String?,
    @ParcelProperty("siteId") val siteId: String,
    @ParcelProperty("sessionId") val sessionId: String?
)

@Parcel(BEAN)
data class SayFinishedMessage @ParcelConstructor constructor(
    @ParcelProperty("id") val id: String?,
    @ParcelProperty("sessionId") val sessionId: String?
)

@Parcel
enum class InjectionKind {
    Add
}

@Parcel(BEAN)
data class InjectionOperation @ParcelConstructor constructor(
        @ParcelProperty("kind") val kind : InjectionKind,
        // Using a MutableMap here so that Parceler is happy
        @ParcelProperty("values") val values : MutableMap<String, List<String>>
)

@Parcel(BEAN)
data class InjectionRequestMessage @ParcelConstructor constructor(
        @ParcelProperty("operations") val operations :  List<InjectionOperation>,
        // Using a MutableMap here so that Parceler is happy
        @ParcelProperty("lexicon") val lexicon : MutableMap<String, List<String>>

)
