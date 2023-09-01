use std::{collections::HashMap, convert::Infallible, sync::Arc};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::Mutex;
use warp::{Filter, Rejection};

mod handlers;
mod models;

type PresenceDb = Arc<Mutex<HashMap<usize, models::UserItem>>>;

type Result<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(DB_POOL_MAX_OPEN)
            .connect(db_url).await {
                Ok(pool) => pool,
                Err(e) => 
                    panic!("Failed to connect to database: {}", e),
                };
            Store {
                connection: db_pool,
            }            
        }
    }

const DB_POOL_MAX_OPEN: u32 = 32;
// const DB_POOL_MAX_IDLE: u32 = 8;
// const DB_POOL_TIMEOUT_SECONDS: u32 = 15;
// const INIT_SQL: &str = "./db.sql";

#[tokio::main]
async fn main() {
    let presence_db: PresenceDb = Arc::new(Mutex::new(HashMap::new()));
    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let store = Store::new("postgres://postgres:postgres@localhost:5432/presence")
        .await
        .expect("Failed to create store");

    let store_filter = warp::any().map(move || store.clone());

    

    let user_items_route = warp::path("user_items")
        .and(warp::get())
        .and(with_presence_db(presence_db.clone()))
        .and_then(handlers::get_user_items);

    let user_item_route = warp::path("user_item")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_presence_db(presence_db.clone()))
        .and_then(handlers::create_user_item)
        .or(warp::path!("user_item" / usize)
            .and(warp::get())
            .and(with_presence_db(presence_db.clone()))
            .and_then(handlers::get_user_item_by_id)
        .or(warp::path!("user_item" / usize)
            .and(warp::put())
            .and(warp::body::json())
            .and(with_presence_db(presence_db.clone()))
            .and_then(handlers::update_user_item_by_id))
        .or(warp::path!("user_item" / usize)
            .and(warp::delete())
            .and(with_presence_db(presence_db.clone()))
            .and_then(handlers::delete_user_item_by_id)));

    let routes = root
        .or(user_items_route)
        .or(user_item_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}

fn with_presence_db(
    presence_db: PresenceDb,
) -> impl Filter<Extract = (PresenceDb,), Error = Infallible> + Clone {
    warp::any().map(move || presence_db.clone())
}