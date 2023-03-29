#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde_json::json;
use std::env;

#[get("/")]
fn index() -> Template {
    let context = json!({
        "title": "My Rust Website"
    });
    Template::render("index", &context)
}

fn main() {
	let template_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/templates");
    env::set_current_dir(template_dir).unwrap();
		
	rocket::ignite()
	.mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
	.mount("/", routes![index])
	.attach(Template::fairing())
	.launch();
}