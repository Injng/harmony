use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

use crate::db::user;
use crate::db::user::Entity as User;

use super::auth::auth_encrypt;

pub async fn auth_create_user<'a>(
    username: &str,
    password: &str,
    email: &str,
    key: &str,
    db: &DatabaseConnection,
) -> Result<()> {
    // handle hex-encoded strings
    let mut dec_password: String = password.to_owned();
    if &password[0..4] == "enc:" {
        if let Ok(v) = hex::decode(&password[4..]) {
            if let Ok(p) = String::from_utf8(v) {
                dec_password = p;
            } else {
                return Err(anyhow!("[ERROR] Invalid hex-encoded password"));
            }
        }
    }

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
