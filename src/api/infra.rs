
use std::collections::{hash_map, HashMap};
use std::io::Cursor;
use anyhow::{Context};
use async_trait::async_trait;
use bytes::Bytes;
use h2::RecvStream;
use h2::server::SendResponse;
use http::StatusCode;

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
    body: RecvStream,
}

#[async_trait]
pub trait FromBody: Sized {
    async fn from_body(body: Vec<u8>) -> anyhow::Result<Self>;
}

#[async_trait]
impl FromBody for String {
    async fn from_body(body: Vec<u8>) -> anyhow::Result<Self> {
        String::from_utf8(body)
            .with_context(|| "failed to parse body as utf8")
    }
}

impl Request {
    pub async fn body<T: FromBody>(&mut self) -> anyhow::Result<T> {
        let mut bytes = Vec::new();
        while let Some(chunk) = self.body.data().await {
            let chunk = chunk
                .with_context(|| "failed to read chunk")?;
            bytes.extend_from_slice(&chunk);
        }
        let body = T::from_body(bytes).await?;
        Ok(body)
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
                        request: http::Request<RecvStream>,
                        mut send_response: SendResponse<Bytes>) -> anyhow::Result<()> {
        let path = request.uri().path().to_string();
        let method = request.method().to_string();
        let Some(api) = self.routes.get(&path) else {
            return Ok(());
        };

        let (parts, body) = request.into_parts();
        let request = Request {
            parts,
            body,
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
            Ok(response) if response.code == 0 => {}
            Ok(response) => {
                match StatusCode::from_u16(response.code) {
                    Ok(code) => {
                        let mut res = http::Response::new(());
                        *res.status_mut() = code;
                        if let Some((content_type, body)) = &response.body {
                            res.headers_mut().insert(
                                http::header::CONTENT_TYPE,
                                http::HeaderValue::from_str(content_type)
                                    .expect("failed to parse content type"),
                            );
                            res.headers_mut().insert(
                                http::header::CONTENT_LENGTH,
                                http::HeaderValue::from_str(&body.get_ref().len().to_string())
                                    .expect("failed to parse content length"));
                        }
                        let mut send = send_response.send_response(res, false)?;
                        if let Some((_, body)) = response.body {
                            let bytes = body.into_inner();
                            let body = Bytes::from(bytes);
                            send.send_data(body, true)?;
                        }
                    }
                    Err(_) => {
                        let mut res = http::Response::new(());
                        *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                        let _ = send_response.send_response(res, true);
                    }
                }
            }
            Err(_e) => {
                let mut res = http::Response::new(());
                *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                let _ = send_response.send_response(res, true);
            }
        }

        Ok(())
    }
}
