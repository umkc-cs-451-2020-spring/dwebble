use crate::schema::user_;
use crate::util::{hash_password, verify_password};
use crate::{LoginForm, RegisterForm};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket_contrib::databases::diesel::Queryable;

#[derive(DbEnum, Debug)]
pub enum InstructorEnum {
    Tenured,
    TenuredTrack,
    NonTenured,
    PtAdjunct,
    FtAdjunct,
    GraduateTa,
}

#[derive(DbEnum, Debug)]
pub enum SemesterEnum {
    Summer,
    Fall,
    Winter,
    Sprint,
}

#[derive(DbEnum, Debug)]
pub enum PermissionEnum {
    Admin,
    Standard,
}

#[derive(Queryable)]
// #[table_name = "user_"]
pub struct User_ {
    pub id: i32,
    pub username: String,
    pub f_name: String,
    pub l_name: String,
    pub email: String,
    pw_hash: String,
    user_auth: PermissionEnum,
}

#[derive(Insertable)]
#[table_name = "user_"]
pub struct NewUser<'a> {
    // pub dob: Option<&'a NaiveDate>,
    pub f_name: &'a str,
    pub l_name: Option<&'a str>,
    pub email: &'a str,
    pub username: &'a str,
    pub pw_hash: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct AuthUser {
    pub username: String,
    pub session: String,
}

pub fn register_user(db: &PgConnection, form: &RegisterForm) -> Result<User_, String> {
    let pw_hash = hash_password(&form.password);
    println!("pw_hash: {:?}", pw_hash);
    let username = &form.username;
    let f_name = &form.f_name;
    let l_name = &form.l_name;
    let email = &form.email;
    // let dob = NaiveDate::parse_from_str(&form.DOB, "%Y-%m-%d").expect("Could not parse dob");

    let new_user = &NewUser {
        // dob: Some(&dob),
        pw_hash: &pw_hash, // std::str::from_utf8(&pw_hash).expect("Could not convert hash into utf8 str"),
        f_name: &f_name,
        l_name: Some(&l_name),
        username: &username,
        email: &email,
    };

    diesel::insert_into(user_::table)
        .values(new_user)
        .get_result::<User_>(db)
        .map_err(|e| format!("Error: {:?}\nTry a different username or email.", e))
}

pub fn login_user(db: &PgConnection, form: &LoginForm) -> Result<AuthUser, String> {
    let pw = &form.password;
    let username = &form.username;

    let user = user_::table
        .filter(user_::username.eq(username))
        .get_result::<User_>(db)
        .map_err(|e| format!("Error: {:?}", e))?;

    let check_pw = verify_password(&pw, &user.pw_hash);

    if check_pw {
        Ok(AuthUser {
            username: String::clone(username),
            session: hash_password(&format!("{}{}", user.username, user.id)),
        })
    } else {
        Err(String::from("Invalid username/password"))
    }
}
