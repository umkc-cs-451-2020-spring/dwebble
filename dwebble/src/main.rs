#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate validator_derive;
#[macro_use] extern crate lazy_static;

extern crate validator;

use csrf::{AesGcmCsrfProtection, CsrfCookie, CsrfProtection, CsrfToken};
use data_encoding::BASE64;
use regex::Regex;
use rocket::fairing::AdHoc;
use rocket::http::RawStr;
use rocket::http::{Cookie, Cookies};
use rocket::request::FlashMessage;
use rocket::request::FromForm;
use rocket::request::LenientForm;
use rocket::response::Flash;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::{GlobalFn, Result as TeraResult};
use rocket_contrib::templates::Template;
use serde_json::from_value;
use serde_json::to_value;
use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;

use validator::{Validate, ValidationError};

use std::sync::RwLock;

pub mod models;
pub mod scheduler;
pub mod schema;

#[database("dwebble_dev")]
struct DevDbConn(diesel::PgConnection);

const CSRF_COOKIE_ID: &str = "dwebble-cookie";
const SESSION_COOKIE_ID: &str = "dwebble-session";

struct CsrfSecret(String);

lazy_static! {
    static ref VALID_USERNAME_REGEX: Regex =
        Regex::new(r"^[[:word:]-]{3,10}").expect("Failed to build user name regex! Wth!?");
}

struct AppConfig {
    aes_generator: AesGcmCsrfProtection,
    csrf_tokens: HashMap<CsrfCookie, CsrfToken>,
}

type DwebbleConfig = RwLock<AppConfig>;

#[derive(FromForm)]
struct LoginForm {
    input: String,
}

#[derive(Serialize)]
struct LoginContext {
    csrf_token: String,
    flash: String,
    error: bool,
}

#[derive(Serialize)]
struct RegisterContext {
    csrf_token: String,
    flash: String,
    error: bool,
}

#[derive(Validate, FromForm)]
pub struct RegisterForm {
    #[validate(length(min = 2))]
    f_name: String,
    #[validate(length(min = 2))]
    l_name: String,
    #[validate(regex(
        path = "VALID_USERNAME_REGEX",
        message = "Invalid username. A-Za-z0-9, '-', and '_' characters and of 3 to 10 characters long."
    ))]
    username: String,
    // DOB: String,
    #[validate(email)]
    email: String,
    #[validate(length(
        min = 12,
        max = 64,
        message = "Invalid password. Minimum length of 12, maximum of 64."
    ))]
    password: String,
    //    #[validate(must_match = "password")]
    confirm_password: String,
    csrf_token: String,
}

/// A very basic Template Context
#[derive(Serialize)]
struct HelloContext {
    name: String,
}

#[derive(Serialize)]
struct IndexContext {
    name: String,
    auth: bool,
    staff_schedules: Vec<String>,
}

fn generate_csrf_pair(cfg: &mut AppConfig) -> (String, String) {
    let (token, cookie) = cfg
        .aes_generator
        .generate_token_pair(None, 300)
        .expect("Could not generate csrf token-cookie pair.");

    cfg.csrf_tokens.insert(cookie.clone(), token.clone());
    (token.b64_string(), cookie.b64_string())
}

fn inject_csrf(mut cookies: Cookies, cfg: &mut AppConfig) -> (String) {
    let (token, cookie) = cfg
        .aes_generator
        .generate_token_pair(None, 300)
        .expect("Could not generate csrf token-cookie pair.");

    cfg.csrf_tokens.insert(cookie.clone(), token.clone());
    let (token_str, cookie_str) = (token.b64_string(), cookie.b64_string());

    cookies.add_private(Cookie::new(CSRF_COOKIE_ID, cookie_str));
    drop(cookies);

    token_str
}

/// GET the login page, with login form to fill out and submit
#[get("/login")]
fn login(
    flash: Option<FlashMessage>,
    mut cookies: Cookies,
    state: State<DwebbleConfig>,
) -> Template {
    let mut cfg = state
        .write()
        .expect("Cannot write, config locked by Readers");

    let token_str = inject_csrf(cookies, &mut cfg);

    let mut s = String::new();
    let mut err = false;
    // If we were redirected via a Flash Redirect, handle that here.
    if let Some(ref msg) = flash {
        s = String::from(msg.msg());
        if msg.name() == "error" {
            err = true;
        }
    }

    Template::render(
        "login",
        &LoginContext {
            csrf_token: token_str,
            flash: s,
            error: err,
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

#[get("/register")]
fn register(
    flash: Option<FlashMessage>,
    mut cookies: Cookies,
    state: State<DwebbleConfig>,
) -> Template {
    let mut cfg = state
        .write()
        .expect("Cannot write, config locked by Readers");

    let token_str = inject_csrf(cookies, &mut cfg);

    let mut s = String::new();
    let mut err = false;
    // If we were redirected via a Flash Redirect, handle that here.
    if let Some(ref msg) = flash {
        s = String::from(msg.msg());
        if msg.name() == "error" {
            err = true;
        }
    }

    Template::render(
        "register",
        &RegisterContext {
            csrf_token: token_str,
            flash: s,
            error: err,
        },
    )
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
            auth: false,
            staff_schedules: staff,
        },
    )
}

fn make_url_for(urls: BTreeMap<String, String>) -> GlobalFn {
    Box::new(move |args| -> TeraResult<serde_json::Value> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => Ok(to_value(urls.get(&v).unwrap()).unwrap()),
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}

// #[get("/static/<path..>")]
// fn static_resource() ->

/// main calls rocket to execute the web application, which will serve every valid route based off
/// the mounting route given to it.
fn main() {
    // routes! is a macro that will collect and return every handle name given to it.
    let app_routes = routes![index, submit, login, login_submit, register, get_schedules];

    rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("CSRF Secret Key", |rocket| {
            let csrf_secret = rocket
                .config()
                .get_str("csrf_secret_key")
                .unwrap_or("You-dont-have-a-csrf-secret-configured!")
                .to_string();
            Ok(rocket.manage(CsrfSecret(csrf_secret)))
        }))
        .attach(AdHoc::on_attach("AppConfig", |rocket| {
            let csrf_secret = rocket.state::<CsrfSecret>();

            let mut arr_secret: [u8; 32] = Default::default();
            match csrf_secret {
                Some(secret) => {
                    arr_secret.copy_from_slice(&secret.0.as_bytes()[0..32]);
                    Ok(rocket.manage(RwLock::new(AppConfig {
                        aes_generator: AesGcmCsrfProtection::from_key(arr_secret),
                        csrf_tokens: HashMap::new(),
                    })))
                }
                None => panic!("No CsrfSecret, unable to generate AppConfig struct"),
            }
        }))
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/", app_routes)
        .launch();
}
