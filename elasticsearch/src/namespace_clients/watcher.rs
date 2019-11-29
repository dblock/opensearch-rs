// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use crate::{
    client::Elasticsearch, enums::*, error::ElasticsearchError, http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Ack Watch API"]
pub enum WatcherAckWatchUrlParts {
    WatchId(String),
    WatchIdActionId(String, Vec<String>),
}
impl WatcherAckWatchUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherAckWatchUrlParts::WatchId(ref watch_id) => {
                let mut p = String::with_capacity(21usize + watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_ack");
                p.into()
            }
            WatcherAckWatchUrlParts::WatchIdActionId(ref watch_id, ref action_id) => {
                let action_id_str = action_id.join(",");
                let mut p = String::with_capacity(22usize + watch_id.len() + action_id_str.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_ack/");
                p.push_str(action_id_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Watcher Ack Watch API"]
pub struct WatcherAckWatch<B> {
    client: Elasticsearch,
    parts: WatcherAckWatchUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> WatcherAckWatch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: WatcherAckWatchUrlParts) -> Self {
        WatcherAckWatch {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherAckWatch<T>
    where
        T: Serialize,
    {
        WatcherAckWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Watcher Ack Watch API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Activate Watch API"]
pub enum WatcherActivateWatchUrlParts {
    WatchId(String),
}
impl WatcherActivateWatchUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherActivateWatchUrlParts::WatchId(ref watch_id) => {
                let mut p = String::with_capacity(26usize + watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_activate");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Watcher Activate Watch API"]
pub struct WatcherActivateWatch<B> {
    client: Elasticsearch,
    parts: WatcherActivateWatchUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> WatcherActivateWatch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: WatcherActivateWatchUrlParts) -> Self {
        WatcherActivateWatch {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherActivateWatch<T>
    where
        T: Serialize,
    {
        WatcherActivateWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Watcher Activate Watch API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Deactivate Watch API"]
pub enum WatcherDeactivateWatchUrlParts {
    WatchId(String),
}
impl WatcherDeactivateWatchUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherDeactivateWatchUrlParts::WatchId(ref watch_id) => {
                let mut p = String::with_capacity(28usize + watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_deactivate");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Watcher Deactivate Watch API"]
pub struct WatcherDeactivateWatch<B> {
    client: Elasticsearch,
    parts: WatcherDeactivateWatchUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> WatcherDeactivateWatch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: WatcherDeactivateWatchUrlParts) -> Self {
        WatcherDeactivateWatch {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherDeactivateWatch<T>
    where
        T: Serialize,
    {
        WatcherDeactivateWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Watcher Deactivate Watch API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Delete Watch API"]
pub enum WatcherDeleteWatchUrlParts {
    Id(String),
}
impl WatcherDeleteWatchUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherDeleteWatchUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(16usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Watcher Delete Watch API"]
pub struct WatcherDeleteWatch {
    client: Elasticsearch,
    parts: WatcherDeleteWatchUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherDeleteWatch {
    pub fn new(client: Elasticsearch, parts: WatcherDeleteWatchUrlParts) -> Self {
        WatcherDeleteWatch {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Watcher Delete Watch API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Execute Watch API"]
pub enum WatcherExecuteWatchUrlParts {
    Id(String),
    None,
}
impl WatcherExecuteWatchUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherExecuteWatchUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(25usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.push_str("/_execute");
                p.into()
            }
            WatcherExecuteWatchUrlParts::None => "/_watcher/watch/_execute".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Watcher Execute Watch API"]
pub struct WatcherExecuteWatch<B> {
    client: Elasticsearch,
    parts: WatcherExecuteWatchUrlParts,
    body: Option<B>,
    debug: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> WatcherExecuteWatch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: WatcherExecuteWatchUrlParts) -> Self {
        WatcherExecuteWatch {
            client,
            parts,
            body: None,
            debug: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherExecuteWatch<T>
    where
        T: Serialize,
    {
        WatcherExecuteWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            debug: self.debug,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "indicates whether the watch should execute in debug mode"]
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = Some(debug);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Watcher Execute Watch API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
                debug: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                debug: self.debug,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Get Watch API"]
pub enum WatcherGetWatchUrlParts {
    Id(String),
}
impl WatcherGetWatchUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherGetWatchUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(16usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Watcher Get Watch API"]
pub struct WatcherGetWatch {
    client: Elasticsearch,
    parts: WatcherGetWatchUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherGetWatch {
    pub fn new(client: Elasticsearch, parts: WatcherGetWatchUrlParts) -> Self {
        WatcherGetWatch {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Watcher Get Watch API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Put Watch API"]
pub enum WatcherPutWatchUrlParts {
    Id(String),
}
impl WatcherPutWatchUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherPutWatchUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(16usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Watcher Put Watch API"]
pub struct WatcherPutWatch<B> {
    client: Elasticsearch,
    parts: WatcherPutWatchUrlParts,
    active: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    source: Option<String>,
    version: Option<i64>,
}
impl<B> WatcherPutWatch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: WatcherPutWatchUrlParts) -> Self {
        WatcherPutWatch {
            client,
            parts,
            active: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            pretty: None,
            source: None,
            version: None,
        }
    }
    #[doc = "Specify whether the watch is in/active by default"]
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherPutWatch<T>
    where
        T: Serialize,
    {
        WatcherPutWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            active: self.active,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            pretty: self.pretty,
            source: self.source,
            version: self.version,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "only update the watch if the last operation that has changed the watch has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only update the watch if the last operation that has changed the watch has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Creates an asynchronous request to the Watcher Put Watch API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
                active: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "if_primary_term", skip_serializing_if = "Option::is_none")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no", skip_serializing_if = "Option::is_none")]
                if_seq_no: Option<i64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
            }
            let query_params = QueryParamsStruct {
                active: self.active,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                pretty: self.pretty,
                source: self.source,
                version: self.version,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Start API"]
pub enum WatcherStartUrlParts {
    None,
}
impl WatcherStartUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherStartUrlParts::None => "/_watcher/_start".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Watcher Start API"]
pub struct WatcherStart<B> {
    client: Elasticsearch,
    parts: WatcherStartUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> WatcherStart<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        WatcherStart {
            client,
            parts: WatcherStartUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherStart<T>
    where
        T: Serialize,
    {
        WatcherStart {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Watcher Start API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Stats API"]
pub enum WatcherStatsUrlParts {
    None,
    Metric(Vec<String>),
}
impl WatcherStatsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherStatsUrlParts::None => "/_watcher/stats".into(),
            WatcherStatsUrlParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(16usize + metric_str.len());
                p.push_str("/_watcher/stats/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Watcher Stats API"]
pub struct WatcherStats {
    client: Elasticsearch,
    parts: WatcherStatsUrlParts,
    emit_stacktraces: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    metric: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherStats {
    pub fn new(client: Elasticsearch, parts: WatcherStatsUrlParts) -> Self {
        WatcherStats {
            client,
            parts,
            emit_stacktraces: None,
            error_trace: None,
            filter_path: None,
            human: None,
            metric: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Emits stack traces of currently running watches"]
    pub fn emit_stacktraces(mut self, emit_stacktraces: bool) -> Self {
        self.emit_stacktraces = Some(emit_stacktraces);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Controls what additional stat metrics should be include in the response"]
    pub fn metric(mut self, metric: Vec<String>) -> Self {
        self.metric = Some(metric);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Watcher Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "emit_stacktraces", skip_serializing_if = "Option::is_none")]
                emit_stacktraces: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "metric",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                metric: Option<Vec<String>>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                emit_stacktraces: self.emit_stacktraces,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                metric: self.metric,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Stop API"]
pub enum WatcherStopUrlParts {
    None,
}
impl WatcherStopUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherStopUrlParts::None => "/_watcher/_stop".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Watcher Stop API"]
pub struct WatcherStop<B> {
    client: Elasticsearch,
    parts: WatcherStopUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> WatcherStop<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        WatcherStop {
            client,
            parts: WatcherStopUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherStop<T>
    where
        T: Serialize,
    {
        WatcherStop {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Watcher Stop API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Watcher APIs"]
pub struct Watcher {
    client: Elasticsearch,
}
impl Watcher {
    pub fn new(client: Elasticsearch) -> Self {
        Watcher { client }
    }
    pub fn ack_watch(&self, parts: WatcherAckWatchUrlParts) -> WatcherAckWatch<()> {
        WatcherAckWatch::new(self.client.clone(), parts)
    }
    pub fn activate_watch(&self, parts: WatcherActivateWatchUrlParts) -> WatcherActivateWatch<()> {
        WatcherActivateWatch::new(self.client.clone(), parts)
    }
    pub fn deactivate_watch(
        &self,
        parts: WatcherDeactivateWatchUrlParts,
    ) -> WatcherDeactivateWatch<()> {
        WatcherDeactivateWatch::new(self.client.clone(), parts)
    }
    pub fn delete_watch(&self, parts: WatcherDeleteWatchUrlParts) -> WatcherDeleteWatch {
        WatcherDeleteWatch::new(self.client.clone(), parts)
    }
    pub fn execute_watch(&self, parts: WatcherExecuteWatchUrlParts) -> WatcherExecuteWatch<()> {
        WatcherExecuteWatch::new(self.client.clone(), parts)
    }
    pub fn get_watch(&self, parts: WatcherGetWatchUrlParts) -> WatcherGetWatch {
        WatcherGetWatch::new(self.client.clone(), parts)
    }
    pub fn put_watch(&self, parts: WatcherPutWatchUrlParts) -> WatcherPutWatch<()> {
        WatcherPutWatch::new(self.client.clone(), parts)
    }
    pub fn start(&self) -> WatcherStart<()> {
        WatcherStart::new(self.client.clone())
    }
    pub fn stats(&self, parts: WatcherStatsUrlParts) -> WatcherStats {
        WatcherStats::new(self.client.clone(), parts)
    }
    pub fn stop(&self) -> WatcherStop<()> {
        WatcherStop::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Watcher APIs"]
    pub fn watcher(&self) -> Watcher {
        Watcher::new(self.clone())
    }
}