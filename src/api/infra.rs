use std::collections::{hash_map, HashMap};
use std::io::Cursor;
use anyhow::anyhow;

#[derive(Default)]
pub struct App {
    routes: HashMap<String, Box<dyn Api>>,
}

pub struct Response {
    code: u16,
    body: Cursor<Vec<u8>>,
    content_type: String,
}

impl Response {
    pub fn none() -> Response {
        Response {
            code: 0,
            content_type: "text/html".to_string(),
            body: Cursor::new(Vec::with_capacity(0)),
        }
    }

    pub fn text(text: String) -> Response {
        Response {
            code: 200,
            content_type: "text/html".to_string(),
            body: Cursor::new(text.into_bytes()),
        }
    }

    pub fn html(html: impl Into<String>) -> Response {
        Response {
            code: 200,
            content_type: "text/html".to_string(),
            body: Cursor::new(html.into().into_bytes()),
        }
    }

    pub fn created(self) -> Response {
        Response {
            code: 201,
            ..self
        }
    }
}

pub trait Api: Send + Sync + 'static {
    fn get(&self) -> Response {
        Response::none()
    }

    fn post(&self) -> Response {
        Response::none()
    }

    fn put(&self) -> Response {
        Response::none()
    }

    fn delete(&self) -> Response {
        Response::none()
    }

    fn patch(&self) -> Response {
        Response::none()
    }

    fn head(&self) -> Response {
        Response::none()
    }

    fn options(&self) -> Response {
        Response::none()
    }

    fn trace(&self) -> Response {
        Response::none()
    }

    fn connect(&self) -> Response {
        Response::none()
    }

    fn custom(&self, method: String) -> Response {
        Response::none()
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

    pub fn handle(&self, request: tiny_http::Request) -> anyhow::Result<()> {
        let path = request.url().to_string();
        let method = request.method().to_string();
        let Some(api) = self.routes.get(&path) else {
            return Ok(());
        };

        let response = match method.as_str() {
            "GET" => api.get(),
            "POST" => api.post(),
            "PUT" => api.put(),
            "DELETE" => api.delete(),
            "PATCH" => api.patch(),
            "HEAD" => api.head(),
            "OPTIONS" => api.options(),
            "TRACE" => api.trace(),
            "CONNECT" => api.connect(),
            _ => api.custom(method),
        };

        if response.code == 0 {
            return Ok(());
        }

        let length = response.body.get_ref().len();


        let response = tiny_http::Response::new(
            tiny_http::StatusCode::from(response.code),
            vec![
                tiny_http::Header::from_bytes(b"Content-Type", response.content_type.as_bytes())
                    .map_err(|_| anyhow!("Invalid header!"))?,
            ],
            response.body,
            Some(length),
            None,
        );
        request.respond(response)?;
        Ok(())
    }
}
