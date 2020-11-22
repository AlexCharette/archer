use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use super::messenger::Messenger;
use super::AppData;
use database::{fetch_auth, fetch_balance, PgPool};

#[derive(Deserialize)]
struct AccountData {
    name: String,
    number: u32,
}

#[derive(Deserialize)]
struct MerchantData {
    name: String,
    public_key: String,
    password: String,
}

#[derive(Deserialize)]
struct AuthData {
    public_key: String,
    password: String,
}

#[derive(Deserialize)]
struct UpdateBalanceJson {
    name: String,
    number: u32,
    amount: i32,
}

pub async fn authenticate(
    app_data: web::Data<AppData>,
    merchant_data: web::Json<MerchantData>,
) -> impl Responder {
    let secret_key = app_data.secret_key;
    let pool = app_data.pool;
    /*
        body = await decode_request(request)
        required_fields = ['public_key', 'password']
        validate_fields(required_fields, body)

        password = bytes(body.get('password'), 'utf-8')

        auth_info = await self._database.fetch_auth_resource(
            body.get('public_key'))
        if auth_info is None:
            raise ApiUnauthorized('No agent with that public key exists')

        hashed_password = auth_info.get('hashed_password')
        if not bcrypt.checkpw(password, bytes.fromhex(hashed_password)):
            raise ApiUnauthorized('Incorrect public key or password')

        token = generate_auth_token(
            request.app['secret_key'], body.get('public_key'))

        return json_response({'authorization': token})
    */
    let connection = pool.get().expect("Could not get connection from pool");

    let auth = web::block(move || fetch_auth(name, number, &*connection))
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
    HttpResponse::Ok().body("Payload received") // TODO
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

    HttpResponse::Ok().body("Payload received") // TODO
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

    HttpResponse::Ok().body("Payload received") // TODO
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

    HttpResponse::Ok().body("Payload received") // TODO
}

pub async fn add_merchant(
    _app_data: web::Data<AppData>,
    merchant_data: web::Json<MerchantData>,
) -> impl Responder {
    let messenger: Messenger = Messenger::new("secp256k1");

    let (public_key, private_key): (String, String) = messenger.get_new_key_pair();

    messenger
        .send_add_merchant_txn(
            &private_key,
            merchant_data.name.to_owned(),
            NaiveDateTime::timestamp(),
        )
        .await;

    /*
        encrypted_private_key = encrypt_private_key(
            request.app['aes_key'], public_key, private_key)
        hashed_password = hash_password(body.get('password'))

        await self._database.create_auth_entry(
            public_key, encrypted_private_key, hashed_password)

        token = generate_auth_token(
            request.app['secret_key'], public_key)

        return json_response({'authorization': token})
    */

    HttpResponse::Ok().body("Payload received") // TODO
}

pub async fn get_balance(
    app_data: web::Data<AppData>,
    account_data: web::Json<AccountData>,
) -> Result<HttpResponse, Error> {
    // TODO add authorization GUARD
    // (don't allow just anyone to reach this route, but the private key does not need to be used)
    let pool = app_data.pool;

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

pub async fn health_check(request: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
