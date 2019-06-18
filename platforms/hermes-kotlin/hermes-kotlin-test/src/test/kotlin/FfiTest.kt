import ai.snips.hermes.AsrDecodingDuration
import ai.snips.hermes.AsrToken
import ai.snips.hermes.AsrTokenRange
import ai.snips.hermes.ContinueSessionMessage
import ai.snips.hermes.DialogueConfigureIntent
import ai.snips.hermes.DialogueConfigureMessage
import ai.snips.hermes.EndSessionMessage
import ai.snips.hermes.InjectionCompleteMessage
import ai.snips.hermes.HermesComponent
import ai.snips.hermes.InjectionKind.Add
import ai.snips.hermes.InjectionOperation
import ai.snips.hermes.InjectionRequestMessage
import ai.snips.hermes.IntentClassifierResult
import ai.snips.hermes.IntentMessage
import ai.snips.hermes.IntentNotRecognizedMessage
import ai.snips.hermes.SessionEndedMessage
import ai.snips.hermes.SessionInit
import ai.snips.hermes.SessionQueuedMessage
import ai.snips.hermes.SessionTermination
import ai.snips.hermes.Slot
import ai.snips.hermes.StartSessionMessage
import ai.snips.hermes.TextCapturedMessage
import ai.snips.hermes.test.HermesTest
import ai.snips.nlu.ontology.Range
import ai.snips.nlu.ontology.SlotValue
import com.google.common.truth.Truth.assertThat
import org.junit.Test

class FfiTest {

    @Test
    fun roundTripSessionQueued() {
        val input = SessionQueuedMessage(sessionId = "some session id",
                                         siteId = "some site id",
                                         customData = "some custom data")

        assertThat(HermesTest().roundTripSessionQueuedJson(input)).isEqualTo(input)
    }

