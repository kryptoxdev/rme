#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

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

	let reminders_data = conn.query_map("SELECT * FROM reminders",|(id, title, description, date)| 
	Reminder { id, title, description, date }).unwrap();

	let context = json!({
		"pagetitle": "r.me | Reminders",
		"reminders": reminders_data,
	});

	Template::render("reminders", &context)
}

fn main() {
	rocket::ignite()
		.mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
		.mount("/", routes![index, reminders])
		.attach(Template::fairing())
		.launch();
}
