
use std::env;
use std::convert::TryFrom;
// use log::{error, info, warn};
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::connection::*;
use diesel::pg::PgConnection;
use diesel::result::QueryResult;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use diesel::expression_methods::NullableExpressionMethods;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};

pub mod schema;
pub mod models;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool.")
}

// TODO where should this go?
// fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
//     pool.get().map_err(|err| {
//         HttpResponse::InternalServerError().json(err.to_string())
//     })
// }

pub fn commit(connection: &PgConnection) -> QueryResult<()> {
    let txn_manager: AnsiTransactionManager = AnsiTransactionManager::new();
    txn_manager.commit_transaction(connection)
}

pub fn rollback(connection: &PgConnection) -> QueryResult<()> {
    let txn_manager: AnsiTransactionManager = AnsiTransactionManager::new();
    txn_manager.rollback_transaction(connection)
}

pub fn drop_fork(eval_block_num: i64, connection: &PgConnection) -> QueryResult<usize> {
    use schema::blocks::dsl::*;
    use schema::accounts::dsl::*;

    diesel::delete(
        accounts.filter(start_block_num.nullable().eq(eval_block_num))
    ).execute(connection)?;
    diesel::update(
        accounts.filter(end_block_num.nullable().eq(eval_block_num))
    ).set(end_block_num.eq(None::<i64>)).execute(connection)?;
    diesel::delete(blocks.filter(block_num.ge(eval_block_num))).execute(connection)
}

pub fn insert_block(new_block_num: i64, new_block_id: &str, connection: &PgConnection) -> QueryResult<usize> {
    use schema::blocks::dsl::*;

    let block: models::NewBlock = models::NewBlock {
        block_num: new_block_num,
        block_id: new_block_id,
    };

    diesel::insert_into(blocks).values(&block).execute(connection)
}

pub fn fetch_last_known_blocks(count: i64, connection: &PgConnection) -> Result<Vec<models::Block>, diesel::result::Error> {
    use schema::blocks::dsl::*;

    blocks.order_by(block_num.desc()).limit(count).load::<models::Block>(connection)
}

pub fn fetch_block(block_num_param: i64, connection: &PgConnection) -> Result<Option<models::Block>, diesel::result::Error> {
    use schema::blocks::dsl::*;

    blocks.filter(block_num.eq(block_num_param)).select((block_num, block_id)).first(connection).optional()
}

pub fn fetch_balance(account_name: String, account_number: u32, connection: &PgConnection) -> QueryResult<i32> {
    use schema::accounts::dsl::*;

    let account_number = i32::try_from(account_number).expect("Error converting account number from u32 to i32");

    accounts.filter(name.eq(account_name).and(number.eq(account_number))).select(balance).first(connection)
}

pub fn insert_account(account: models::NewAccount, connection: &PgConnection) -> QueryResult<usize> {
    use schema::accounts::dsl::*;

    diesel::update(accounts.filter(end_block_num.eq(account.end_block_num)))
        .set(end_block_num.eq(account.start_block_num))
        .execute(connection)?;
    diesel::insert_into(accounts).values(&account).execute(connection)
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_init_pool() {}

    #[test]
    fn test_establish_connection() {}

    #[test]
    fn test_commit() {}
}