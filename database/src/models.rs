use super::schema::{accounts, auth, blocks, merchants};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

#[derive(Clone, Debug, Queryable)]
pub struct Account {
    pub name: String,
    pub number: u32,
    pub balance: i32,
    pub start_block_num: Option<i64>,
    pub end_block_num: Option<i64>,
}

#[derive(AsChangeset, Clone, Debug, Insertable)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub number: i32,
    pub balance: i32,
    pub start_block_num: Option<i64>,
    pub end_block_num: Option<i64>,
}

#[derive(Clone, Debug, Queryable)]
pub struct Credentials {
    pub public_key: String,
    pub hashed_password: String,
    pub encrypted_private_key: String,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "auth"]
pub struct NewCredentials<'a> {
    pub public_key: &'a str,
    pub hashed_password: &'a str,
    pub encrypted_private_key: &'a str,
}

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
pub struct Merchant {
    pub public_key: String,
    pub name: String,
    pub created: NaiveDateTime,
    pub start_block_num: Option<i64>,
    pub end_block_num: Option<i64>,
}

#[derive(AsChangeset, Clone, Debug, Insertable)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "merchants"]
pub struct NewMerchant<'a> {
    pub public_key: &'a str,
    pub name: &'a str,
    pub created: &'a NaiveDateTime,
    pub start_block_num: Option<i64>,
    pub end_block_num: Option<i64>,
}
