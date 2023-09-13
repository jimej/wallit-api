// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Uuid,
        group_name -> Varchar,
        user_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        last_modified -> Timestamp,
        // rowid -> Int8,
        team -> Nullable<Bool>,
    }
}

diesel::table! {
    history (rowid) {
        id -> Uuid,
        group_id -> Uuid,
        cname -> Varchar,
        url -> Nullable<Text>,
        login -> Text,
        password -> Text,
        email -> Varchar,
        description -> Nullable<Text>,
        mode -> Varchar,
        created_at -> Nullable<Timestamp>,
        rowid -> Int8,
    }
}

diesel::table! {
    logins (id) {
        id -> Uuid,
        group_id -> Uuid,
        cname -> Varchar,
        url -> Nullable<Text>,
        login -> Text,
        password -> Text,
        email -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        last_modified -> Timestamp,
        // rowid -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Nullable<Timestamp>,
        last_modified -> Timestamp,
        // rowid -> Int8,
    }
}

diesel::table! {
    users_teams (user_id, team_id) {
        id -> Uuid,
        user_id -> Uuid,
        team_id -> Uuid,
        permission -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(groups, history, logins, users, users_teams,);
