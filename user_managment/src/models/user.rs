use actix_web::web;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub telephone: String,
    pub password: String,
    pub ledger: String,
    pub subscriber_type: String,
    pub email: Option<String>,
    pub wechat: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub new_telephone: Option<String>,
    pub new_password: Option<String>,
    pub old_password: String,
    pub ledger: Option<String>,
    pub subscriber_type: Option<String>,
    pub email: Option<String>,
    pub wechat: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub telephone: String,
    pub password: String,
    pub ledger: String,
    pub subscriber_type: String,
    pub email: Option<String>,
    pub wechat: Option<String>,
}

//web::JSON to CreateUser
impl From<web::Json<CreateUser>> for CreateUser {
    fn from(create_user: web::Json<CreateUser>) -> Self {
        CreateUser {
            name: create_user.name.clone(),
            telephone: create_user.telephone.clone(),
            password: create_user.password.clone(),
            ledger: create_user.ledger.clone(),
            subscriber_type: create_user.subscriber_type.clone(),
            email: create_user.email.clone(),
            wechat: create_user.wechat.clone()
        }
    }
}

//web::JSON to UpdateUser
impl From<web::Json<UpdateUser>> for UpdateUser {
    fn from(update_user: web::Json<UpdateUser>) -> Self {
        UpdateUser {
            name: update_user.name.clone(),
            new_telephone: update_user.new_telephone.clone(),
            new_password: update_user.new_password.clone(),
            old_password: update_user.old_password.clone(),
            ledger: update_user.ledger.clone(),
            subscriber_type: update_user.subscriber_type.clone(),
            email: update_user.email.clone(),
            wechat: update_user.wechat.clone(),
        }
    }
}

//web::JSON to User
impl From<web::Json<User>> for User {
    fn from(user: web::Json<User>) -> Self {
        User {
            id: user.id,
            name: user.name.clone(),
            telephone: user.telephone.clone(),
            password: user.password.clone(),
            ledger: user.ledger.clone(),
            subscriber_type: user.subscriber_type.clone(),
            email: user.email.clone(),
            wechat: user.wechat.clone(),
        }
    }
}
