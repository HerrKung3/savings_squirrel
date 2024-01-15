use actix_web::{HttpResponse, web, web::Path};
use crate::errors::MyError;
use crate::state::AppState;
use crate::dbaccess::user::*;
use crate::models::user::{CreateUser, UpdateUser};

pub async fn get_user_details(
    app_state: web::Data<AppState>,
    telephone: Path<String>,
) -> Result<HttpResponse, MyError> {
    get_user_details_db(&app_state.db, telephone.into_inner())
        .await
        .map(|user|HttpResponse::Ok().json(user))
}

pub async fn create_user(
    app_state: web::Data<AppState>,
    new_user: web::Json<CreateUser>,
) -> Result<HttpResponse, MyError> {
    //check pwd and tel format
    if new_user.password.len() < 6 {
        return Err(MyError::InvalidInput("Invalid password format".to_string()));
    }
    if new_user.telephone.len() != 11 {
        return Err(MyError::InvalidInput("Invalid telephone format".to_string()));
    }

    //query database to check the use if already exists
    let user_exists = match get_user_details_db(&app_state.db, new_user.telephone.clone()).await {
        Ok(_) => true,
        Err(_) => false,
    };
    if user_exists {
        return Err(MyError::InvalidInput("Telephone is already registered".to_string()));
    }

    //register user
    create_user_db(&app_state.db, new_user.into())
        .await
        .map(|_|HttpResponse::Ok().json("Create user successfully"))
}

pub async fn update_user(
    app_state: web::Data<AppState>,
    update_user: web::Json<UpdateUser>,
    telephone: Path<String>,
) -> Result<HttpResponse, MyError> {
    //deserialize
    let update_user: UpdateUser = update_user.into();

    //check new pwd and tel format
    if let Some(new_pwd) = update_user.new_password.clone() {
        if new_pwd.len() < 6 {
            return Err(MyError::InvalidInput("Invalid new password format".to_string()));
        }
    }
    if let Some(new_tel) = update_user.new_telephone.clone() {
        if new_tel.len() != 11 {
            return Err(MyError::InvalidInput("Invalid new telephone format".to_string()));
        }
    }

    //TODO: if it's update telephone or email, we need to send code to old telephone or email

    //query database to check the password correctness
    let telephone = telephone.into_inner();
    let user = get_user_details_db(&app_state.db, telephone.clone()).await?;
    if user.password != update_user.old_password {
       return Err(MyError::InvalidInput("Password is NOT correct".to_string()));
    }

    //update user information
    update_user_db(&app_state.db, update_user, telephone)
        .await
        .map(|_|HttpResponse::Ok().json("Update user successfully"))
}

pub async fn delete_user(
    app_state: web::Data<AppState>,
    telephone: Path<String>,
    pwd: web::Json<String>,
) -> Result<HttpResponse, MyError> {
    //check pwd and tel format
    let pwd = pwd.into_inner();
    if pwd.len() < 6 {
        return Err(MyError::InvalidInput("Invalid password format".to_string()));
    }
    let telephone = telephone.into_inner();
    if telephone.len() != 11 {
        return Err(MyError::InvalidInput("Invalid telephone format".to_string()));
    }

    //query database to check the password correctness
    let user = get_user_details_db(&app_state.db, telephone.clone()).await?;
    if user.password != pwd {
        return Err(MyError::InvalidInput("Password is NOT correct".to_string()));
    }

    //delete user information
    delete_user_db(&app_state.db, telephone)
        .await
        .map(|_|HttpResponse::Ok().json("Delete user successfully"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use actix_web::http::StatusCode;
    use actix_web::web;
    use dotenv::dotenv;
    use sqlx::encode::IsNull::No;
    use sqlx::mysql::MySqlPoolOptions;
    use crate::state::AppState;

    //pass
    #[ignore]
    #[actix_rt::test]
    async fn get_user_details_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            db: db_pool,
        });
        let tel = web::Path::from("18570771568".to_string());

        let resp = get_user_details(app_state, tel).await.unwrap();

        println!("response body = {:?}", resp.body());
        assert_eq!(resp.status(), StatusCode::OK);
    }

    //pass with postman
    #[ignore]
    #[actix_rt::test]
    async fn create_user_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            db: db_pool,
        });
        let new_user = web::Json(CreateUser{
            name: "Haydn Kong".to_string(),
            telephone: "18570771568".to_string(),
            password: "hansiyuanshidameinv".to_string(),
            ledger: "daily".to_string(),
            subscriber_type: "Not".to_string(),
            email: None,
            wechat: Some("18301633792".to_string())
        });

        let resp = create_user(app_state, new_user).await.unwrap();

        println!("response body = {:?}", resp.body());
        assert_eq!(resp.status(), StatusCode::OK);
    }

    //pass
    #[ignore]
    #[actix_rt::test]
    async fn create_user_failed() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            db: db_pool,
        });
        let new_user = web::Json(CreateUser{
            name: "kong qiang".to_string(),
            telephone: "18111354101".to_string(),
            password: "hansi".to_string(),
            ledger: "daily".to_string(),
            subscriber_type: "Not".to_string(),
            email: None,
            wechat: Some("18301633792".to_string())
        });

        let resp = create_user(app_state, new_user).await.unwrap();

        println!("response body = {:?}", resp.body());
        assert_ne!(resp.status(), StatusCode::OK);
    }

    //pass
    #[ignore]
    #[actix_rt::test]
    async fn delete_user_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            db: db_pool,
        });
        let telephone = web::Path::from("15103954542".to_string());
        let pwd = web::Json("hansiyuanshidameinv".to_string());

        let resp = delete_user(app_state, telephone, pwd).await.unwrap();

        println!("response body = {:?}", resp.body());
        assert_eq!(resp.status(), StatusCode::OK);
    }

    //pass
    #[ignore]
    #[actix_rt::test]
    async fn delete_user_failed() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            db: db_pool,
        });
        let telephone = web::Path::from("18570771568".to_string());
        let pwd = web::Json("hansiyuanshidamein".to_string());

        let resp = delete_user(app_state, telephone, pwd).await.unwrap();

        println!("response body = {:?}", resp.body());
        assert_ne!(resp.status(), StatusCode::OK);
    }

    //pass
    #[ignore]
    #[actix_rt::test]
    async fn update_user_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            db: db_pool,
        });
        let telephone = web::Path::from("18570771568".to_string());
        let user = web::Json(UpdateUser{
            name: Some("haydn".to_string()),
            new_password: Some("654321".to_string()),
            old_password: "123456".to_string(),
            new_telephone: None,
            ledger: Some("business".to_string()),
            subscriber_type: None,
            email: None,
            wechat: None,
        });

        let resp = update_user(app_state, user, telephone).await.unwrap();

        println!("response body = {:?}", resp.body());
        assert_eq!(resp.status(), StatusCode::OK);
    }
}