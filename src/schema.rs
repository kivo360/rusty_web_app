table! {
    events (id) {
        id -> Int4,
        streamer_name -> Varchar,
        event_type -> Varchar,
        viewer_name -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        api_key -> Text,
        favorite_streamer -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    users,
);
