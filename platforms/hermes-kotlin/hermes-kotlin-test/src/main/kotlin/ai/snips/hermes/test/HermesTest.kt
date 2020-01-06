package ai.snips.hermes.test

import ai.snips.hermes.AsrToken
import ai.snips.hermes.ContinueSessionMessage
import ai.snips.hermes.DialogueConfigureMessage
import ai.snips.hermes.EndSessionMessage
import ai.snips.hermes.InjectionRequestMessage
import ai.snips.hermes.InjectionCompleteMessage
import ai.snips.hermes.InjectionResetCompleteMessage
import ai.snips.hermes.InjectionResetRequestMessage
import ai.snips.hermes.IntentAlternative
import ai.snips.hermes.IntentMessage
import ai.snips.hermes.IntentNotRecognizedMessage
import ai.snips.hermes.SessionEndedMessage
import ai.snips.hermes.SessionQueuedMessage
import ai.snips.hermes.SessionStartedMessage
import ai.snips.hermes.SessionTermination
import ai.snips.hermes.StartSessionMessage
import ai.snips.hermes.TextCapturedMessage
import ai.snips.hermes.ffi.*
import ai.snips.hermes.test.HermesTest.HermesTestLib.Companion.INSTANCE
import ai.snips.nlu.ontology.ffi.readString
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.ptr.PointerByReference

class HermesTest {
    companion object {
        fun parseError(returnCode: Int) {
            if (returnCode != 0) {
                PointerByReference().apply {
                    INSTANCE.hermes_ffi_test_get_last_error(this)
                    throw RuntimeException(value.getString(0).apply {
                        INSTANCE.hermes_ffi_test_destroy_string(value)
                    })
                }
            }
        }

        val JSON_MAPPER = jacksonObjectMapper()
    }

    fun roundTripContinueSession(input: ContinueSessionMessage) =
            roundTrip(input,
                      CContinueSessionMessage.Companion::fromContinueSessionMessage,
                      INSTANCE::hermes_ffi_test_round_trip_continue_session,
                      { CContinueSessionMessage(it).toContinueSessionMessage() },
                      INSTANCE::hermes_drop_continue_session_message)

    fun roundTripStartSession(input: StartSessionMessage) =
            roundTrip(input,
                      CStartSessionMessage.Companion::fromStartSessionMessage,
                      INSTANCE::hermes_ffi_test_round_trip_start_session,
                      { CStartSessionMessage(it).toStartSessionMessage() },
                      INSTANCE::hermes_drop_start_session_message)

    fun roundTripEndSession(input: EndSessionMessage) =
            roundTrip(input,
                      CEndSessionMessage.Companion::fromEndSessionMessage,
                      INSTANCE::hermes_ffi_test_round_trip_end_session,
                      { CEndSessionMessage(it).toEndSessionMessage() },
                      INSTANCE::hermes_drop_end_session_message)

    fun roundTripIntent(input: IntentMessage) =
            roundTrip(input,
                      CIntentMessage.Companion::fromIntentMessage,
                      INSTANCE::hermes_ffi_test_round_trip_intent,
                      { CIntentMessage(it).toIntentMessage() },
                      INSTANCE::hermes_drop_intent_message)

    fun roundTripIntentAlternative(input: IntentAlternative) =
            roundTrip(input,
                      CNluIntentAlternative.Companion::fromIntentAlternative,
                      INSTANCE::hermes_ffi_test_round_trip_nlu_intent_alternative,
                      { CNluIntentAlternative(it).toIntentAlternative() },
                      INSTANCE::hermes_ffi_test_destroy_nlu_intent_alternative)

    fun roundTripIntentAlternativeArray(input: List<IntentAlternative>) =
            roundTrip(input,
                      CNluIntentAlternativeArray.Companion::fromIntentAlternativeList,
                      INSTANCE::hermes_ffi_test_round_trip_nlu_intent_alternative_array,
                      { CNluIntentAlternativeArray(it).toIntentAlternativeList() },
                      INSTANCE::hermes_ffi_test_destroy_nlu_intent_alternative_array)


    fun roundTripIntentNotRecognized(input: IntentNotRecognizedMessage) =
            roundTrip(input,
                      CIntentNotRecognizedMessage.Companion::fromIntentNotRecognizedMessage,
                      INSTANCE::hermes_ffi_test_round_trip_intent_not_recognized,
                      { CIntentNotRecognizedMessage(it).toIntentNotRecognizedMessage() },
                      INSTANCE::hermes_drop_intent_not_recognized_message)

    fun roundTripInjectionRequest(input: InjectionRequestMessage) =
            roundTrip(input,
                      CInjectionRequestMessage.Companion::fromInjectionRequest,
                      INSTANCE::hermes_ffi_test_round_trip_injection_request,
                      { CInjectionRequestMessage(it).toInjectionRequestMessage() },
                      INSTANCE::hermes_drop_injection_request_message)

