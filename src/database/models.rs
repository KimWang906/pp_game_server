use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};

use super::schema::users::{self};

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DatabaseUser {
    pub id: i32,
    pub player_id: i32,
    pub student_id: String,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewDatabaseUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub student_id: &'a str,
}

impl DatabaseUser {
    pub fn create(
        conn: &mut PgConnection,
        new_user: NewDatabaseUser,
    ) -> Result<DatabaseUser, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(new_user)
            .returning(DatabaseUser::as_returning())
            .get_result(conn)
    }

    pub fn find_by_username(conn: &mut PgConnection, username: &str) -> Option<DatabaseUser> {
        users::dsl::users
            .select(DatabaseUser::as_select())
            .filter(users::dsl::username.eq(username))
            .first(conn)
            .ok()
    }

    pub fn find_by_student_id(conn: &mut PgConnection, student_id: &str) -> Option<DatabaseUser> {
        users::dsl::users
            .select(DatabaseUser::as_select())
            .filter(users::dsl::student_id.eq(student_id))
            .first(conn)
            .ok()
    }

    pub fn student_id_exists(conn: &mut PgConnection, student_id: &str) -> bool {
        use diesel::dsl::exists;

        diesel::select(exists(
            users::dsl::users.filter(users::dsl::student_id.eq(student_id)),
        ))
        .get_result(conn)
        .unwrap()
    }
}
