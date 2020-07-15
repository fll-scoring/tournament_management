use actix_web::{get, post, HttpResponse, web, HttpRequest};
use bson::{Bson, doc, Document};
use crate::models::*;
use fll_scoring::data::get_mongo_database;

#[post("/api/teams/")]
pub async fn new_team(info: web::Json<Team>) -> HttpResponse {
    /*!
     * Handles adding new teams programmatically.
     */
    let db = match get_mongo_database().await {
        Ok(d) => d,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("MongoDB: {:?}", err));
        }
    };
    let collection = db.collection("teams");
    let doc = doc! { "number": info.number.clone(), "name": info.name.clone(), "affiliation": info.affiliation.clone() };
    if let Err(err) = collection.insert_one(doc, None).await {
        return HttpResponse::InternalServerError().body("MongoDB");
    }
    HttpResponse::Ok().json(info.0)
}

#[get("/api/teams/{number}")]
async fn get_team(req: HttpRequest) -> HttpResponse {
    /*!
     * Gets a new team from the API
     */
    let team_number_str = match req.match_info().get("number") {
        Some(num) => num,
        None => {
            return HttpResponse::BadRequest().body("No Team number specified.");
        }
    };
    let team_number = match team_number_str.parse::<u32>() {
        Ok(n) => n,
        Err(_err) => {
            return HttpResponse::BadRequest().body("Invalid Team number provided");
        }
    };

    let db = match get_mongo_database().await {
        Ok(d) => d,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("MongoDB: {:?}", err));
        }
    };
    let collection = db.collection("teams");
    let doc = doc! {"number": team_number};
    let result = match collection.find_one(doc, None).await {
        Ok(opt) => match opt {
            Some(doc) => doc,
            None => {
                return HttpResponse::NotFound().body("No team found");
            }
        },
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("MongoDB: {:?}", err));
        }
    };
    let res_team_number = match result.get_i32("number") {
        Ok(num) => num as u32,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("number BSON: {:?}", err));
        }
    };
    let res_team_name = match result.get_str("name") {
        Ok(name) => name,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("name BSON: {:?}", err));
        }
    };
    let res_team_affiliation = match result.get_str("affiliation") {
        Ok(aff) => aff,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("affil BSON: {:?}", err));
        }
    };

    let json = web::Json(Team {
        number: res_team_number,
        name: res_team_name.to_string(),
        affiliation: res_team_affiliation.to_string(),
    });
    HttpResponse::Ok().json(json.0)
}
