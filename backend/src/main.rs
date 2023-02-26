use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{
    http::{header, Method},
    Filter, Rejection,
};

mod db;
mod error;
mod handler;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    let pet = warp::path!("owner" / i32 / "pet");
    let pet_param = warp::path!("owner" / i32 / "pet" / i32);
    let owner = warp::path!("owner");

    let pet_routes = pet
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_pets_handler)
        .or(pet
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_pet_handler)
        )
        .or(pet_param
            .and(warp::delete())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_pet_handler)
        );
}

fn with_db(db_pool: DBPool) -> impl Filter