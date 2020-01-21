#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod payload;

use rocket_contrib::json::{JsonValue};
use rocket::request::{LenientForm};

use payload::{Payload, SlackText};

#[get("/coffee/add")]
fn add_coffee_get() -> &'static str {
    "type '/add_coffee artist-title'"
}

#[post("/coffee/start", data="<payload>")]
fn start_coffee_post(payload: LenientForm<Payload>) -> JsonValue {
    match &payload.text {
        Some(slack_text) => {
            let SlackText(text) = slack_text;
            
            json!({
                "blocks": [
                    {
                        "type": "section",
                        "text": {
                            "type": "mrkdwn",
                            "text": "*It's 80 degrees right now.*"
                        }
                    },
                    {
                        "type": "section",
                        "text": {
                            "type": "mrkdwn",
                            "text": text
                        }
                    }
                ]
            })
        },
        None => json!({
            "response_type": "ephemeral",
            "text": "Sorry, that didn't work. Please try again."
        })
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![add_coffee_get, start_coffee_post])
        .launch();
}