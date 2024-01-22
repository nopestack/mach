use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::{delete, post};
use axum::{routing::get, Router};
use tokio::sync::{mpsc, RwLock};
use tokio_util::sync::CancellationToken;
use tower_http::trace::TraceLayer;

use crate::api::routes;
use crate::exec::task::Task;
use crate::exec::TaskExecutor;
use crate::storage::FnStorage;

#[derive(Debug)]
pub struct Server {
    router: Router,
    addr: SocketAddr,
    cancel_token: CancellationToken,
}

pub type JobTx = mpsc::Sender<Task>;
pub const API_VERSION: &str = "v1";

#[derive(Debug, Clone)]
pub struct ServerState<F, T>
where
    F: FnStorage,
    T: TaskExecutor,
{
    pub task_exec: T,
    pub storage_backend: F,
}

pub type SharedServerState<F, T> = Arc<RwLock<ServerState<F, T>>>;

impl Server {
    pub fn new<F, T>(addr: SocketAddr, task_exec: T, storage_backend: F) -> Self
    where
        F: FnStorage + 'static + std::marker::Sync,
        T: TaskExecutor + 'static,
    {
        let state = Arc::new(RwLock::new(ServerState {
            task_exec,
            storage_backend,
        }));

        let v1_routes = Router::new()
            .route("/", get(routes::root::root))
            .route("/functions", get(routes::list::list_handler))
            .route("/functions", post(routes::upload::upload_handler))
            .route("/functions/:id", get(routes::get::get_handler))
            .route("/functions/:id", delete(routes::delete::delete_handler))
            .route("/functions/:id", post(routes::call::call_handler));

        let api_prefix = format!("/{API_VERSION}");

        let router = Router::new()
            .nest(&api_prefix, v1_routes)
            .layer(TraceLayer::new_for_http())
            .fallback(routes::not_found::not_found_handler)
            .with_state(state);

        let cancel_token = CancellationToken::new();

        Self {
            router,
            cancel_token,
            addr,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn run(&mut self) -> anyhow::Result<()> {
        tracing::info!("starting server...");
        let listener = tokio::net::TcpListener::bind(self.addr).await?;

        self.addr = listener.local_addr()?;

        tracing::info!("server listening on {addr}", addr = listener.local_addr()?);

        tokio::spawn(server_task(
            self.cancel_token.clone(),
            listener,
            self.router.clone(),
        ));

        Ok(())
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn stop(&self) {
        tracing::info!("stopping server...");
        self.cancel_token.cancel();
        tracing::info!("server stopped");
    }
}

async fn server_task(
    cancel_token: CancellationToken,
    listener: tokio::net::TcpListener,
    router: Router,
) -> anyhow::Result<()> {
    let cancel_token = cancel_token.clone();

    if let Err(err) = axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            cancel_token.cancelled().await;
        })
        .await
    {
        tracing::error!("server error: {err}");
    }

    Ok(())
}
