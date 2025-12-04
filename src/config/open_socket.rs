use std::{fmt::Debug, sync::Arc};
#[cfg(windows)]
use windows_sys::Win32::Networking::WinSock::SOCKADDR;

/// A type alias for a reference-counted custom socket opener.
pub type CustomOpenSocket = Arc<dyn OpenSocket>;

/// A helper trait for applying a configuration value to a given curl handle.
pub trait OpenSocket: Debug + Sync + Send {
    /// Open a socket with the given parameters.
    ///
    /// Customizable socket opening function for use with libcurl.
    fn open_socket(
        &self,
        purpose: curl_sys::curlsocktype,
        family: libc::c_int,
        socktype: libc::c_int,
        protocol: libc::c_int,
        address_length: libc::c_uint,
        #[cfg(unix)] address_data: libc::sockaddr,
        #[cfg(windows)] address_data: SOCKADDR,
    ) -> Option<curl_sys::curl_socket_t>;
}
