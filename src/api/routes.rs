use crate::api::handlers::{
    listing_by_id_handler,
    listing_send_handler,
    listings_handler,
    orders_by_id_handler,
    orders_send_handler,
};
use crate::db::interfaces::DbConnectionWithMarket;
use warp::{ Filter, Rejection, Reply };
use weaver_core::api::interfaces::{ CFilterConnection, CacheConnection };
use weaver_core::api::utils::{
    get_cors,
    handle_rejection,
    map_api_res,
    post_cors,
    with_node_component,
};

// ========== LISTING ROUTES ========== //

/// GET /listings
///
/// Retrieves all listings from the database
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cuckoo_filter` - The cuckoo filter connection to use
pub async fn listings(
    db: DbConnectionWithMarket,
    cache: CacheConnection
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("listings")
        .and(warp::get())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and_then(move |cache, db| map_api_res(listings_handler(db, cache)))
        .recover(handle_rejection)
        .with(get_cors())
}

/// GET /listings/{id}
///
/// Retrieves a listing from the database by its ID
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cuckoo_filter` - The cuckoo filter connection to use
pub async fn listing_by_id(
    db: DbConnectionWithMarket,
    cache: CacheConnection,
    cuckoo_filter: CFilterConnection
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("listings")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |id, cache, db, cf| map_api_res(listing_by_id_handler(id, db, cache, cf)))
        .recover(handle_rejection)
        .with(get_cors())
}

/// POST /listings/send
///
/// Adds a listing to the database
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cuckoo_filter` - The cuckoo filter connection to use
/// * `body_limit` - The maximum size of the request body
pub async fn listing_send(
    db: DbConnectionWithMarket,
    cache: CacheConnection,
    cuckoo_filter: CFilterConnection,
    body_limit: u64
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("listings" / "send")
        .and(warp::post())
        .and(warp::body::content_length_limit(body_limit))
        .and(warp::body::json())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |data, cache, db, cf| map_api_res(listing_send_handler(data, db, cache, cf)))
        .recover(handle_rejection)
        .with(post_cors())
}

// ========== ORDER ROUTES ========== //

/// GET /orders/{id}
///
/// Retrieves an order from the database by its listing ID
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cuckoo_filter` - The cuckoo filter connection to use
pub async fn orders_by_id(
    db: DbConnectionWithMarket,
    cache: CacheConnection,
    cuckoo_filter: CFilterConnection
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("orders")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |id, cache, db, cf| map_api_res(orders_by_id_handler(id, db, cache, cf)))
        .recover(handle_rejection)
        .with(get_cors())
}

/// POST /orders/send
///
/// Adds an open order to a listing
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cuckoo_filter` - The cuckoo filter connection to use
/// * `body_limit` - The maximum size of the request body
pub async fn orders_send(
    db: DbConnectionWithMarket,
    cache: CacheConnection,
    cuckoo_filter: CFilterConnection,
    body_limit: u64
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("orders" / "send")
        .and(warp::post())
        .and(warp::body::content_length_limit(body_limit))
        .and(warp::body::json())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |data, cache, db, cf| map_api_res(orders_send_handler(data, db, cache, cf)))
        .recover(handle_rejection)
        .with(post_cors())
}
