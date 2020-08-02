use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::schema::*;
use crate::{DbConnection, Pool};
use diesel::prelude::*;
use diesel::sql_query;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::sql_types::{BigInt, Text};
use diesel::PgConnection;
use diesel::{Insertable, Queryable};
use fake::faker::company::raw::*;
use fake::faker::internet::raw::*;
// use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::option::Option;

/// ----------------------------------------------------------------------------------------------- ///
/// -------------------------------------- Request and Responses ---------------------------------- ///
/// ----------------------------------------------------------------------------------------------- ///
#[derive(Serialize, Deserialize)]
pub struct Status {
    pub status: String,
}

/// Register User, request/response
#[derive(Serialize, Deserialize)]
pub struct RegisterUserReqInput {
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUserResOutput {
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserReqInput {
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserResOutput {
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralUpdateReqInput {
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralUpdateResOutput {
    pub status: String,
}

/// ----------------------------------------------------------------------------------------------- ///
/// --------------------------------------- Database Models --------------------------------------- ///
/// ----------------------------------------------------------------------------------------------- ///
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[table_name = "events"]
pub struct Event {
    pub id: i32,
    pub event_type: String,
    pub streamer_name: String,
    pub viewer_name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "events"]
pub struct EventNoId {
    pub event_type: String,
    pub streamer_name: String,
    pub viewer_name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub api_key: String,
    pub favorite_streamer: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct UserNoId {
    pub api_key: String,
    pub favorite_streamer: String,
}

#[derive(QueryableByName, Debug, Clone)]
pub struct StreamerEventCount {
    #[sql_type = "Text"]
    pub streamer_name: String,
    #[sql_type = "BigInt"]
    pub total: i64,
}

impl UserNoId {
    pub fn new(api_key: &str, favorite_streamer: &str) -> UserNoId {
        UserNoId {
            api_key: String::from(api_key),
            favorite_streamer: String::from(favorite_streamer),
        }
    }
}

/// ----------------------------------------------------------------------------------------------- ///
/// ------------------------------------- App's Database Commands --------------------------------- ///
/// ----------------------------------------------------------------------------------------------- ///

// Will Perhaps move this code and tests to another file
pub struct DBCommands {
    pub conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl DBCommands {
    /// Determines if the user exist inside of the database. Use to filter through information
    /// Returns false if the user doesn't exist, and true if the user does.
    pub fn is_user_exist(&self, _api_key: &str) -> Result<bool, diesel::result::Error> {
        use super::schema::users::dsl::*;
        let tess_id: Result<Option<i32>, diesel::result::Error> = users
            .filter(api_key.eq(_api_key))
            .select(id)
            .first(&self.conn)
            .optional();
        let _unwrapped = tess_id.unwrap().unwrap_or(-1);
        if _unwrapped == -1 {
            return Ok(false);
        }
        Ok(true)
    }
    pub fn create_user(&self, user: UserNoId) -> Result<User, diesel::result::Error> {
        use super::schema::users::dsl::*;
        let savable_user = UserNoId::from(user);
        let inserted_user: User = diesel::insert_into(users)
            .values(savable_user)
            .get_result(&self.conn)
            .expect("User was not inserted");
        Ok(inserted_user)
    }

    pub fn create_event(&self, event: EventNoId) -> Result<Event, diesel::result::Error> {
        // Create Event inside of the database
        use super::schema::events::dsl::*;
        let savable_user = EventNoId::from(event);
        let inserted_user: Event = diesel::insert_into(events)
            .values(savable_user)
            .get_result(&self.conn)
            .expect("User was not inserted");
        Ok(inserted_user)
    }

    pub fn get_events_by_user(&self, user_id: i32) -> Result<Vec<Event>, diesel::result::Error> {
        // Finds the events the user is subscribed to.
        // Determined by the favorite_streamer attached to the user
        use super::schema::events::dsl::*;
        use super::schema::users::dsl::*;
        let user: User = users
            .find(&user_id)
            .get_result(&self.conn)
            .expect("User Not Found");

        // We find events here.
        let items = events
            .filter(streamer_name.eq(&user.favorite_streamer))
            .load::<Event>(&self.conn)
            .expect("Events not found");
        Ok(items)
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Result<User, diesel::result::Error> {
        // Finds the events the user is subscribed to.
        // Determined by the favorite_streamer attached to the user
        use super::schema::users::dsl::*;
        let user: User = users
            .find(&user_id)
            .get_result(&self.conn)
            .expect("User Not Found");

        Ok(user)
    }

    pub fn update_user_favorite(&self, user_id: i32, _streamer_name: String) {
        use super::schema::users::dsl::*;
        diesel::update(users.filter(id.eq(user_id)))
            .set(favorite_streamer.eq(_streamer_name))
            .get_result::<User>(&self.conn)
            .expect("User doesn't exist.");
    }

    pub fn get_event_by_id(&self, event_id: i32) -> Result<Event, diesel::result::Error> {
        // Finds the events the user is subscribed to.
        // Determined by the favorite_streamer attached to the user
        use super::schema::events::dsl::*;
        let event: Event = events
            .find(&event_id)
            .get_result(&self.conn)
            .expect("User Not Found");

        Ok(event)
    }

    pub fn get_event_type_count_by_streamer(
        &self,
        _streamer_name: &String,
    ) -> Result<i64, diesel::result::Error> {
        // Finds the events the user is subscribed to.
        // Determined by the favorite_streamer attached to the user
        use super::schema::events::dsl::*;
        let distinct_type_count = events
            .filter(streamer_name.eq(_streamer_name))
            .count()
            .get_result(&self.conn)
            .expect("Something aint right!!!");

        Ok(distinct_type_count)
    }
    pub fn get_event_total(&self) -> Result<Vec<StreamerEventCount>, diesel::result::Error> {
        // Finds the events the user is subscribed to.
        // Determined by the favorite_streamer attached to the user
        let distinct_type_count =
            sql_query("SELECT streamer_name, COUNT(*) as total FROM events GROUP BY streamer_name")
                .load::<StreamerEventCount>(&self.conn)
                .expect("Unable to get the total number of events for user.");
        Ok(distinct_type_count)
    }

    pub fn latest_events_each_user(
        &self,
        _streamer_name: &String,
        each_count: i32,
    ) -> Result<Vec<Event>, diesel::result::Error> {
        let latest_events_query = sql_query(include_str!("./queries/latest_events_by_user.sql"))
            .bind::<diesel::sql_types::Text, _>(_streamer_name)
            .bind::<diesel::sql_types::Integer, _>(each_count)
            .load::<Event>(&self.conn)
            .expect("Unable to get the total number of events for user.");

        Ok(latest_events_query)
    }
}

lazy_static! {
    static ref POOL: Pool = {
        let db_url = std::env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() -> Result<DbConnection, std::io::Error> {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    Ok(conn)
}

pub fn connection() -> Result<DbConnection, std::io::Error> {
    POOL.get()
        .map_err(|_e| Error::new(ErrorKind::Other, "Oh no! The connection failed."))
}

/// ----------------------------------------------------------------------------------------------- ///
/// ------------------------------------- Database Commands Tests --------------------------------- ///
/// ----------------------------------------------------------------------------------------------- ///

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user_single_user() {
        // Creates a single user
        // Look for that exact user.
        let conn = init().unwrap();
        let commands = DBCommands {
            // user_id: 0,
            conn: conn,
        };

        let ultimate_user: UserNoId = UserNoId {
            api_key: String::from("sample_key"),
            favorite_streamer: String::from("my_free_pay_site"),
        };

        let new_user = commands.create_user(ultimate_user).unwrap();
        let current_user = commands.get_user_by_id(new_user.id).unwrap();
        assert_eq!(new_user.id, current_user.id);
    }

    #[test]
    fn create_single_event() {
        // Creates a single user
        // Look for that exact user.
        let conn = init().unwrap();
        let commands = DBCommands {
            // user_id: 0,
            conn: conn,
        };

        let ultimate_event: EventNoId = EventNoId {
            event_type: String::from("sample_key"),
            streamer_name: String::from("sample_key"),
            viewer_name: String::from("sample_key"),
            created_at: None,
        };

        let new_event = commands.create_event(ultimate_event).unwrap();
        let selected_event = commands.get_event_by_id(new_event.id).unwrap();
        assert_eq!(new_event.id, selected_event.id);
        assert_eq!(new_event.event_type, selected_event.event_type);
    }

    #[test]
    fn count_event_streamer() {
        // Count the number of events of each type.
        // Create three fake streamer names (using faker)
        // Total number of events received by each streamer
        let conn = init().unwrap();
        let commands = DBCommands { conn: conn };
        // vs.choose(&mut rand::thread_rng())
        let mut _streamer_names = Vec::new();
        let mut _viewer_names = Vec::new();
        let mut _event_types = Vec::new();

        for _ in 0..3 {
            let _streamer_name: String = Username(EN).fake();
            let _viewer_name: String = Username(EN).fake();
            let _event_type: String = Buzzword(EN).fake();
            _streamer_names.push(_streamer_name);
            _viewer_names.push(_viewer_name);
            _event_types.push(_event_type);
        }
        let _viewer_name: String = Username(EN).fake();

        for _ in 0..75 {
            let streamer_sample = _streamer_names
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone();

            let event_type_sample = _viewer_names
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone();
            let current_event: EventNoId = EventNoId {
                event_type: event_type_sample,
                streamer_name: streamer_sample,
                viewer_name: _viewer_name.clone(),
                created_at: None,
            };
            let _new_event = commands.create_event(current_event).unwrap();
        }

        // let new_event = commands.create_event(ultimate_event).unwrap();
        // let selected_event = commands.get_event_by_id(new_event.id).unwrap();
        // Number of viewers should be 3
        let streamer_sample = _streamer_names
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        let etype_count = commands
            .get_event_type_count_by_streamer(&streamer_sample)
            .unwrap();

        assert!(
            18 < etype_count && etype_count < 35,
            "The number of events for the {} wasn't between the bounds of 20 and 35",
            &streamer_sample
        );
    }

    #[test]
    fn event_count_each_streamer() {
        // Creates a single user
        // Look for that exact user.
        let selected_user = String::from("cornelius_eum");
        let conn = init().unwrap();
        let commands = DBCommands { conn: conn };

        let totals = commands.get_event_total();
        let unwrapped_totals = totals.unwrap();
        let latest_events = commands.latest_events_each_user(&selected_user, 2);
        let unwrapped_events = latest_events.unwrap();

        assert!(
            unwrapped_events.len() > 0,
            "The number of events for the {} wasn't greater than 0.",
            "cornelius_eum"
        );
        assert!(
            unwrapped_totals.len() > 0,
            "The number of events for the {} wasn't greater than 0.",
            "cornelius_eum"
        );
    }

    #[test]
    fn change_favorite_streamer() {
        // Creates a single user
        // Look for that exact user.
        let conn = init().unwrap();
        let commands = DBCommands { conn: conn };

        let ultimate_user: UserNoId = UserNoId {
            api_key: String::from("sample_key"),
            favorite_streamer: String::from("sample_streamer"),
        };

        let new_user = commands.create_user(ultimate_user).unwrap();
        let current_user = commands.get_user_by_id(new_user.id).unwrap();
        assert_eq!(new_user.id, current_user.id);

        commands.update_user_favorite(current_user.id, String::from("sample_streamerr"));
        let updated_user = commands.get_user_by_id(new_user.id).unwrap();
        assert_eq!(
            updated_user.favorite_streamer,
            String::from("sample_streamerr")
        );
    }
}
