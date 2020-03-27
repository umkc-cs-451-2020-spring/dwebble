#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket::http::Cookies;
use rocket::http::RawStr;
use rocket::request::FlashMessage;
use rocket::request::FromForm;
use rocket::request::LenientForm;
use rocket::response::Flash;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

use std::sync::RwLock;

pub mod models;
pub mod scheduler;
pub mod schema;

#[database("dwebble_dev")]
struct DevDbConn(diesel::PgConnection);

struct AppConfig {}

type DwebbleConfig = RwLock<AppConfig>;

#[derive(FromForm)]
struct LoginForm {
    input: String,
}

#[derive(Serialize)]
struct LoginContext {
    flash: String,
    error: bool,
}

/// A very basic Template Context
#[derive(Serialize)]
struct HelloContext {
    name: String,
}

#[derive(Serialize)]
struct IndexContext {
    name: String,
    staff_schedules: Vec<String>,
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

/// GET the login page, with login form to fill out and submit
#[get("/login")]
fn login(
    flash: Option<FlashMessage>,
    mut cookies: Cookies,
    state: State<DwebbleConfig>,
) -> Template {
    Template::render(
        "login",
        &LoginContext {
            flash: "".to_string(),
            error: false,
        },
    )
}

/// POST login form data, attempt at login attempt.
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

/// GET the current list of faculty for which there is data entered with respect to schedules
/// Return type is completely undetermined at this point, setting to String by default.
#[get("/schedule_data")]
fn get_schedules() -> String {
    unimplemented!()
}

#[get("/")]
fn index() -> Template {
    let staff = vec![
        "hare".to_string(),
        "bingham".to_string(),
        "mitchell".to_string(),
        "xu".to_string(),
    ];

    Template::render(
        "index",
        &IndexContext {
            name: String::from("Anonymous User"),
            staff_schedules: staff,
        },
    )
}

/// main calls rocket to execute the web application, which will serve every valid route based off
/// the mounting route given to it.
fn main() {
    // routes! is a macro that will collect and return every handle name given to it.
    let app_routes = routes![
        index,
        index_name,
        submit,
        login,
        login_submit,
        get_schedules
    ];

    let cfg = RwLock::new(AppConfig {});

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", app_routes)
        .manage(cfg)
        .launch();
}