    fun roundTripInjectionComplete(input: InjectionCompleteMessage) =
            roundTrip(input,
                      CInjectionCompleteMessage.Companion::fromInjectionCompleteMessage,
                      INSTANCE::hermes_ffi_test_round_trip_injection_complete,
                      { CInjectionCompleteMessage(it).toInjectionCompleteMessage() },
                      INSTANCE::hermes_drop_injection_complete_message)

    fun roundTripInjectionResetRequest(input: InjectionResetRequestMessage) =
            roundTrip(input,
                    CInjectionResetRequestMessage.Companion::fromInjectionResetRequestMessage,
                    INSTANCE::hermes_ffi_test_round_trip_injection_reset_request,
                    { CInjectionResetRequestMessage(it).toInjectionResetRequestMessage() },
                    INSTANCE::hermes_drop_injection_reset_request_message)

    fun roundTripInjectionResetComplete(input: InjectionResetCompleteMessage) =
            roundTrip(input,
                      CInjectionResetCompleteMessage.Companion::fromInjectionResetCompleteMessage,
                      INSTANCE::hermes_ffi_test_round_trip_injection_reset_complete,
                      { CInjectionResetCompleteMessage(it).toInjectionResetCompleteMessage() },
                      INSTANCE::hermes_drop_injection_reset_complete_message)

    fun roundTripMapStringToStringArray(input: Map<String, List<String>>) =
            roundTrip(input,
                      CMapStringToStringArray.Companion::fromMap,
                      INSTANCE::hermes_ffi_test_round_trip_map_string_to_string_array,
                      { CMapStringToStringArray(it).toMap() },
                      INSTANCE::hermes_ffi_test_destroy_map_string_to_string_array)

    fun roundTripAsrToken(input: AsrToken) =
            roundTrip(input,
                      CAsrToken.Companion::cReprOf,
                      INSTANCE::hermes_ffi_test_round_trip_asr_token,
                      { CAsrToken(it).asJava() },
                      INSTANCE::hermes_ffi_test_destroy_asr_token)


    fun roundTripAsrTokenArray(input: List<AsrToken>) =
            roundTripArray<AsrToken, CAsrToken>(
                input,
                INSTANCE::hermes_ffi_test_round_trip_asr_token_array,
                INSTANCE::hermes_ffi_test_destroy_asr_token_array)

    fun roundTripAsrTokenDoubleArray(input: List<List<AsrToken>>): List<List<AsrToken>> {
        return input.map {
            roundTripArray<AsrToken, CAsrToken>(
                    it,
                    INSTANCE::hermes_ffi_test_round_trip_asr_token_array,
                    INSTANCE::hermes_ffi_test_destroy_asr_token_array)
        }
    }

    fun roundTripTextCaptured(input: TextCapturedMessage) =
            roundTrip(input,
                      CTextCapturedMessage.Companion::cReprOf,
                      INSTANCE::hermes_ffi_test_round_trip_text_captured,
                      { CTextCapturedMessage(it).asJava() },
                      INSTANCE::hermes_drop_text_captured_message)

    fun roundTripDialogueConfigure(input: DialogueConfigureMessage) =
            roundTrip(input,
                      CDialogueConfigureMessage.Companion::fromDialogueConfigureMessage,
                      INSTANCE::hermes_ffi_test_round_trip_dialogue_configure,
                      { CDialogueConfigureMessage(it).toDialogueConfigureMessage() },
                      INSTANCE::hermes_drop_dialogue_configure_message)

    fun roundTripSessionEnded(input: SessionEndedMessage) =
        roundTrip(input,
                  CSessionEndedMessage.Companion::fromSessionEndedMessage,
                  INSTANCE::hermes_ffi_test_round_trip_session_ended,
                  { CSessionEndedMessage(it).toSessionEndedMessage() },
                  INSTANCE::hermes_drop_session_ended_message)

    fun <T, U> roundTrip(input: T,
                         toCConverter: (T) -> U,
                         roundTrip: (U, PointerByReference) -> Int,
                         fromCConverter: (Pointer) -> T,
                         drop: (Pointer) -> Int): T {
        return PointerByReference().apply {
            parseError(roundTrip(toCConverter(input), this))
        }.value.let {
            fromCConverter(it).apply {
                parseError(drop(it))
            }
        }
    }

    private inline fun <T,reified U: CStruct<T>> roundTripArray(
            input: List<T>,
            roundTrip: (CArray<T>, PointerByReference) -> Int,
            drop: (Pointer) -> Int
    ): List<T> {
        return PointerByReference().apply {
            parseError(roundTrip(CArray.cReprOf<T, U>(input), this))
        }.value.let {
            CArray<T>(it).asJava<U>().apply {
                parseError(drop(it))
            }
        }
    }


