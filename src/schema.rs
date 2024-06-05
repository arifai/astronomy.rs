// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (token) {
        astronaut_id -> Uuid,
        token -> Text,
        valid_thru -> Timestamp,
        created_time -> Timestamp,
    }
}

diesel::table! {
    astronauts (id) {
        id -> Uuid,
        full_name -> Varchar,
        avatar -> Nullable<Text>,
        email -> Varchar,
        active -> Bool,
        registered_time -> Timestamp,
    }
}

diesel::table! {
    astronauts_pass_hashed (astronaut_id) {
        astronaut_id -> Uuid,
        pass_hashed -> Nullable<Text>,
    }
}

diesel::table! {
    constellations (id) {
        id -> Uuid,
        name -> Varchar,
        created_time -> Timestamp,
    }
}

diesel::table! {
    galaxies (id) {
        id -> Uuid,
        constellation_id -> Uuid,
        name -> Varchar,
        image -> Text,
        note -> Nullable<Text>,
        created_time -> Timestamp,
    }
}

diesel::table! {
    planets (id) {
        id -> Uuid,
        galaxy_id -> Uuid,
        name -> Varchar,
        image -> Text,
        short_description -> Nullable<Varchar>,
        details -> Nullable<Text>,
        created_time -> Timestamp,
    }
}

diesel::joinable!(access_tokens -> astronauts (astronaut_id));
diesel::joinable!(astronauts_pass_hashed -> astronauts (astronaut_id));
diesel::joinable!(galaxies -> constellations (constellation_id));
diesel::joinable!(planets -> galaxies (galaxy_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_tokens,
    astronauts,
    astronauts_pass_hashed,
    constellations,
    galaxies,
    planets,
);
