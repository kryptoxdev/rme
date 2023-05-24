#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*;
use rocket::request::Form;
use rocket::response::Redirect;
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

#[derive(FromForm)]
struct ReminderForm {
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

    conn.query_drop("DELETE FROM REMINDERS WHERE date < NOW();")
        .unwrap();

    let reminders_data: Vec<Reminder> = conn
        .query_map(
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
            let date_time =
                NaiveDateTime::parse_from_str(&reminder.date, "%Y-%m-%d %H:%M:%S").unwrap();
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
fn render_add() -> Template {
    let context = json!({
        "pagetitle": "r.me | Add Reminder"
    });

    Template::render("addreminder", &context)
}

#[post("/reminders/add", data = "<form>")]
fn add_reminder(form: Form<ReminderForm>) -> Redirect {
    let url = "mysql://root:password@localhost:3306/jackreminders";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let title = &form.title;
    let description = &form.description;
    let date = &form.date;

    let query = "INSERT INTO reminders (title, description, date) VALUES (?, ?, ?)";

    conn.exec_drop(query, (title, description, date)).unwrap();

    Redirect::to("/reminders")
}

#[get("/reminders/delete/<id>")]
fn render_delete(id: u32) -> Template {
    let url = "mysql://root:password@localhost:3306/jackreminders";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let reminder_data: Vec<Reminder> = conn
        .query_map(
            format!("SELECT * FROM reminders WHERE id = {}", id),
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
            let date_time =
                NaiveDateTime::parse_from_str(&reminder.date, "%Y-%m-%d %H:%M:%S").unwrap();
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
        "pagetitle": "r.me | Delete Reminder",
        "reminders": reminder_data,
    });

    Template::render("deletereminder", &context)
}

#[post("/reminders/delete/<id>")]
fn delete_reminder(id: u32) -> Redirect {
    let url = "mysql://root:password@localhost:3306/jackreminders";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let query = format!("DELETE FROM reminders WHERE id = {}", id);

    conn.query_drop(query).unwrap();

    Redirect::to("/reminders")
}

#[get("/reminders/edit/<id>")]
fn render_edit(id: u32) -> Template {
    let url = "mysql://root:password@localhost:3306/jackreminders";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let reminder_data: Vec<Reminder> = conn
        .query_map(
            format!("SELECT * FROM reminders WHERE id = {}", id),
            |(id, title, description, date)| Reminder {
                id,
                title,
                description,
                date,
            },
        )
        .unwrap()
        .iter()
        .map(|reminder| Reminder {
            id: reminder.id,
            title: reminder.title.clone(),
            description: reminder.description.clone(),
            date: reminder.date.clone(),
        })
        .collect();

    let context = json!({
        "pagetitle": "r.me | Edit Reminder",
        "reminders": reminder_data
    });

    Template::render("editreminder", &context)
}

#[post("/reminders/edit/<id>", data = "<form>")]
fn edit_reminder(id: u32, form: Form<ReminderForm>) -> Redirect {
    let url = "mysql://root:password@localhost:3306/jackreminders";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let title = &form.title;
    let description = &form.description;
    let date = &form.date;

    let query = format!(
        "UPDATE reminders SET title = ?, description = ?, date = ? WHERE id = ?"
    );

    conn.exec_drop(query, (title, description, date, id)).unwrap();

    Redirect::to("/reminders")
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("404", ())
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount(
            "/",
            routes![
                index,
                reminders,
                render_add,
                add_reminder,
                render_delete,
                delete_reminder,
                render_edit,
                edit_reminder
            ],
        )
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}
