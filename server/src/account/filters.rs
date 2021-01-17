pub fn account(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // addon_list(db.clone())
    // .or(addon_create(db.clone()))
    // .or(addon_update(db.clone()))
    // .or(addon_delete(db))
}
