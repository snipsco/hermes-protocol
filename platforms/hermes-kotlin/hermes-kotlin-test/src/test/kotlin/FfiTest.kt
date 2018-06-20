import ai.snips.hermes.ContinueSessionMessage
import ai.snips.hermes.EndSessionMessage
import ai.snips.hermes.InjectionKind.Add
import ai.snips.hermes.InjectionOperation
import ai.snips.hermes.InjectionRequestMessage
import ai.snips.hermes.SessionInit
import ai.snips.hermes.StartSessionMessage
import ai.snips.hermes.test.HermesTest
import com.google.common.truth.Truth.assertThat
import org.junit.Test

class FfiTest {

    @Test
    fun roundTripStartSessionAction() {
        val input = StartSessionMessage(
                init = SessionInit.Action(
                        text = "smdlfk",
                        intentFilter = listOf("an intent filter", "another intent filter"),
                        canBeEnqueued = true
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
                sessionId = "qsmd3711EAED"
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
    fun roundTripInjectionRequest() {
        val input = InjectionRequestMessage(
                operations = listOf(),
                lexicon = mapOf()
        )

        assertThat(HermesTest().roundTripInjectionRequest(input)).isEqualTo(input)

        val input2 = InjectionRequestMessage(
                operations = listOf(InjectionOperation(Add, mapOf("hello" to listOf("hello", "world"),
                                                                  "yop" to listOf(),
                                                                  "foo" to listOf("bar", "baz")))),
                lexicon = mapOf("toto" to listOf("tutu", "tata"),
                                "" to listOf(),
                                "pif" to listOf("paf", "pouf"))
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
