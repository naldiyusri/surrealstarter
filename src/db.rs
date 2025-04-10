use surrealdb::{
    Surreal,
    engine::any::{self, Any},
    opt::auth::Root,
};

use crate::error::Error;

pub const NAMESPACE: &str = "prod_rickspace";
pub const DATABASE: &str = "prod_dbtahlewandoski";

pub async fn connect(address: &str, username: &str, password: &str) -> Result<Surreal<Any>, Error> {
    let db = any::connect(address).await?;

    db.signin(Root { username, password }).await?;

    db.use_ns(NAMESPACE).use_db(DATABASE).await?;

    Ok(db)
}
