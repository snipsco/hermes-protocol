import ai.snips.hermes.*
import ai.snips.hermes.InjectionKind.Add
import ai.snips.hermes.test.HermesTest
import ai.snips.nlu.ontology.Range
import ai.snips.nlu.ontology.SlotValue
import com.google.common.truth.Truth.assertThat
import org.junit.Test

class FfiTest {

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
    }

    @Test
    fun roundTripEndSession() {
        val input = EndSessionMessage(
                text = "smdlfk",
                sessionId = "qsmd3711EAED"
        )
        assertThat(HermesTest().roundTripEndSession(input)).isEqualTo(input)
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

        val input2 = IntentNotRecognizedMessage(
                input = null,
                sessionId = "qsmd3711EAED",
                siteId = "msdklfj",
                customData = null,
                confidenceScore = 0.5f
        )
        assertThat(HermesTest().roundTripIntentNotRecognized(input2)).isEqualTo(input2)
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
    }

    @Test
    fun roundTripMapStringToStringArray() {
        val map = mapOf("toto" to listOf("tutu", "tata"),
                        "" to listOf(),
                        "pif" to listOf("paf", "pouf"))

        assertThat(HermesTest().roundTripMapStringToStringArray(map)).isEqualTo(map)
    }
}
