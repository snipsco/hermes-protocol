package ai.snips.hermes.test

import ai.snips.hermes.ContinueSessionMessage
import ai.snips.hermes.EndSessionMessage
import ai.snips.hermes.IntentMessage
import ai.snips.hermes.SessionEndedMessage
import ai.snips.hermes.SessionQueuedMessage
import ai.snips.hermes.SessionStartedMessage
import ai.snips.hermes.StartSessionMessage
import ai.snips.hermes.ffi.CContinueSessionMessage
import ai.snips.hermes.ffi.CEndSessionMessage
import ai.snips.hermes.ffi.CIntentMessage
import ai.snips.hermes.ffi.CSessionEndedMessage
import ai.snips.hermes.ffi.CSessionQueuedMessage
import ai.snips.hermes.ffi.CSessionStartedMessage
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

    interface HermesTestLib : Library {
        companion object {
            val INSTANCE: HermesTestLib = Native.loadLibrary("hermes_ffi_test", HermesTestLib::class.java)
        }

        fun hermes_ffi_test_round_trip_start_session(input: CStartSessionMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_continue_session(input: CContinueSessionMessage, output: PointerByReference): Int
        fun hermes_ffi_test_round_trip_end_session(input: CEndSessionMessage, output: PointerByReference): Int

        fun hermes_ffi_test_get_last_error(error: PointerByReference): Int

        fun hermes_ffi_test_destroy_string(ptr: Pointer): Int

        fun hermes_drop_continue_session_message(ptr: Pointer): Int
        fun hermes_drop_start_session_message(ptr: Pointer): Int
        fun hermes_drop_end_session_message(ptr: Pointer): Int
    }

}

