use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;

use crate::auth::WebOAuth;

pub const CALLBACK_URL: &str = "http://localhost:8080/callback";

#[derive(Clone, Debug)]
pub enum WebCallbackError {
    UnableToBind,
    ChannelClosed
}

pub struct WebCallback {
    receiver: crossbeam::channel::Receiver<Result<String, WebCallbackError>>,
    _handle: tokio::task::JoinHandle<()>
}

impl WebCallback {
    pub async fn oauth(auth: WebOAuth) -> Result<String, WebCallbackError> {
        let manager = Self::spawn();
        let _ = auth.browser_auth().await;
        manager.callback().await
    }

    fn spawn() -> Self {
        let (sender, receiver) = crossbeam::channel::unbounded();
        let _handle = tokio::spawn(start_server(sender));
        Self { receiver, _handle }
    }

    async fn callback(&self) -> Result<String, WebCallbackError> {
        let recv = self.receiver.clone();
        let res = tokio::task::spawn_blocking(move || recv.recv()).await;

        match res {
            Ok(channel_result) => match channel_result {
                Ok(token) => token,
                Err(_) => Err(WebCallbackError::ChannelClosed)
            }
            Err(_) => Err(WebCallbackError::ChannelClosed)
        }
    }
}

#[derive(Debug, Deserialize)]
struct CallbackQuery {
    token: String,
}

async fn start_server(token_sender: crossbeam::channel::Sender<Result<String, WebCallbackError>>) {
    let reporter = token_sender.clone();
    let app = Router::new().route(
        "/callback",
        get(move |Query(params): Query<CallbackQuery>| {
            async move {
                if token_sender.send(Ok(params.token.clone())).is_ok() {
                    Html(format!("
                        <h1>AUTHENTICATION SUCCESSFUL!</h1>
                        <p>Token: {}</p>
                        <p>Close this tab.</p>
                    ", params.token))
                } else {
                    Html("<h1>AUTHENTICATION FAILED.</h1>".to_string())
                }
            }
        }),
    );

    let listener = match tokio::net::TcpListener::bind("127.0.0.1:8080").await {
        Ok(listener) => listener,
        Err(_) => {
            let _ = reporter.send(Err(WebCallbackError::UnableToBind));
            return;
        }
    };

    axum::serve(listener, app).await.unwrap();
}
