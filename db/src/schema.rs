// @generated automatically by Diesel CLI.

diesel::table! {
    author (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    directory (id) {
        id -> Integer,
        name -> Text,
        full_path -> Text,
    }
}

diesel::table! {
    info_file (id) {
        id -> Integer,
        location_id -> Integer,
        directory_id -> Integer,
        object_id -> Text,
        numeric_version -> Integer,
        display_version -> Text,
        earliest_compat_numeric_version -> Integer,
        earliest_compat_display_version -> Text,
        author_id -> Integer,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    location (id) {
        id -> Integer,
        name -> Text,
        directory_id -> Integer,
    }
}

diesel::joinable!(info_file -> author (author_id));
diesel::joinable!(info_file -> directory (directory_id));
diesel::joinable!(info_file -> location (location_id));
diesel::joinable!(location -> directory (directory_id));

diesel::allow_tables_to_appear_in_same_query!(author, directory, info_file, location,);
