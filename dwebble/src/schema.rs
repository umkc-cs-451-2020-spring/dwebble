use crate::models::*;
use crate::models::{InstructorEnum, PermissionEnum, SemesterEnum};
use diesel::prelude::*;

table! {
    use diesel::sql_types::*;
    use super::InstructorEnumMapping;
    instructor (instructor_id) {
        instructor_id -> Int4,
        type_ -> InstructorEnumMapping,
        user_id -> Nullable<Int4>,
        f_name -> Varchar,
        l_name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use super::SemesterEnumMapping;
    schedule (schedule_id) {
        schedule_id -> Int4,
        instructor_id -> Nullable<Int4>,
        schedule_data -> Jsonb,
        semester -> SemesterEnumMapping,
        year -> Nullable<Int2>,
    }
}

table! {
    use diesel::sql_types::*;
    use super::PermissionEnumMapping;
    user_ (id) {
        id -> Int4,
        username -> Varchar,
        f_name -> Varchar,
        l_name -> Varchar,
        email -> Varchar,
        pw_hash -> Text,
        user_auth -> PermissionEnumMapping,
    }
}

joinable!(instructor -> user_ (user_id));
joinable!(schedule -> instructor (instructor_id));

allow_tables_to_appear_in_same_query!(instructor, schedule, user_,);
