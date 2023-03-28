#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use serde_json::json;

#[get("/")]
fn index() -> Template {
    let context = json!({
        "title": "My Rust Website"
    });
    Template::render("index", &context)
}

fn main() {
	rocket::ignite()
	.mount("/", routes![index])
	.attach(Template::fairing())
	.launch();
}