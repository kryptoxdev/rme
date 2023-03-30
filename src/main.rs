#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::database;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_contrib::databases::mysql;
use serde_json::json;
use std::env;

#[database("jackreminders")]
struct DatabasePool(mysql::Connection);

#[get("/")]
fn index() -> Template {
    let context = json!({
        "title": "r.me | Home"
    });
    Template::render("index", &context)
}

#[get("/reminders")]
fn reminders() -> Template {
	let context = json!({
		"title": "r.me | Reminders"
	});
	Template::render("reminders", &context)
}

fn main() {	
	rocket::ignite()
	.mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
	.mount("/", routes![index, reminders])
	.attach(Template::fairing())
	.attach(dbConnection::fairing())
	.manage(mysql::Client::open("mysql://root:password@localhost/jackreminders").unwrap())
	.launch();
}