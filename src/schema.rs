// @generated automatically by Diesel CLI.

diesel::table! {
    groups (rowid) {
        id -> Nullable<Uuid>,
        group_name -> Varchar,
        user_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        last_modified -> Timestamp,
        rowid -> Int8,
    }
}

diesel::table! {
    history (rowid) {
        id -> Nullable<Uuid>,
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
    logins (rowid) {
        id -> Nullable<Uuid>,
        group_id -> Uuid,
        cname -> Varchar,
        url -> Nullable<Text>,
        login -> Text,
        password -> Text,
        email -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        last_modified -> Timestamp,
        rowid -> Int8,
    }
}

diesel::table! {
    users (rowid) {
        id -> Nullable<Uuid>,
        username -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Nullable<Timestamp>,
        last_modified -> Timestamp,
        rowid -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    history,
    logins,
    users,
);
