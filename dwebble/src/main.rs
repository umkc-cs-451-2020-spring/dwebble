#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::http::RawStr;
use rocket_contrib::templates::Template;

/// A very basic Template Context
#[derive(Serialize)]
struct HelloContext {
    name: String,
}

/// Basic `GET` route on `/` that returns the text "Hello, World!".
#[get("/")]
fn index() -> String {
    String::from("Hello, World!")
}

/// Route that contains a dynamic segment, `<name`, which is which is handled by a parameter guard.
/// This allows us to take the value provided in the segment and interact with it as if it were
/// a formal parameter passed to our handle function.
#[get("/<name>")]
fn index_name(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

/// Route that demonstrates using a basic Tera Template + dynamic segment data.
#[get("/pretty/<name>")]
fn pretty_hello_name(name: &RawStr) -> Template {
    Template::render(
        "index",
        &HelloContext {
            name: String::from(name.as_str()),
        },
    )
}

/// Route that demonstrates using a basic Tera template via the rocket_contrib crate.
/// To render a template, we have to provide it a context, which we defined above, which will
/// carry important data like possible variables expected by the template and so forth.
#[get("/pretty")]
fn pretty_hello() -> Template {
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
    let app_routes = routes![index, index_name, pretty_hello, pretty_hello_name];

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", app_routes)
        .launch();
}
