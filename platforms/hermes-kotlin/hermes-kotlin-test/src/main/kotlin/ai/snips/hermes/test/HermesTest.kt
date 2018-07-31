package ai.snips.hermes.test

import ai.snips.hermes.ContinueSessionMessage
import ai.snips.hermes.EndSessionMessage
import ai.snips.hermes.InjectionRequestMessage
import ai.snips.hermes.IntentNotRecognizedMessage
import ai.snips.hermes.StartSessionMessage
import ai.snips.hermes.ffi.CContinueSessionMessage
import ai.snips.hermes.ffi.CEndSessionMessage
import ai.snips.hermes.ffi.CInjectionRequestMessage
import ai.snips.hermes.ffi.CIntentNotRecognizedMessage
import ai.snips.hermes.ffi.CIntentNotRecognizedMessage.Companion
import ai.snips.hermes.ffi.CMapStringToStringArray
import ai.snips.hermes.ffi.CStartSessionMessage
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

    fun roundTripContinueSession(input: ContinueSessionMessage): ContinueSessionMessage {
        return PointerByReference().apply {
            parseError(INSTANCE.hermes_ffi_test_round_trip_continue_session(CContinueSessionMessage.fromContinueSessionMessage(input), this))
        }.value.let {
            CContinueSessionMessage(it).toContinueSessionMessage().apply {
                parseError(INSTANCE.hermes_drop_continue_session_message(it))
            }
        }
    }

    fun roundTripStartSession(input: StartSessionMessage): StartSessionMessage {
        return PointerByReference().apply {
            parseError(INSTANCE.hermes_ffi_test_round_trip_start_session(CStartSessionMessage.fromStartSessionMessage(input), this))
        }.value.let {
            CStartSessionMessage(it).toStartSessionMessage().apply {
                parseError(INSTANCE.hermes_drop_start_session_message(it))
            }
        }
    }

    fun roundTripEndSession(input: EndSessionMessage): EndSessionMessage {
        return PointerByReference().apply {
            parseError(INSTANCE.hermes_ffi_test_round_trip_end_session(CEndSessionMessage.fromEndSessionMessage(input), this))
        }.value.let {
            CEndSessionMessage(it).toEndSessionMessage().apply {
                parseError(INSTANCE.hermes_drop_end_session_message(it))
            }
        }
    }

    fun roundTripIntentNotRecognized(input: IntentNotRecognizedMessage): IntentNotRecognizedMessage {
        return PointerByReference().apply {
            parseError(INSTANCE.hermes_ffi_test_round_trip_intent_not_recognized(CIntentNotRecognizedMessage.fromIntentNotRecognizedMessage(input), this))
        }.value.let {
            CIntentNotRecognizedMessage(it).toIntentNotRecognizedMessage().apply {
                parseError(INSTANCE.hermes_drop_intent_not_recognized_message(it))
            }
        }
    }

    fun roundTripInjectionRequest(input: InjectionRequestMessage): InjectionRequestMessage {
        return PointerByReference().apply {
            parseError(INSTANCE.hermes_ffi_test_round_trip_injection_request(CInjectionRequestMessage.fromInjectionRequest(input), this))
        }.value.let {
            CInjectionRequestMessage(it).toInjectionRequestMessage().apply {
                parseError(INSTANCE.hermes_drop_injection_request_message(it))
            }
        }
    }


    fun roundTripMapStringToStringArray(input: Map<String, List<String>>): Map<String, List<String>> {
        return PointerByReference().apply {
            parseError(INSTANCE.hermes_ffi_test_round_trip_map_string_to_string_array(CMapStringToStringArray.fromMap(input), this))
        }.value.let {
            CMapStringToStringArray(it).toMap().apply {
                parseError(INSTANCE.hermes_ffi_test_destroy_map_string_to_string_array(it))
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

        fun hermes_ffi_test_get_last_error(error: PointerByReference): Int

        fun hermes_ffi_test_destroy_string(ptr: Pointer): Int
        fun hermes_ffi_test_destroy_map_string_to_string_array(ptr: Pointer): Int

        fun hermes_drop_continue_session_message(ptr: Pointer): Int
        fun hermes_drop_start_session_message(ptr: Pointer): Int
        fun hermes_drop_end_session_message(ptr: Pointer): Int
        fun hermes_drop_intent_not_recognized_message(ptr: Pointer): Int
        fun hermes_drop_injection_request_message(ptr: Pointer): Int
    }

}

