package ai.snips.hermes.test

import ai.snips.hermes.AsrToken
import ai.snips.hermes.ContinueSessionMessage
import ai.snips.hermes.EndSessionMessage
import ai.snips.hermes.InjectionRequestMessage
import ai.snips.hermes.IntentNotRecognizedMessage
import ai.snips.hermes.StartSessionMessage
import ai.snips.hermes.TextCapturedMessage
import ai.snips.hermes.ffi.CAsrToken
import ai.snips.hermes.ffi.CAsrTokenArray
import ai.snips.hermes.ffi.CAsrTokenDoubleArray
import ai.snips.hermes.ffi.CContinueSessionMessage
import ai.snips.hermes.ffi.CEndSessionMessage
import ai.snips.hermes.ffi.CInjectionRequestMessage
import ai.snips.hermes.ffi.CIntentNotRecognizedMessage
import ai.snips.hermes.ffi.CMapStringToStringArray
import ai.snips.hermes.ffi.CStartSessionMessage
import ai.snips.hermes.ffi.CTextCapturedMessage
import ai.snips.hermes.test.HermesTest.HermesTestLib.Companion.INSTANCE
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.ptr.PointerByReference


class HermesTest {
    companion object {
        private fun parseError(returnCode: Int) {
            if (returnCode != 0) {
                PointerByReference().apply {
                    INSTANCE.hermes_ffi_test_get_last_error(this)
                    throw RuntimeException(value.getString(0).apply {
                        INSTANCE.hermes_ffi_test_destroy_string(value)
                    })
                }
            }
        }
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

    fun roundTripMapStringToStringArray(input: Map<String, List<String>>) =
            roundTrip(input,
                      CMapStringToStringArray.Companion::fromMap,
                      INSTANCE::hermes_ffi_test_round_trip_map_string_to_string_array,
                      { CMapStringToStringArray(it).toMap() },
                      INSTANCE::hermes_ffi_test_destroy_map_string_to_string_array)

    fun roundTripAsrToken(input: AsrToken) =
            roundTrip(input,
                      CAsrToken.Companion::fromAsrToken,
                      INSTANCE::hermes_ffi_test_round_trip_asr_token,
                      { CAsrToken(it).toAsrToken() },
                      INSTANCE::hermes_ffi_test_destroy_asr_token)


    fun roundTripAsrTokenArray(input: List<AsrToken>) =
            roundTrip(input,
                      CAsrTokenArray.Companion::fromAsrTokenList,
                      INSTANCE::hermes_ffi_test_round_trip_asr_token_array,
                      { CAsrTokenArray(it).toAsrTokenList() },
                      INSTANCE::hermes_ffi_test_destroy_asr_token_array)

    fun roundTripAsrTokenDoubleArray(input: List<List<AsrToken>>) =
            roundTrip(input,
                      CAsrTokenDoubleArray.Companion::fromAsrTokenDoubleList,
                      INSTANCE::hermes_ffi_test_round_trip_asr_token_double_array,
                      { CAsrTokenDoubleArray(it).toAsrTokenDoubleList() },
                      INSTANCE::hermes_ffi_test_destroy_asr_token_double_array)

    fun roundTripTextCaptured(input: TextCapturedMessage) =
            roundTrip(input,
                      CTextCapturedMessage.Companion::fromTextCapturedMessage,
                      INSTANCE::hermes_ffi_test_round_trip_text_captured,
                      { CTextCapturedMessage(it).toTextCapturedMessage() },
                      INSTANCE::hermes_drop_text_captured_message)


    private fun <T, U> roundTrip(input: T,
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


    interface HermesTestLib : Library {
        companion object {
            val INSTANCE: HermesTestLib = Native.loadLibrary("hermes_ffi_test", HermesTestLib::class.java)
        }

        fun hermes_ffi_test_round_trip_start_session(input: CStartSessionMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_continue_session(input: CContinueSessionMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_end_session(input: CEndSessionMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_intent_not_recognized(input: CIntentNotRecognizedMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_injection_request(input: CInjectionRequestMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_map_string_to_string_array(input: CMapStringToStringArray, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_asr_token(input: CAsrToken, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_asr_token_array(input: CAsrTokenArray, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_asr_token_double_array(input: CAsrTokenDoubleArray, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_text_captured(input: CTextCapturedMessage, output: PointerByReference): Int

        fun hermes_ffi_test_get_last_error(error: PointerByReference): Int

        fun hermes_ffi_test_destroy_string(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_map_string_to_string_array(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_asr_token(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_asr_token_array(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_asr_token_double_array(ptr: Pointer): Int

        fun hermes_drop_continue_session_message(ptr: Pointer): Int
        fun hermes_drop_start_session_message(ptr: Pointer): Int
        fun hermes_drop_end_session_message(ptr: Pointer): Int
        fun hermes_drop_intent_not_recognized_message(ptr: Pointer): Int
        fun hermes_drop_injection_request_message(ptr: Pointer): Int
        fun hermes_drop_text_captured_message(ptr: Pointer): Int
    }
}
