#![allow(clippy::all, missing_docs, unknown_lints)]
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
    }
}

diesel::table! {
    metadata (id) {
        id -> Integer,
        object_id -> Text,
        name -> Text,
        version -> Text,
        raw_identifier -> Text,
        object_type_id -> Integer,
        raw_file_path -> Text,
        module_id -> Integer,
    }
}

diesel::table! {
    module (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        loction_id -> Integer,
        directory_id -> Integer,
        info_file_id -> Nullable<Integer>,
    }
}

diesel::table! {
    object_type (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(info_file -> author (author_id));
diesel::joinable!(metadata -> module (module_id));
diesel::joinable!(metadata -> object_type (object_type_id));
diesel::joinable!(module -> directory (directory_id));
diesel::joinable!(module -> info_file (info_file_id));
diesel::joinable!(module -> location (loction_id));

diesel::allow_tables_to_appear_in_same_query!(
    author,
    directory,
    info_file,
    location,
    metadata,
    module,
    object_type,
);
