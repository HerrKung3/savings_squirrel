use sqlx::MySqlPool;
use crate::errors::MyError;
use crate::models::user::{CreateUser, UpdateUser, User};

pub async fn get_user_details_db(pool: &MySqlPool, telephone: String) -> Result<User, MyError> {
    let row = sqlx::query_as!(
        User,
        "SELECT * FROM user WHERE telephone = ?",
        telephone
    )
        .fetch_one(pool)
        .await
        .map_err(|_|MyError::NotFound("No user found".to_string()))?;

    Ok(row)
}

pub async fn create_user_db(pool: &MySqlPool, new_user: CreateUser) ->Result<(), MyError> {
    let _post_row = sqlx::query!(
        "INSERT INTO user (name, telephone, password, ledger, subscriber_type, email, wechat)\
        VALUE (?, ?, ?, ?, ?, ?, ?)",
        new_user.name,
        new_user.telephone,
        new_user.password,
        new_user.ledger,
        new_user.subscriber_type,
        new_user.email,
        new_user.wechat,
    )
        .execute(pool)
        .await
        .map_err(|_|MyError::DBError("Failed to create user".to_string()))?;

    Ok(())
}

pub async fn update_user_db(pool: &MySqlPool, update_user: UpdateUser, telephone: String) -> Result<(), MyError> {
    let row = sqlx::query_as!(
        User,
        "SELECT * FROM user WHERE telephone = ?",
        telephone
    )
        .fetch_one(pool)
        .await
        .map_err(|_|MyError::NotFound("User is NOT found".into()))?;

    let temp = User {
        id: row.id,
        name: if let Some(name) = update_user.name {
            name
        }else {
            row.name
        },
        telephone: if let Some(telephone) = update_user.new_telephone {
            telephone
        }else {
            row.telephone
        },
        password: if let Some(password) = update_user.new_password {
            password
        }else {
            row.password
        },
        ledger: if let Some(ledgers) = update_user.ledger {
            ledgers
        }else {
            row.ledger
        },
        subscriber_type: if let Some(subscriber_type) = update_user.subscriber_type {
            subscriber_type
        }else {
            row.subscriber_type
        },
        email: if let Some(email) = update_user.email {
            Some(email)
        }else {
            row.email
        },
        wechat: if let Some(wechat) = update_user.wechat {
            Some(wechat)
        }else {
            row.wechat
        },
    };

    let _update_row = sqlx::query!(
        "UPDATE user \
        SET name=?, telephone=?, password=?, ledger=?, subscriber_type=?, email=?, wechat=? \
        WHERE id = ?",
        temp.name,
        temp.telephone,
        temp.password,
        temp.ledger,
        temp.subscriber_type,
        temp.email,
        temp.wechat,
        temp.id,
    )
        .execute(pool)
        .await
        .map_err(|_|MyError::DBError("Update user failed".into()))?;

    Ok(())
}

pub async fn delete_user_db(pool: &MySqlPool, telephone: String) -> Result<(), MyError> {
    let _row = sqlx::query!("DELETE FROM user WHERE telephone = ?", telephone)
        .execute(pool)
        .await
        .map_err(|_|MyError::DBError("Unable to delete user".to_string()))?;

    Ok(())
}