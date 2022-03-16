#[macro_use]
extern crate shuttle_service;

#[macro_use]
extern crate rocket;

use rocket::response::content::{Html, Json};
use rocket::{Build, Rocket};
use serde::{Deserialize, Serialize};

#[get("/")]
fn index() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

#[derive(Default, Deserialize)]
struct Req {
    text: String,
}

#[derive(Serialize)]
struct Resp {
    censored: String,
    analysis: String,
    width: usize,
}

#[post("/", data = "<req>")]
fn censor(req: String) -> Json<String> {
    use rustrict::Censor;

    let req: Req = serde_json::from_str(&req).unwrap_or_default();

    let (censored, analysis) = Censor::from_str(&req.text).censor_and_analyze();
    let width = rustrict::width_str(&req.text);

    let resp = Resp {
        censored,
        analysis: format!("{:?}", analysis),
        width,
    };

    Json(serde_json::to_string(&resp).unwrap())
}

fn init() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, censor])
}

declare_service!(Rocket<Build>, init);
