use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use tracing::{error};

use super::auth::{encrypt_private_key, hash_password, verify_password};
use super::messenger::Messenger;
use super::AppData;
use database::models::NewCredentials;
use database::{fetch_auth, fetch_balance, insert_auth};

#[derive(Deserialize)]
pub struct AccountData {
    name: String,
    number: u32,
}

#[derive(Deserialize)]
pub struct MerchantData {
    name: String,
    password: String,
}

#[derive(Deserialize)]
pub struct AuthData {
    public_key: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UpdateBalanceJson {
    name: String,
    number: u32,
    amount: i32,
}

// TODO attribute ID to every request

pub async fn authenticate(
    app_data: web::Data<AppData>,
    auth_data: web::Json<AuthData>,
) -> impl Responder {
    let pool = &app_data.pool;

    let connection = pool.get().expect("Could not get connection from pool");

    let public_key = auth_data.public_key.to_owned();
    let password = auth_data.password.to_owned();
    let auth = web::block(move || fetch_auth(public_key, &*connection))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Could not find credentials associated with that public key");
    let is_match =
        verify_password(password, auth.hashed_password).expect("Error verifying password");
    if is_match == false {
        HttpResponse::Unauthorized().body("Passwords did not match")
    } else {
        // TODO Generate and return {'authorization': token} (?)
        HttpResponse::Ok().body("OK")
    }
}

pub async fn deposit(
    _app_data: web::Data<AppData>,
    account_data: web::Json<UpdateBalanceJson>,
) -> impl Responder {
    let messenger: Messenger = Messenger::new("secp256k1");

    // TODO change to auth
    let (_public_key, private_key): (String, String) = messenger.get_new_key_pair();

    messenger
        .send_deposit_txn(
            &private_key,
            account_data.name.to_owned(),
            account_data.number,
            account_data.amount,
        )
        .await;

    HttpResponse::Ok().json("Deposit transaction submitted to validator")
}

pub async fn withdraw(
    _app_data: web::Data<AppData>,
    account_data: web::Json<UpdateBalanceJson>,
) -> impl Responder {
    let messenger: Messenger = Messenger::new("secp256k1");

    // TODO change to auth
    let (_public_key, private_key): (String, String) = messenger.get_new_key_pair();

    messenger
        .send_withdraw_txn(
            &private_key,
            account_data.name.to_owned(),
            account_data.number,
            account_data.amount,
        )
        .await;

    HttpResponse::Ok().json("Withdraw transaction submitted to validator")
}

pub async fn add_account(
    _app_data: web::Data<AppData>,
    account_data: web::Json<AccountData>,
) -> impl Responder {
    let messenger: Messenger = Messenger::new("secp256k1");

    // TODO change to auth
    let (_public_key, private_key): (String, String) = messenger.get_new_key_pair();

    messenger
        .send_add_account_txn(
            &private_key,
            account_data.name.to_owned(),
            account_data.number,
        )
        .await;

    HttpResponse::Ok().json("Add account transaction submitted to validator")
}

pub async fn add_merchant(
    app_data: web::Data<AppData>,
    merchant_data: web::Json<MerchantData>,
) -> impl Responder {
    let messenger: Messenger = Messenger::new("secp256k1");

    let pool = &app_data.pool;

    let connection = pool.get().expect("Could not get connection from pool");

    let (public_key, private_key): (String, String) = messenger.get_new_key_pair();

    let date_time = chrono::offset::Utc::now();
    messenger
        .send_add_merchant_txn(
            &private_key,
            merchant_data.name.to_owned(),
            date_time.timestamp(),
        )
        .await;

    let hashed_password =
        hash_password(merchant_data.password.to_owned()).expect("Could not hash password");

    let encrypted_private_key = encrypt_private_key(public_key.clone(), private_key);

    let credentials = NewCredentials {
        public_key: &public_key,
        hashed_password: &hashed_password,
        encrypted_private_key: &encrypted_private_key,
    };

    let _result = insert_auth(credentials, &*connection);

    // TODO generate and return {'authorization': token}
    HttpResponse::Ok().json("Add merchant transaction submitted to validator")
}

pub async fn get_balance(
    app_data: web::Data<AppData>,
    account_data: web::Json<AccountData>,
) -> Result<HttpResponse, Error> {
    // TODO add authorization GUARD
    // (don't allow just anyone to reach this route, but the private key does not need to be used)
    let pool = &app_data.pool;

    let connection = pool.get().expect("Could not get connection from pool");

    let name = account_data.name.clone();
    let number = account_data.number;

    let balance = web::block(move || fetch_balance(name, number, &*connection))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        });

    match balance {
        Ok(balance) => Ok(HttpResponse::Ok().json(balance)),
        Err(_) => {
            let res = HttpResponse::NotFound().body(format!(
                "No account found with name and number: {}, {}",
                account_data.name, account_data.number
            ));
            Ok(res)
        }
    }
}

pub async fn health_check(_request: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
