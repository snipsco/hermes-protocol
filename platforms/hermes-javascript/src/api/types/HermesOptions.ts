/**
 * Options used to create an Hermes instance.
 */
export type HermesOptions = {
    /** Hermes bus address. *(default localhost:1883)* */
    address?: string,
    /** Enables or Disables stdout logs. *(default false)* */
    logs?: boolean,
    /** A custom path/name for the dynamic Hermes ffi library. */
    libraryPath?: string,
    /** Username used when connecting to the broker. */
    username?: string,
    /** Password used when connecting to the broker. */
    password?: string,
    /** Hostname to use for the TLS configuration. If set, enables TLS. */
    tls_hostname?: string,
    /** CA files to use if TLS is enabled. */
    tls_ca_file?: string[],
    /** CA paths to use if TLS is enabled. */
    tls_ca_path?: string[],
    /** Client key to use if TLS is enabled. */
    tls_client_key?: string,
    /** Client cert to use if TLS is enabled. */
    tls_client_cert?: string,
    /** Boolean indicating if the root store should be disabled if TLS is enabled. */
    tls_disable_root_store?: string
}
