use async_trait::async_trait;
use crate::api::Api;
use crate::api::infra::{Request, Response};
use crate::html::prelude::*;

pub struct About;

#[async_trait]
impl Api for About {
    async fn get(&self, _req: Request) -> anyhow::Result<Response> {
        let html = div()
            .child(h1("About Me"))
            .child(h2("Dillon Shaffer - Software Tinkerer"))
            .child(div()
                .child(p("I am a software engineer with a passion for learning and building things. I have experience with a wide range of technologies, including:"))
                .child(ul()
                    .child(li().child("Rust"))
                    .child(li().child("Java"))
                    .child(li().child("Dart"))
                    .child(li().child("Python"))
                    .child(li().child("JavaScript"))
                    .child(li().child("Node.js"))

                    .child(li().child("PostgreSQL"))
                    .child(li().child("MongoDB"))
                    .child(li().child("Redis"))

                    .child(li().child("Docker"))
                    .child(li().child("Kubernetes"))
                    .child(li().child("AWS"))

                    .child(li().child("Linux"))
                    .child(li().child("Git"))));

        Ok(Response::html(html))
    }
}