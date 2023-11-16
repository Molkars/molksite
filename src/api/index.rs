use anyhow::Context;
use async_trait::async_trait;
use crate::api::Api;
use crate::api::infra::{Request, Response};
use crate::html::prelude::{body, h1, header, main, p};

pub struct Index;

#[async_trait]
impl Api for Index {
    async fn get(&self, _req: Request) -> anyhow::Result<Response> {
        Ok(Response::html(
            body()
                .child(header()
                    .child(p("Welcome to the Rusty Web Server!")))
                .child(main()
                    .child(h1("Index"))
                    .child(p("This is the index page.")))
        ))
    }

    async fn post(&self, mut req: Request) -> anyhow::Result<Response> {
        let body = req.body::<String>().await
            .context("failed to read request body")?;

        println!("received body: {:?}", body);

        Ok(Response::empty())
    }
}
