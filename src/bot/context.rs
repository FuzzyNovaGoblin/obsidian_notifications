use serenity::http::CacheHttp;
use std::sync::Arc;

pub type DisCtx = Arc<CacheAndHttp>;


pub struct CacheAndHttp {
    pub http: Arc<serenity::http::Http>,
    pub cache: Arc<serenity::client::Cache>,
}

impl CacheAndHttp {
    pub fn new(http: Arc<serenity::http::Http>, cache: Arc<serenity::client::Cache>) -> Self {
        Self { http, cache }
    }
}

impl CacheHttp for CacheAndHttp {
    fn http(&self) -> &serenity::http::Http {
        &self.http
    }
    fn cache(&self) -> Option<&Arc<serenity::client::Cache>> {
        Some(&self.cache)
    }
}