use rocket::{get, State};
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;

use crate::{
    byond::{self, ServerStatus},
    config::{Config, Server},
};

use super::{Cache, CacheEntry, GenericResponse};

#[derive(Debug, Clone, Serialize)]
pub struct Status(Value);

impl Status {
    fn new(server: &Server, status: Option<ServerStatus>) -> Self {
        Self(match status {
            Some(status) => json!({
                "server_status": 1,
                "name": server.name,
                "round_id": status.round_id,
                "players": status.players,
                "map": status.map_name,
                "security_level": status.security_level,
                "round_duration": status.round_duration,
                "gamestate": status.gamestate,
                "connection_info": server.connection_address,
            }),
            None => json!({
                "server_status": 0,
                "name": server.name,
                "err_str": server.error_message,
            }),
        })
    }
}

#[get("/server")]
pub async fn route(
    config: &State<Config>,
    cache: &State<Arc<Mutex<Cache>>>,
) -> GenericResponse<Vec<Status>> {
    let mut cache = cache.lock().await;

    if let Some(cache) = &cache.server {
        if SystemTime::now() < cache.expires {
            return GenericResponse::Success(cache.data.clone());
        }
    }

    let mut response = Vec::new();

    for server in config.servers.iter() {
        let status = byond::status(&server.address).await.ok();
        response.push(Status::new(server, status));
    }

    if !response.is_empty() {
        cache.server = Some(CacheEntry {
            data: response.clone(),
            expires: SystemTime::now() + Duration::from_secs(30),
        });
    }

    GenericResponse::Success(response)
}
