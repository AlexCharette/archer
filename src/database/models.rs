
use diesel::{Insertable, Queryable};
use super::schema::{accounts, blocks};

#[derive(Clone, Debug, Queryable)]
pub struct Block {
    pub block_num: i64,
    pub block_id: String,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "blocks"]
pub struct NewBlock<'a> {
    pub block_num: i64,
    pub block_id: &'a str,
}

#[derive(Clone, Debug, Queryable)]
pub struct Account {
    pub name: String,
    pub number: u32,
    pub balance: i32,
    pub start_block_num: Option<i64>,
    pub end_block_num: Option<i64>,
}

#[derive(AsChangeset, Clone, Debug, Insertable)]
#[changeset_options(treat_none_as_null="true")]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub number: i32,
    pub balance: i32,
    pub start_block_num: Option<i64>,
    pub end_block_num: Option<i64>,
}