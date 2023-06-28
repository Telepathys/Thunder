
use actix_web::{get, post, HttpResponse, Responder};
use serde_json::json;
use crate::utils::jwt::{
    create_jwt,
};
use log::{info,error};
use crate::database::mongo::user::users::{
    user_join,
    user_login,
    find_by_name,
};
use crate::game::components::user::user_component::{
    Join,
    Login,
    TokenInput,
    User,
};

#[get("/asd")]
pub async fn asd() -> impl Responder {
    // let uuid = Uuid::new_v4();
    // insert_document().await;
    // let get_user = find_by_name().await;

    // let result = fetch_an_integer().await;
    // match result {
    //     Ok(value) => {
    //         println!("Fetched value: {}", value);
    //     }
    //     Err(error) => {
    //         eprintln!("Failed to fetch value: {}", error);
    //     }
    // }
    HttpResponse::Ok().body("get_user1".to_string())
}


#[post("/join")]
async fn join(req_body: String) -> impl Responder {
    let request: Join = match serde_json::from_str(&req_body) {
        Ok(body) => body,
        Err(err) => {
            error!("Failed to parse JSON: {}", err);
            let error_message = json!({ "error": "Failed to parse JSON"});
            return HttpResponse::BadRequest().json(error_message);
        }
    };
    let name = request.name.clone();
    let get_user = find_by_name(&name).await;
    if get_user.is_empty() {
        user_join(request).await;
    } else {
        error!("Failed to parse JSON: {}", "이미 존재하는 유저입니다.");
        let error_message = json!({ "error": "이미 존재하는 유저입니다."});
        return HttpResponse::BadRequest().json(error_message);
    }

    HttpResponse::Ok().body(format!("Received POST request with id: {}", "asd"))
}
#[post("/login")]
pub async fn login(req_body: String) -> impl Responder {
    let request: Login = match serde_json::from_str(&req_body) {
        Ok(body) => body,
        Err(err) => {
            error!("Failed to parse JSON: {}", err);
            let error_message = json!({ "error": "Failed to parse JSON"});
            return HttpResponse::BadRequest().json(error_message);
        }
    };
    let user_data = user_login(request).await;
    if user_data.is_empty() {
        error!("Failed to parse JSON: {}", "존재하지 않는 유저");
        let error_message = json!({ "error": "존재하지 않는 유저"});
        return HttpResponse::BadRequest().json(error_message);
    }

    let uuid = user_data.get("uuid").unwrap().to_string().trim_matches('"').to_string();
    let id = user_data.get("id").unwrap().to_string().trim_matches('"').to_string();
    let name = user_data.get("name").unwrap().to_string().trim_matches('"').to_string();

    let token = create_jwt(
        TokenInput {
            uuid: uuid.clone(),
            id: id.clone(),
            name: name.clone(),
        }
    );

    let user = User {
        id: id,
        uuid: uuid,
        name: name,
        token: token,
    };

    info!("{}이 로그인에 성공하였습니다.", user.name);
    HttpResponse::Ok().json(user)
}