    fun roundTripSessionQueuedJson(input: SessionQueuedMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_session_queued_json)

    fun roundTripSessionStartedJson(input: SessionStartedMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_session_started_json)

    fun roundTripSessionEndedJson(input: SessionEndedMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_session_ended_json)

    fun roundTripIntentJson(input: IntentMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_intent_json)

    fun roundTripIntentNotRecognizedJson(input: IntentNotRecognizedMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_intent_not_recognized_json)

    fun roundTripStartSessionJson(input: StartSessionMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_start_session_json)

    fun roundTripContinueSessionJson(input: ContinueSessionMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_continue_session_json)

    fun roundTripEndSessionJson(input: EndSessionMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_end_session_json)

    fun roundTripInjectionRequestJson(input: InjectionRequestMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_injection_request_json)

    fun roundTripInjectionCompleteJson(input: InjectionCompleteMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_injection_complete_json)

    fun roundTripInjectionResetRequestJson(input: InjectionResetRequestMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_injection_reset_request_json)

    fun roundTripInjectionResetCompleteJson(input: InjectionResetCompleteMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_injection_reset_complete_json)

    fun roundTripTextCapturedJson(input: TextCapturedMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_text_captured_json)

    fun roundTripDialogueConfigureJson(input: DialogueConfigureMessage) =
            roundTripJson(input, INSTANCE::hermes_ffi_test_round_trip_dialogue_configure_json)

    inline fun <reified T> roundTripJson(input: T,
                                         noinline roundTrip: (String, PointerByReference) -> Int) =
            roundTrip(input,
                      JSON_MAPPER::writeValueAsString,
                      roundTrip,
                      { JSON_MAPPER.readValue(it.readString(), T::class.java) },
                      INSTANCE::hermes_ffi_test_destroy_string)


    interface HermesTestLib : Library {
        companion object {
            val INSTANCE: HermesTestLib = Native.loadLibrary("hermes_ffi_test", HermesTestLib::class.java)
        }

        fun hermes_ffi_test_round_trip_start_session(input: CStartSessionMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_continue_session(input: CContinueSessionMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_end_session(input: CEndSessionMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_intent(input: CIntentMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_nlu_intent_alternative(input: CNluIntentAlternative, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_nlu_intent_alternative_array(input: CNluIntentAlternativeArray, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_intent_not_recognized(input: CIntentNotRecognizedMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_injection_request(input: CInjectionRequestMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_injection_complete(input: CInjectionCompleteMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_injection_reset_request(input: CInjectionResetRequestMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_injection_reset_complete(input: CInjectionResetCompleteMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_map_string_to_string_array(input: CMapStringToStringArray, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_asr_token(input: CStruct<AsrToken>, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_asr_token_array(input: CArray<AsrToken>, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_asr_token_double_array(input: CArray<List<AsrToken>>, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_text_captured(input: CStruct<TextCapturedMessage>, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_dialogue_configure(input: CDialogueConfigureMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_session_ended(input: CSessionEndedMessage, output: PointerByReference) : Int


        fun hermes_ffi_test_round_trip_session_queued_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_session_started_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_session_ended_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_intent_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_intent_not_recognized_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_start_session_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_continue_session_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_end_session_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_injection_request_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_injection_complete_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_injection_reset_request_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_injection_reset_complete_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_text_captured_json(input: String, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_dialogue_configure_json(input: String, output: PointerByReference): Int


        fun hermes_ffi_test_get_last_error(error: PointerByReference): Int

        fun hermes_ffi_test_destroy_string(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_map_string_to_string_array(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_asr_token(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_asr_token_array(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_asr_token_double_array(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_nlu_intent_alternative(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_nlu_intent_alternative_array(ptr: Pointer): Int

        fun hermes_drop_continue_session_message(ptr: Pointer): Int
        fun hermes_drop_start_session_message(ptr: Pointer): Int
        fun hermes_drop_end_session_message(ptr: Pointer): Int
        fun hermes_drop_intent_message(ptr: Pointer): Int
        fun hermes_drop_intent_not_recognized_message(ptr: Pointer): Int
        fun hermes_drop_injection_request_message(ptr: Pointer): Int
        fun hermes_drop_injection_complete_message(ptr: Pointer): Int
        fun hermes_drop_injection_reset_request_message(ptr: Pointer): Int
        fun hermes_drop_injection_reset_complete_message(ptr: Pointer): Int
        fun hermes_drop_text_captured_message(ptr: Pointer): Int
        fun hermes_drop_dialogue_configure_message(ptr: Pointer): Int
        fun hermes_drop_session_ended_message(ptr: Pointer) : Int
    }
}
