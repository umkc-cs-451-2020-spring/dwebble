#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[get("/")]
fn index() -> String {
    String::from("Hello, World!")
}

fn main() {
    let app_routes = routes![index];

    rocket::ignite().mount("/", app_routes).launch();
}
