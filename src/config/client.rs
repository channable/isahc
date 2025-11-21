use crate::NoCustomOpt;

use super::{
    dns::{DnsCache, ResolveMap},
    request::SetOpt,
};
use std::{fmt::Debug, time::Duration};

#[derive(Debug)]
pub(crate) struct ClientConfig<T: SetOpt> {
    pub(crate) connection_cache_ttl: Option<Duration>,
    pub(crate) close_connections: bool,
    pub(crate) dns_cache: Option<DnsCache>,
    pub(crate) dns_resolve: Option<ResolveMap>,
    pub(crate) custom_curl_options: Option<T>,
}

impl Default for ClientConfig<NoCustomOpt> {
    fn default() -> Self {
        Self {
            connection_cache_ttl: Default::default(),
            close_connections: Default::default(),
            dns_cache: Default::default(),
            dns_resolve: Default::default(),
            custom_curl_options: None,
        }
    }
}

impl<T: SetOpt> SetOpt for ClientConfig<T> {
    fn set_opt<H>(&self, easy: &mut curl::easy::Easy2<H>) -> Result<(), curl::Error> {
        if let Some(ttl) = self.connection_cache_ttl {
            easy.maxage_conn(ttl)?;
        }

        if let Some(cache) = self.dns_cache.as_ref() {
            cache.set_opt(easy)?;
        }

        if let Some(map) = self.dns_resolve.as_ref() {
            map.set_opt(easy)?;
        }

        if let Some(custom_curl_options) = self.custom_curl_options.as_ref() {
            custom_curl_options.set_opt(easy)?;
        }

        easy.forbid_reuse(self.close_connections)
    }
}
