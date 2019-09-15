open Ctypes;
open Foreign;
open HermesReason.Structs;
open HermesReason.Bindings;
open Console;

Unix.putenv("RUST_LOG", "debug");

let suffixes = [".so", ".dylib", ".exe", ""];

suffixes |> List.iter(suff => {
  try (
    Dl.dlopen(~filename="../../target/debug/libhermes_mqtt_ffi" ++ suff, ~flags=[RTLD_NOW]) |> ignore
  ) {
    | _ => (/* Ignore */);
  }
});

let check_res =
  fun
  | Error(i) => {
      let errorStringPtr = allocate(string, "");
      hermes_get_last_error(errorStringPtr) |> ignore;
      Console.error(!@errorStringPtr);
    }
  | _ => ();

hermes_enable_debug_logs() |> check_res;

let protocolHandlerDblPtr =
  allocate(
    ptr @@ CProtocolHandler.view,
    from_voidp(CProtocolHandler.view, null),
  );

let mqttOptions =
  allocate(
    CMqttOptions.view,
    {
      broker_address: "localhost:1883",
      username: None,
      password: None,
      tls_hostname: None,
      tls_ca_file: None,
      tls_ca_path: None,
      tls_client_key: None,
      tls_client_cert: None,
      tls_disable_root_store: false,
    },
  );

hermes_protocol_handler_new_mqtt_with_options(
  protocolHandlerDblPtr,
  mqttOptions,
  null,
)
|> check_res;

let protocolHandlerPtr = !@protocolHandlerDblPtr;

let dialogueFacadeDblPtr =
  allocate(
    ptr(CDialogueFacade.view),
    from_voidp(CDialogueFacade.view, null),
  );

hermes_protocol_handler_dialogue_facade(
  protocolHandlerPtr,
  dialogueFacadeDblPtr,
)
|> check_res;

let dialogueFacadePtr = !@dialogueFacadeDblPtr;

let startSessionMessage =
  CStartSessionMessage.{
    init: Notification("Hello world!"),
    custom_data: Some("Test"),
    site_id: "default",
  };

let startSessionMessagePtr =
  allocate(CStartSessionMessage.view, startSessionMessage);

hermes_dialogue_publish_start_session(
  dialogueFacadePtr,
  startSessionMessagePtr,
)
|> check_res;

hermes_dialogue_subscribe_session_started(dialogueFacadePtr, message => {
  Console.log(message)
})
|> check_res;

hermes_drop_dialogue_facade(dialogueFacadePtr) |> check_res;

while (true) {
  Unix.sleep(60);
};