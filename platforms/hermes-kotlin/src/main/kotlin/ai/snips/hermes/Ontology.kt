package ai.snips.hermes

import ai.snips.hermes.SessionInit.Action
import ai.snips.hermes.SessionInit.Notification
import ai.snips.hermes.SessionInit.Type.ACTION
import ai.snips.hermes.SessionInit.Type.NOTIFICATION
import ai.snips.nlu.ontology.Range
import ai.snips.nlu.ontology.SlotValue
import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonFormat
import com.fasterxml.jackson.annotation.JsonFormat.Shape.ARRAY
import com.fasterxml.jackson.annotation.JsonIgnore
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonSubTypes.Type
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonUnwrapped
import org.parceler.Parcel
import org.parceler.Parcel.Serialization.BEAN
import org.parceler.ParcelConstructor
import org.parceler.ParcelProperty

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.EXISTING_PROPERTY, property = "type")
@JsonSubTypes(
        Type(value = Action::class, name = "action"),
        Type(value = Notification::class, name = "notification")
)
sealed class SessionInit(val type: SessionInit.Type) {
    enum class Type {
        @JsonProperty("action")
        ACTION,
        @JsonProperty("notification")
        NOTIFICATION
    }

    @Parcel(BEAN)
    data class Action @ParcelConstructor constructor(
            @ParcelProperty("text") val text: String?,
            @ParcelProperty("intentFilter") val intentFilter: List<String>,
            @ParcelProperty("canBeEnqueued") val canBeEnqueued: Boolean,
            @ParcelProperty("sendIntentNotRecognized") val sendIntentNotRecognized: Boolean
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
        @ParcelProperty("intentFilter") val intentFilter: List<String>,
        @ParcelProperty("customData") val customData: String?,
        @ParcelProperty("slot") val slot: String?,
        @ParcelProperty("sendIntentNotRecognized") val sendIntentNotRecognized: Boolean
)

@Parcel(BEAN)
data class EndSessionMessage @ParcelConstructor constructor(
        @ParcelProperty("sessionId") val sessionId: String,
        @ParcelProperty("text") val text: String?
)

@Parcel(BEAN)
data class Slot @ParcelConstructor constructor(
        @ParcelProperty("rawValue") val rawValue: String,
        @ParcelProperty("value") val value: SlotValue,
        @ParcelProperty("range") val range: Range?,
        @ParcelProperty("entity") val entity: String,
        @ParcelProperty("slotName") val slotName: String,
        @ParcelProperty("confidenceScore") val confidenceScore: Float?)

@Parcel(BEAN)
data class IntentClassifierResult @ParcelConstructor constructor(
        @ParcelProperty("intentName") val intentName: String,
        @ParcelProperty("confidenceScore") val confidenceScore: Float)

@Parcel(BEAN)
data class IntentMessage @ParcelConstructor constructor(
        @ParcelProperty("sessionId") val sessionId: String,
        @ParcelProperty("customData") val customData: String?,
        @ParcelProperty("siteId") val siteId: String,
        @ParcelProperty("input") val input: String,
        @ParcelProperty("intent") val intent: IntentClassifierResult,
        @ParcelProperty("slots") val slots: List<Slot>,
        @ParcelProperty("asrConfidence") val asrConfidence: Float?,
        // Use a mutable list here so that Parceler is happy
        @ParcelProperty("asrTokens") val asrTokens: MutableList<List<AsrToken>>)

@Parcel(BEAN)
data class IntentNotRecognizedMessage @ParcelConstructor constructor(
        @ParcelProperty("sessionId") val sessionId: String,
        @ParcelProperty("customData") val customData: String?,
        @ParcelProperty("siteId") val siteId: String,
        @ParcelProperty("input") val input: String?,
        @ParcelProperty("confidenceScore") val confidenceScore: Float)

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
    @JsonProperty("add") Add,
    @JsonProperty("addFromVanilla") AddFromVanilla,
}

@Parcel(BEAN)
@JsonFormat(shape = ARRAY)
data class InjectionOperation @ParcelConstructor constructor(
        @ParcelProperty("kind") val kind: InjectionKind,
        // Using a MutableMap here so that Parceler is happy
        @ParcelProperty("values") val values: MutableMap<String, List<String>>
)

@Parcel(BEAN)
data class InjectionRequestMessage @ParcelConstructor constructor(
        @ParcelProperty("operations") val operations: List<InjectionOperation>,
        // Using a MutableMap here so that Parceler is happy
        @ParcelProperty("lexicon") val lexicon: MutableMap<String, List<String>>,
        @ParcelProperty("crossLanguage") val crossLanguage: String?,
        @ParcelProperty("id") val id: String?
)

@Parcel(BEAN)
data class AsrDecodingDuration @ParcelConstructor constructor(
        @ParcelProperty("start") val start: Float,
        @ParcelProperty("end") val end: Float
)

@Parcel(BEAN)
data class AsrTokenRange @ParcelConstructor constructor(
        @ParcelProperty("start") val start: Int,
        @ParcelProperty("end") val end: Int
)

@Parcel(BEAN)
data class AsrToken @ParcelConstructor constructor(
        @ParcelProperty("value") val value: String,
        @ParcelProperty("confidence") val confidence: Float,
        @ParcelProperty("range") @JsonIgnore val range: AsrTokenRange,
        @ParcelProperty("time") val time: AsrDecodingDuration
) {
        @Deprecated("use the range property")
        val rangeStart = range.start
        @Deprecated("use the range property")
        val rangeEnd = range.end

        @JsonCreator
        @Deprecated("use the constructor with the range parameter")
        constructor(@JsonProperty("value") value: String,
                @JsonProperty("confidence") confidence: Float,
                @JsonProperty("rangeStart") rangeStart: Int,
                @JsonProperty("rangeEnd") rangeEnd: Int,
                @JsonProperty("time") time: AsrDecodingDuration) :
                this(value, confidence, AsrTokenRange(rangeStart, rangeEnd), time)
}

@Parcel(BEAN)
data class TextCapturedMessage @ParcelConstructor constructor(
        @ParcelProperty("text") val text: String,
        @ParcelProperty("likelihood") val likelihood: Float,
        @ParcelProperty("tokens") val tokens: List<AsrToken>,
        @ParcelProperty("seconds") val seconds: Float,
        @ParcelProperty("siteId") val siteId: String,
        @ParcelProperty("sessionId") val sessionId: String?
)
