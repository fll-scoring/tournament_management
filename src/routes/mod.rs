use crate::models::*;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use bson::{doc, Bson, Document};
use fll_scoring::data::{get_mongo_database, get_next_mongo_sequence_number};
use futures::stream::StreamExt;

#[post("/api/teams/")]
pub async fn new_team(info: web::Form<NewTeam>) -> HttpResponse {
    /*!
     * Handles adding new teams programmatically.
     */
    let db = match get_mongo_database().await {
        Ok(d) => d,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("MongoDB: {:?}", err));
        }
    };
    let team_number = match info.number.parse::<u32>() {
        Ok(num) => num,
        Err(err) => {
            return HttpResponse::BadRequest().body("Invalid Team number");
        }
    };
    let collection = db.collection("teams");
    let is_unique = match collection
        .find(doc! {"number": team_number.clone()}, None)
        .await
    {
        Ok(mut curs) => {
            if let Some(_) = curs.next().await {
                false
            } else {
                true
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("MongoDB: {:?}", err));
        }
    };
    if !is_unique {
        return HttpResponse::BadRequest().body("This team already exists");
    }
    let doc = doc! { "number": team_number.clone(), "name": info.name.clone(), "affiliation": info.affiliation.clone() };
    if let Err(err) = collection.insert_one(doc, None).await {
        return HttpResponse::InternalServerError().body("MongoDB");
    }
    let json = web::Json(Team {
        number: team_number,
        name: info.name.clone(),
        affiliation: info.affiliation.clone(),
    });
    HttpResponse::Ok().json(json.0)
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

#[post("/api/tournaments/")]
pub async fn new_tournament(info: web::Form<NewTournament>) -> HttpResponse {
    let db = match get_mongo_database().await {
        Ok(d) => d,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("MongoDB: {:?}", err));
        }
    };
    let collection = db.collection("tournaments");
    let counter_collection = db.collection("counter");
    let new_id =
        match get_next_mongo_sequence_number("tournaments".to_string(), counter_collection).await {
            Ok(id) => id as u32,
            Err(err) => {
                return HttpResponse::InternalServerError().body(format!("MongoDB: {:?}", err));
            }
        };
    let doc = doc! {"id": new_id, "name": info.name.clone(), "teams": info.teams.clone(), "current_stage": info.current_stage.clone()};
    if let Err(err) = collection.insert_one(doc, None).await {
        return HttpResponse::InternalServerError().body(format!("MongoDB: {:?}", err));
    }
    let json = web::Json(Tournament {
        id: new_id,
        name: info.name.clone(),
        teams: info.teams.clone(),
        current_stage: info.current_stage.clone(),
    });
    HttpResponse::Ok().json(json.0)
}

#[get("/api/tournaments/{id}")]
pub async fn get_tournament(req: HttpRequest) -> HttpResponse {
    let tournament_id_str = match req.match_info().get("id") {
        Some(num) => num,
        None => {
            return HttpResponse::BadRequest().body("No team number specified");
        }
    };
    let tournament_id = match tournament_id_str.parse::<u32>() {
        Ok(n) => n,
        Err(_err) => {
            return HttpResponse::BadRequest().body("Invalid tournament ID");
        }
    };
    let db = match get_mongo_database().await {
        Ok(d) => d,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("MongoDB: {:?}", err));
        }
    };
    let collection = db.collection("tournaments");
    let filter = doc! {"id": tournament_id};
    let result = match collection.find_one(filter, None).await {
        Ok(opt) => match opt {
            Some(doc) => doc,
            None => {
                return HttpResponse::NotFound().body("No tournament found with that ID.");
            }
        },
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("MongoDB: {:?}", err));
        }
    };
    let res_id = tournament_id.clone();
    let res_tournament_name = match result.get_str("name") {
        Ok(name) => name.to_string(),
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("BSON: {:?}", err));
        }
    };
    let mut res_teams: Vec<u32> = Vec::new();
    match result.get("teams") {
        Some(doc) => match doc {
            Bson::Array(arr) => {
                for bs in arr {
                    match bs {
                        Bson::Int32(num) => {
                            res_teams.push(*num as u32);
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        },
        _ => (),
    }
    let current_stage = match result.get_str("current_stage") {
        Ok(stage) => stage.to_string(),
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("BSON: {:?}", err));
        }
    };

    let json = web::Json(Tournament {
        id: res_id,
        name: res_tournament_name,
        teams: res_teams,
        current_stage,
    });

    HttpResponse::Ok().json(json.0)
}
