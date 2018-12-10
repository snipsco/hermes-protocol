#[macro_use]
extern crate failure;
extern crate ffi_utils;
extern crate hermes;
extern crate hermes_ffi;
extern crate hermes_mqtt;
extern crate libc;
#[macro_use]
extern crate log;

use failure::Fallible;
use failure::ResultExt;
use ffi_utils::*;
use hermes_ffi::*;

generate_error_handling!(hermes_get_last_error);

/// A struct representing the configuration of the MQTT client
#[repr(C)]
pub struct CMqttOptions {
    /// Address of the MQTT broker in the form `ip:port`
    broker_address: *mut libc::c_char,
    /// Username to use on the broker. Nullable
    username: *mut libc::c_char,
    /// Password to use on the broker. Nullable
    password: *mut libc::c_char,
    /// Hostname to use for the TLS configuration. Nullable, setting a value enables TLS
    tls_hostname: *mut libc::c_char,
    /// CA files to use if TLS is enabled. Nullable
    tls_ca_file: *mut CStringArray,
    /// CA path to use if TLS is enabled. Nullable
    tls_ca_path: *mut CStringArray,
    /// Client key to use if TLS is enabled. Nullable
    tls_client_key: *mut libc::c_char,
    /// Client cert to use if TLS is enabled. Nullable
    tls_client_cert: *mut libc::c_char,
    /// Boolean indicating if the root store should be disabled if TLS is enabled. The is
    /// interpreted as a boolean, 0 meaning false, all other values meaning true
    tls_disable_root_store: libc::c_uchar,
}

impl AsRust<hermes_mqtt::MqttOptions> for CMqttOptions {
    fn as_rust(&self) -> std::result::Result<hermes_mqtt::MqttOptions, failure::Error> {
        let id = hermes_mqtt::get_mqtt_id();
        let mut options = ::hermes_mqtt::MqttOptions::new(id, create_rust_string_from!(self.broker_address));
        options.username = create_optional_rust_string_from!(self.username);
        options.password = create_optional_rust_string_from!(self.password);
        if let Some(hostname) = create_optional_rust_string_from!(self.tls_hostname) {
            let mut tls = ::hermes_mqtt::TlsOptions::new(hostname);
            tls.disable_root_store = self.tls_disable_root_store != 0;
            tls.cafile = create_optional_rust_vec_string_from!(self.tls_ca_file)
                .unwrap_or(vec![])
                .iter()
                .map(::std::path::PathBuf::from)
                .collect();
            tls.capath = create_optional_rust_vec_string_from!(self.tls_ca_path)
                .unwrap_or(vec![])
                .iter()
                .map(::std::path::PathBuf::from)
                .collect();
            if let (Some(ref k), Some(ref c)) = (
                create_optional_rust_string_from!(self.tls_client_key),
                create_optional_rust_string_from!(self.tls_client_cert),
            ) {
                tls.client_certs_key = Some((c.into(), k.into()));
            }
            debug!("TLS options: {:?}", tls);
            options.tls = Some(tls)
        }
        Ok(options)
    }
}

#[no_mangle]
pub extern "C" fn hermes_protocol_handler_new_mqtt(
    handler: *mut *const CProtocolHandler,
    broker_address: *const libc::c_char,
    user_data: *mut libc::c_void,
) -> SNIPS_RESULT {
    fn new_mqtt_handler(
        handler: *mut *const CProtocolHandler,
        broker_address: *const libc::c_char,
        user_data: *mut libc::c_void,
    ) -> Fallible<()> {
        let address = create_rust_string_from!(broker_address);
        let cph = CProtocolHandler::new(
            Box::new(
                hermes_mqtt::MqttHermesProtocolHandler::new(&address)
                    .with_context(|e| format_err!("Could not create hermes MQTT handler: {:?}", e))?,
            ),
            user_data,
        );
        let ptr = CProtocolHandler::into_raw_pointer(cph);
        unsafe {
            *handler = ptr;
        }
        Ok(())
    }
    wrap!(new_mqtt_handler(handler, broker_address, user_data))
}

#[no_mangle]
pub extern "C" fn hermes_protocol_handler_new_mqtt_with_options(
    handler: *mut *const CProtocolHandler,
    mqtt_options: *const CMqttOptions,
    user_data: *mut libc::c_void,
) -> SNIPS_RESULT {
    fn new_mqtt_handler(
        handler: *mut *const CProtocolHandler,
        mqtt_options: *const CMqttOptions,
        user_data: *mut libc::c_void,
    ) -> Result<(), failure::Error> {
        let options = unsafe { (&*mqtt_options).as_rust() }?;
        let cph = CProtocolHandler::new(
            Box::new(
                hermes_mqtt::MqttHermesProtocolHandler::new_with_options(options)
                    .with_context(|e| format_err!("Could not create hermes MQTT handler: {:?}", e))?,
            ),
            user_data,
        );
        let ptr = CProtocolHandler::into_raw_pointer(cph);
        unsafe {
            *handler = ptr;
        }
        Ok(())
    }
    wrap!(new_mqtt_handler(handler, mqtt_options, user_data))
}

#[no_mangle]
pub extern "C" fn hermes_destroy_mqtt_protocol_handler(handler: *mut CProtocolHandler) -> SNIPS_RESULT {
    fn destroy_mqtt_handler(handler: *mut CProtocolHandler) -> Fallible<()> {
        let handler = unsafe { CProtocolHandler::from_raw_pointer(handler) }?;
        handler.destroy();
        Ok(())
    }
    wrap!(destroy_mqtt_handler(handler))
}

generate_hermes_c_symbols!();
