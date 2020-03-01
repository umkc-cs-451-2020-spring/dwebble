#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::http::Cookies;
use rocket::http::RawStr;
use rocket::request::FlashMessage;
use rocket::request::Form;
use rocket::request::FromForm;
use rocket::request::LenientForm;
use rocket::response::Flash;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

use std::sync::RwLock;

struct AppConfig {}

type DwebbleConfig = RwLock<AppConfig>;

#[derive(FromForm)]
struct LoginForm {
    input: String,
}

struct LoginContext {}

/// A very basic Template Context
#[derive(Serialize)]
struct HelloContext {
    name: String,
}

/// Route that demonstrates using a basic Tera Template + dynamic segment data.
#[get("/<name>")]
fn index_name(name: &RawStr) -> Template {
    Template::render(
        "index",
        &HelloContext {
            name: String::from(name.as_str()),
        },
    )
}

#[get("/login")]
fn login(
    flash: Option<FlashMessage>,
    mut cookies: Cookies,
    state: State<DwebbleConfig>,
) -> Template {
    unimplemented!()
}

#[post("/login", data = "<login>")]
fn login_submit(
    state: State<DwebbleConfig>,
    login: LenientForm<LoginForm>,
    mut cookies: Cookies,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    unimplemented!()
}

/// POST department schedule data for dwebble to save.
#[post("/submit")]
fn submit() -> Template {
    unimplemented!()
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        &HelloContext {
            name: String::from("Anonymous User"),
        },
    )
}

/// main calls rocket to execute the web application, which will serve every valid route based off
/// the mounting route given to it.
fn main() {
    // routes! is a macro that will collect and return every handle name given to it.
    let app_routes = routes![index, index_name, submit, login, login_submit,];

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", app_routes)
        .launch();
}
