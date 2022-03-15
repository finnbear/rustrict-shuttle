#[macro_use]
extern crate shuttle_service;

#[macro_use]
extern crate rocket;

use rocket::response::content::{Html, Json};
use rocket::{Build, Rocket};
use serde::{Deserialize, Serialize};

#[get("/")]
fn index() -> Html<&'static str> {
    Html(
        r###"
<!DOCTYPE html>
<head>
<title>Rustrict on Shuttle</title>
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@3.3.7/dist/css/bootstrap.min.css" integrity="sha384-BVYiiSIFeK1dGmJRAkycuHAHRg32OmUcww7on3RYdg4Va+PmSTsz/K68vbdEjh4u" crossorigin="anonymous">
</head>
<body style="padding: 1em;">
<h1>Rustrict Shuttle Api</h1>
<a href="https://crates.io/crates/rustrict">Rustrict</a> + <a href="https://www.shuttle.rs/">Shuttle</a> = <a href="https://github.com/finnbear/rustrict-shuttle">Rustrict Shuttle</a>
<h2>Input</h2>
<input id="input" class="form-control" oninput="censor()" type="text"/>
<h2>Analysis</h2>
<p id="analysis"></p>
<h2>Output</h2>
<textarea id="output" class="form-control" rows="10" readonly></textarea>
<script>
async function censor() {
	const req = {text: document.getElementById("input").value};

	let response = await fetch("/", {
		method: "POST",
		body: JSON.stringify(req),
		headers: {
			"Content-Type": "application/json"
		}
	});
	let resp = await response.json();
        document.getElementById("analysis").innerHTML = resp.analysis;
	document.getElementById("output").innerHTML = resp.censored;
}
censor();
</script>
</body>
"###,
    )
}

#[derive(Default, Deserialize)]
struct Req {
    text: String,
}

#[derive(Serialize)]
struct Resp {
    censored: String,
    analysis: String,
}

#[post("/", data = "<req>")]
fn censor(req: String) -> Json<String> {
    use rustrict::Censor;

    let req: Req = serde_json::from_str(&req).unwrap_or_default();

    let (censored, analysis) = Censor::from_str(&req.text).censor_and_analyze();

    let resp = Resp {
        censored,
        analysis: format!("{:?}", analysis),
    };

    Json(serde_json::to_string(&resp).unwrap())
}

fn init() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, censor])
}

declare_service!(Rocket<Build>, init);
