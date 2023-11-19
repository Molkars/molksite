use std::collections::{hash_map, HashMap};
use std::io::{Cursor};
use std::path::Path;
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming as BodyIncoming;

pub async fn render(path: impl AsRef<Path>) -> anyhow::Result<> {

}

#[derive(Default)]
pub struct App {
    routes: HashMap<String, Box<dyn Api>>,
}

pub struct Response {
    code: u16,
    body: Option<(String, Cursor<Vec<u8>>)>,
}

pub struct Request {
    parts: http::request::Parts,
    body: Option<BodyIncoming>,
}

#[async_trait]
pub trait FromBody: Sized {
    async fn from_body(body: &[u8]) -> anyhow::Result<Self>;
}

#[async_trait]
impl FromBody for String {
    async fn from_body(body: &[u8]) -> anyhow::Result<Self> {
        String::from_utf8(Vec::from(body))
            .with_context(|| "failed to parse body as utf8")
    }
}

impl Request {
    pub async fn body<T: FromBody>(&mut self) -> anyhow::Result<T> {
        let body = self.body.take()
            .ok_or_else(|| anyhow!("request body already consumed"))?;
        let buf = body.collect().await
            .context("failed to read request body")?;
        let buf = buf.aggregate();
        T::from_body(buf.chunk()).await
    }
}

impl Response {
    pub fn none() -> Response {
        Response {
            code: 0,
            body: Some((
                "text/html".to_string(),
                Cursor::new(Vec::with_capacity(0))
            )),
        }
    }

    pub fn empty() -> Response {
        Response {
            code: 200,
            body: None,
        }
    }

    pub fn text(text: String) -> Response {
        Response {
            code: 200,
            body: Some((
                "text/plain".to_string(),
                Cursor::new(text.into_bytes())
            )),
        }
    }

    pub fn html(html: impl Into<String>) -> Response {
        Response {
            code: 200,
            body: Some((
                "text/html".to_string(),
                Cursor::new(html.into().into_bytes())
            )),
        }
    }

    pub fn created(self) -> Response {
        Response {
            code: 201,
            ..self
        }
    }

    pub fn internal_server_error(self) -> Response {
        Response {
            code: 500,
            ..self
        }
    }

    pub fn not_found(self) -> Response {
        Response {
            code: 404,
            ..self
        }
    }
}

#[async_trait]
pub trait Api: Send + Sync + 'static {
    async fn get(&self, _req: Request) -> anyhow::Result<Response> {
        Ok(Response::none())
    }

    async fn post(&self, _request: Request) -> anyhow::Result<Response> {
        Ok(Response::none())
    }

    async fn put(&self, _request: Request) -> anyhow::Result<Response> {
        Ok(Response::none())
    }

    async fn delete(&self, _request: Request) -> anyhow::Result<Response> {
        Ok(Response::none())
    }

    async fn patch(&self, _request: Request) -> anyhow::Result<Response> {
        Ok(Response::none())
    }

    async fn head(&self, _request: Request) -> anyhow::Result<Response> {
        Ok(Response::none())
    }

    async fn options(&self, _request: Request) -> anyhow::Result<Response> {
        Ok(Response::none())
    }

    async fn trace(&self, _request: Request) -> anyhow::Result<Response> {
        Ok(Response::none())
    }

    async fn connect(&self, _request: Request) -> anyhow::Result<Response> {
        Ok(Response::none())
    }

    async fn custom(&self, _method: String, _request: Request) -> anyhow::Result<Response> {
        Ok(Response::none())
    }
}

impl App {
    pub(crate) fn route<T: Api>(mut self, path: impl Into<String>, api: T) -> Self {
        match self.routes.entry(path.into()) {
            hash_map::Entry::Occupied(_) => panic!("Route already exists!"),
            hash_map::Entry::Vacant(entry) => {
                entry.insert(Box::new(api));
            }
        };
        self
    }

    pub async fn handle(&self,
                        request: http::Request<BodyIncoming>,
    ) -> anyhow::Result<http::Response<Full<Bytes>>> {
        let path = request.uri().path().to_string();
        let method = request.method().to_string();
        let Some(api) = self.routes.get(&path) else {
            return http::Response::builder()
                .status(404)
                .body(Full::new(Bytes::from("Not Found")))
                .map_err(|e| anyhow!("failed to build response: {}", e));
        };

        let (parts, body) = request.into_parts();
        let request = Request {
            parts,
            body: Some(body),
        };
        let response = match method.as_str() {
            "GET" => api.get(request),
            "POST" => api.post(request),
            "PUT" => api.put(request),
            "DELETE" => api.delete(request),
            "PATCH" => api.patch(request),
            "HEAD" => api.head(request),
            "OPTIONS" => api.options(request),
            "TRACE" => api.trace(request),
            "CONNECT" => api.connect(request),
            _ => api.custom(method, request),
        };
        let response = response.await;

        match response {
            Ok(res) => {
                let mut builder = http::Response::builder()
                    .status(res.code);
                if let Some((content_type, body)) = res.body {
                    builder = builder.header("Content-Type", content_type);
                    builder.body(Full::new(Bytes::from(body.into_inner())))
                        .map_err(|e| anyhow!("failed to build response: {}", e))
                } else {
                    builder.body(Full::new(Bytes::from_static(b"")))
                        .map_err(|e| anyhow!("failed to build response: {}", e))
                }
            }
            Err(e) => {
                eprintln!("failed to handle request: {}", e);
                http::Response::builder()
                    .status(500)
                    .body(Full::new(Bytes::from("Internal Server Error")))
                    .map_err(|e| anyhow!("failed to build response: {}", e))
            }
        }
    }
}
