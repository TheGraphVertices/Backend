use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

type Conn = PooledConnection<ConnectionManager<SqliteConnection>>;
pub fn add_to_table(connection: Conn) {}
