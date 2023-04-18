#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Serialize, Deserialize)]
struct Reminder {
	id: i32,
	title: String,
	description: String,
	date: String,
}

#[get("/")]
fn index() -> Template {
	let context = json!({
		"title": "r.me | Home"
	});
	Template::render("index", &context)
}

#[get("/reminders")]
fn reminders() -> Template {
    let url = "mysql://root:password@localhost:3306/jackreminders";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let reminders_data: Vec<Reminder> = conn.query_map(
        "SELECT * FROM reminders",
        |(id, title, description, date)| Reminder {
            id,
            title,
            description,
            date,
        },
    )
    .unwrap()
    .iter()
    .map(|reminder| {
        let date_time = NaiveDateTime::parse_from_str(&reminder.date, "%Y-%m-%d %H:%M:%S").unwrap();
        let formatted_date = date_time.format("%d %B %Y, %I:%M %p").to_string();

        Reminder {
            id: reminder.id,
            title: reminder.title.clone(),
            description: reminder.description.clone(),
            date: formatted_date,
        }
    })
    .collect();

    let context = json!({
        "pagetitle": "r.me | Reminders",
        "reminders": reminders_data,
    });

    Template::render("reminders", &context)
}

#[get("/reminders/add")]
fn get_reminders() -> Template {
	let context = json!({
		"pagetitle": "r.me | Add Reminder"
	});
	
	Template::render("getreminders", &context)
}

fn main() {
	rocket::ignite()
		.mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
		.mount("/", routes![index, reminders, get_reminders])
		.attach(Template::fairing())
		.launch();
}