    @Test
    fun roundTripStartSessionAction() {
        val input = StartSessionMessage(
                init = SessionInit.Action(
                        text = "smdlfk",
                        intentFilter = listOf("an intent filter", "another intent filter"),
                        canBeEnqueued = true,
                        sendIntentNotRecognized = true
                ),
                customData = "yo",
                siteId = "qlmskdfj"

        )
        assertThat(HermesTest().roundTripStartSession(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripStartSessionJson(input)).isEqualTo(input)
    }

    @Test
    fun roundTripStartSessionNotification() {
        val input = StartSessionMessage(
                init = SessionInit.Notification(
                        text = "smdlfk"
                ),
                customData = "yo",
                siteId = "qlmskdfj"

        )
        assertThat(HermesTest().roundTripStartSession(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripStartSessionJson(input)).isEqualTo(input)
    }

    @Test
    fun roundTripContinueSession() {
        val input = ContinueSessionMessage(
                text = "smdlfk",
                intentFilter = listOf("an intent filter", "another intent filter"),
                sessionId = "qsmd3711EAED",
                sendIntentNotRecognized = true,
                customData = "this is a test custom data",
                slot = "some slot"
        )
        assertThat(HermesTest().roundTripContinueSession(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripContinueSessionJson(input)).isEqualTo(input)
    }

    @Test
    fun roundTripEndSession() {
        val input = EndSessionMessage(
                text = "smdlfk",
                sessionId = "qsmd3711EAED"
        )
        assertThat(HermesTest().roundTripEndSession(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripEndSessionJson(input)).isEqualTo(input)
    }

    @Test
    fun roundIntentNotRecognized() {
        val input = IntentNotRecognizedMessage(
                input = "smdlfk",
                sessionId = "qsmd3711EAED",
                siteId = "msdklfj",
                customData = "fslksk",
                confidenceScore = 0.5f
        )
        assertThat(HermesTest().roundTripIntentNotRecognized(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripIntentNotRecognizedJson(input)).isEqualTo(input)

        val input2 = IntentNotRecognizedMessage(
                input = null,
                sessionId = "qsmd3711EAED",
                siteId = "msdklfj",
                customData = null,
                confidenceScore = 0.5f
        )
        assertThat(HermesTest().roundTripIntentNotRecognized(input2)).isEqualTo(input2)
        assertThat(HermesTest().roundTripIntentNotRecognizedJson(input2)).isEqualTo(input2)
    }

    @Test
    fun roundTripInjectionRequest() {
        val input = InjectionRequestMessage(
                operations = listOf(),
                lexicon = mutableMapOf(),
                crossLanguage = null,
                id = null
        )

        assertThat(HermesTest().roundTripInjectionRequest(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripInjectionRequestJson(input)).isEqualTo(input)

        val input2 = InjectionRequestMessage(
                operations = listOf(InjectionOperation(Add, mutableMapOf("hello" to listOf("hello", "world"),
                                                                         "yop" to listOf(),
                                                                         "foo" to listOf("bar", "baz")))),
                lexicon = mutableMapOf("toto" to listOf("tutu", "tata"),
                                       "" to listOf(),
                                       "pif" to listOf("paf", "pouf")),
                crossLanguage = "en",
                id = "123foo"
        )

        assertThat(HermesTest().roundTripInjectionRequest(input2)).isEqualTo(input2)
        //json is a bit tricky to deserialize properly
        //assertThat(HermesTest().roundTripInjectionRequestJson(input2)).isEqualTo(input2)
    }

    @Test
    fun roundTripInjectionComplete() {
        val input = InjectionCompleteMessage(
                requestId = "foobar"
        )

        assertThat(HermesTest().roundTripInjectionComplete(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripInjectionCompleteJson(input)).isEqualTo(input)
    }

    @Test
    fun roundTripSessionEnded() {
        val input_1 = SessionEndedMessage(
                "some session id",
                "some custom data",
                SessionTermination.AbortedByUser,
                "some site id"
        )

        assertThat(HermesTest().roundTripSessionEndedMessage(input_1)).isEqualTo(input_1)

        val input_2 = SessionEndedMessage(
                "some session id",
                "some custom data",
                SessionTermination.Error(error = "some error"),
                "some site id"
        )

        assertThat(HermesTest().roundTripSessionEndedMessage(input_2)).isEqualTo(input_2)

        val input_3 = SessionEndedMessage(
                "some session id",
                "some custom data",
                SessionTermination.Timeout(component = HermesComponent.ClientApp),
                "some site id"
        )

        assertThat(HermesTest().roundTripSessionEndedMessage(input_3)).isEqualTo(input_3)
    }

    @Test
    fun roundTripMapStringToStringArray() {
        val map = mapOf("toto" to listOf("tutu", "tata"),
                        "" to listOf(),
                        "pif" to listOf("paf", "pouf"))

        assertThat(HermesTest().roundTripMapStringToStringArray(map)).isEqualTo(map)
    }

    @Test
    fun roundTripAsrToken() {
        val input = AsrToken(value = "toto",
                             time = AsrDecodingDuration(start = 1.2f, end = 4.4f),
                             range = AsrTokenRange(start = 5, end = 10),
                             confidence = 0.8f)
        assertThat(HermesTest().roundTripAsrToken(input)).isEqualTo(input)
    }

    @Test
    fun roundTripAsrTokenArray() {
        val input = listOf(AsrToken(value = "toto",
                                    time = AsrDecodingDuration(start = 1.2f, end = 4.4f),
                                    range = AsrTokenRange(start = 5, end = 10),
                                    confidence = 0.8f))
        assertThat(HermesTest().roundTripAsrTokenArray(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripAsrTokenArray(listOf())).isEqualTo(listOf<AsrToken>())
    }

    @Test
    fun roundTripAsrTokenDoubleArray() {
        val input = listOf(listOf(AsrToken(value = "toto",
                                           time = AsrDecodingDuration(start = 1.2f, end = 4.4f),
                                           range = AsrTokenRange(start = 5, end = 10),
                                           confidence = 0.8f)), listOf())
        assertThat(HermesTest().roundTripAsrTokenDoubleArray(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripAsrTokenArray(listOf())).isEqualTo(listOf<List<AsrToken>>())
    }


    @Test
    fun roundTripTextCaptured() {
        val input = TextCapturedMessage(
                text = "hello world",
                sessionId = "a session id",
                siteId = "a site id",
                seconds = 3.2f,
                likelihood = 0.95f,
                tokens = listOf(AsrToken(value = "hello",
                                         time = AsrDecodingDuration(start = 0.2f, end = 1.2f),
                                         range = AsrTokenRange(start = 0, end = 6),
                                         confidence = 0.8f),
                                AsrToken(value = "world",
                                         time = AsrDecodingDuration(start = 1.2f, end = 3.2f),
                                         range = AsrTokenRange(start = 6, end = 10),
                                         confidence = 0.85f))
        )

        assertThat(HermesTest().roundTripTextCaptured(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripTextCapturedJson(input)).isEqualTo(input)

        val input2 = TextCapturedMessage(
                text = "hello world",
                sessionId = null,
                siteId = "a site id",
                seconds = 3.2f,
                likelihood = 0.95f,
                tokens = listOf())
        assertThat(HermesTest().roundTripTextCaptured(input2)).isEqualTo(input2)
        assertThat(HermesTest().roundTripTextCapturedJson(input2)).isEqualTo(input2)
    }


    @Test
    fun roundTripIntent() {
        val input = IntentMessage(
                customData = null,
                siteId = "some site id",
                sessionId = "some session id",
                asrTokens = mutableListOf(),
                asrConfidence = null,
                input = "some input string",
                intent = IntentClassifierResult(
                        intentName = "Some intent",
                        confidenceScore = 0.5f
                ),
                slots = listOf()

        )

        assertThat(HermesTest().roundTripIntent(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripIntentJson(input)).isEqualTo(input)

        val input2 = IntentMessage(
                customData = "some custom data",
                siteId = "some site id",
                sessionId = "some session id",
                asrTokens = mutableListOf(listOf(AsrToken(value = "hello",
                                                          time = AsrDecodingDuration(start = 0.2f, end = 1.2f),
                                                          range = AsrTokenRange(start = 0, end = 6),
                                                          confidence = 0.8f),
                                                 AsrToken(value = "world",
                                                          time = AsrDecodingDuration(start = 1.2f, end = 3.2f),
                                                          range = AsrTokenRange(start = 6, end = 10),
                                                          confidence = 0.85f))),
                asrConfidence = 0.83f,
                input = "some input string",
                intent = IntentClassifierResult(
                        intentName = "Some intent",
                        confidenceScore = 0.5f
                ),
                slots = listOf(Slot(rawValue = "some value",
                                    confidenceScore = 1.0f,
                                    range = Range(start = 1, end = 8),
                                    value = SlotValue.CustomValue("toto"),
                                    entity = "some entity",
                                    slotName = "some slot"))

        )
        // we're still missing a few converters to do that (slot)
        //assertThat(HermesTest().roundTripIntent(input2)).isEqualTo(input2)
        assertThat(HermesTest().roundTripIntentJson(input2)).isEqualTo(input2)

    }

    @Test
    fun roundTripDialogueConfigure() {
        val input = DialogueConfigureMessage(siteId = null, intents = listOf())
        assertThat(HermesTest().roundTripDialogueConfigure(input)).isEqualTo(input)
        assertThat(HermesTest().roundTripDialogueConfigureJson(input)).isEqualTo(input)

        val input2 = DialogueConfigureMessage(siteId = "some site id",
                                              intents = listOf(DialogueConfigureIntent(intentId = "some intent",
                                                                                       enable = true),
                                                               DialogueConfigureIntent(intentId = "some intent",
                                                                                       enable = null),
                                                               DialogueConfigureIntent(intentId = "some intent",
                                                                                       enable = false)))
        assertThat(HermesTest().roundTripDialogueConfigure(input2)).isEqualTo(input2)
        assertThat(HermesTest().roundTripDialogueConfigureJson(input2)).isEqualTo(input2)
    }
}
