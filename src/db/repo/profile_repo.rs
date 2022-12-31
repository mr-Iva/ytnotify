use crate::db::dto::profile::NewUser;
use crate::db::model::profile::{InsertProfile, Profile};
use crate::db::schema::profile;
use anyhow::Result;
use sqlx::PgConnection;

pub struct ProfileRepo {}

impl ProfileRepo {
    fn by_id(&self, id: i32) -> Profile {
        todo!()
    }

    pub fn by_telegram_id(id: String, pg_connection: &mut PgConnection) -> Profile {
        profile::table
            .filter(profile::telegram_id.eq(id))
            .first::<Profile>(pg_connection)
            .unwrap()
    }

    pub fn create(new_user: &NewUser, pg_connection: &mut PgConnection) -> Result<Profile> {
        let result_user = diesel::insert_into(profile::table)
            .values(&insert_user)
            .on_conflict(profile::telegram_id)
            .do_update()
            .set(&insert_user)
            .get_result::<Profile>(pg_connection)
            .unwrap();
        sqlx::query_as!()
        Ok(result_user)
    }
}
