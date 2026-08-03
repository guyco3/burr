use crate::VirtualizationProxy;
use crate::exports::wasi::http::handler;
use crate::exports::wasi::http::client;
use crate::wasi::http::handler as host_handler;
use crate::wasi::http::client as host_client;
use crate::wasi::http::types::{Request, Response, ErrorCode};
use crate::policy::{Action, PolicyEngine};

impl handler::Guest for VirtualizationProxy {
    async fn handle(request: Request) -> Result<Response, ErrorCode> {
        let policy = crate::policy::get_engine();
        let authority = request.get_authority().unwrap_or_default();
        let path = request.get_path_with_query().unwrap_or_default();
        let method_str = match request.get_method() {
            crate::wasi::http::types::Method::Get => "GET".to_string(),
            crate::wasi::http::types::Method::Post => "POST".to_string(),
            crate::wasi::http::types::Method::Put => "PUT".to_string(),
            crate::wasi::http::types::Method::Delete => "DELETE".to_string(),
            crate::wasi::http::types::Method::Patch => "PATCH".to_string(),
            crate::wasi::http::types::Method::Head => "HEAD".to_string(),
            crate::wasi::http::types::Method::Options => "OPTIONS".to_string(),
            crate::wasi::http::types::Method::Connect => "CONNECT".to_string(),
            crate::wasi::http::types::Method::Trace => "TRACE".to_string(),
            crate::wasi::http::types::Method::Other(s) => s, 
        };

        let full_url = format!("{}{}", authority, path);

        policy.authorize(&Action::HttpIncomingRequest { 
            url: full_url, 
            method: method_str 
        }).map_err(|_| ErrorCode::HttpRequestDenied)?;
        host_handler::handle(request).await
    }
}

impl client::Guest for VirtualizationProxy {
    async fn send(request: Request) -> Result<Response, ErrorCode> {
        let policy = crate::policy::get_engine();
        let authority = request.get_authority().unwrap_or_default();
        let path = request.get_path_with_query().unwrap_or_default();
        let method_str = match request.get_method() {
            crate::wasi::http::types::Method::Get => "GET".to_string(),
            crate::wasi::http::types::Method::Post => "POST".to_string(),
            crate::wasi::http::types::Method::Put => "PUT".to_string(),
            crate::wasi::http::types::Method::Delete => "DELETE".to_string(),
            crate::wasi::http::types::Method::Patch => "PATCH".to_string(),
            crate::wasi::http::types::Method::Head => "HEAD".to_string(),
            crate::wasi::http::types::Method::Options => "OPTIONS".to_string(),
            crate::wasi::http::types::Method::Connect => "CONNECT".to_string(),
            crate::wasi::http::types::Method::Trace => "TRACE".to_string(),
            crate::wasi::http::types::Method::Other(s) => s, 
        };

        let full_url = format!("{}{}", authority, path);

        policy.authorize(&Action::HttpOutgoingRequest { 
            url: full_url, 
            method: method_str 
        }).map_err(|_| ErrorCode::HttpRequestDenied)?;
        host_client::send(request).await
    }
}
