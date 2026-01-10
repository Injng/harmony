use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

use crate::db::user;
use crate::db::user::Entity as User;

use super::auth::{auth_check_and_decode_hex, auth_encrypt, auth_verify};

pub async fn auth_check_user(
    username: &str,
    token: &str,
    salt: &str,
    key: &str,
    db: &DatabaseConnection,
) -> Result<()> {
    // check if user with username exists
    let user: Option<user::Model> = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await?;

    // attempt to verify the password
    if let Some(u) = user {
        if auth_verify(&u.password, token, salt, key, &u.nonce) {
            return Ok(());
        } else {
            return Err(anyhow!("[ERROR] Incorrect password for user"));
        }
    } else {
        return Err(anyhow!("[ERROR] User does not exist in database"));
    }
}

pub async fn auth_create_user(
    username: &str,
    password: &str,
    email: &str,
    key: &str,
    db: &DatabaseConnection,
) -> Result<()> {
    // handle hex-encoded strings
    let mut dec_password: String = password.to_owned();
    dec_password = auth_check_and_decode_hex(&dec_password)?;

    // encrypt password based on provided key
    let (enc_password, nonce_str) = auth_encrypt(&dec_password, key)?;

    // if no other users in database, the first user is admin
    let mut is_admin = false;
    let users = User::find().all(db).await?.len();
    if users < 1 {
        is_admin = true;
    }

    // insert the user into the database
    let dt: DateTime<Utc> = Utc::now();
    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(username.to_owned()),
        email: Set(email.to_owned()),
        password: Set(enc_password),
        nonce: Set(nonce_str),
        is_admin: Set(is_admin),
        created_at: Set(dt),
    };
    let _ = user.insert(db).await?;
    Ok(())
}
