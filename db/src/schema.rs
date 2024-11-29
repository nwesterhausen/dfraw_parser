#![allow(clippy::all, missing_docs, unknown_lints)]
// @generated automatically by Diesel CLI.

diesel::table! {
    author (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    color (id) {
        id -> Integer,
        foreground -> Integer,
        background -> Integer,
        brightness -> Integer,
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
        steam_data_id -> Nullable<Integer>,
    }
}

diesel::table! {
    info_file_conflicts_ids (info_file_id, conflicts_info_file_id) {
        info_file_id -> Integer,
        conflicts_info_file_id -> Integer,
    }
}

diesel::table! {
    info_file_requires_after (info_file_id, requires_info_file_id) {
        info_file_id -> Integer,
        requires_info_file_id -> Integer,
    }
}

diesel::table! {
    info_file_requires_before (info_file_id, requires_info_file_id) {
        info_file_id -> Integer,
        requires_info_file_id -> Integer,
    }
}

diesel::table! {
    info_file_requires_ids (info_file_id, requires_info_file_id) {
        info_file_id -> Integer,
        requires_info_file_id -> Integer,
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
        location_id -> Integer,
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

diesel::table! {
    steam_data (id) {
        id -> Integer,
        file_id -> Text,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        changelog -> Nullable<Text>,
    }
}

diesel::table! {
    steam_data_key_value_tag (id) {
        id -> Integer,
        steam_data_id -> Integer,
        steam_tag_id -> Integer,
        value -> Text,
    }
}

diesel::table! {
    steam_data_metadata (id) {
        id -> Integer,
        steam_data_id -> Integer,
        metadata -> Text,
    }
}

diesel::table! {
    steam_data_tag (steam_data_id, steam_tag_id) {
        steam_data_id -> Integer,
        steam_tag_id -> Integer,
    }
}

diesel::table! {
    steam_tag (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(info_file -> author (author_id));
diesel::joinable!(info_file -> steam_data (steam_data_id));
diesel::joinable!(metadata -> module (module_id));
diesel::joinable!(metadata -> object_type (object_type_id));
diesel::joinable!(module -> directory (directory_id));
diesel::joinable!(module -> info_file (info_file_id));
diesel::joinable!(module -> location (location_id));
diesel::joinable!(steam_data_key_value_tag -> steam_data (steam_data_id));
diesel::joinable!(steam_data_key_value_tag -> steam_tag (steam_tag_id));
diesel::joinable!(steam_data_metadata -> steam_data (steam_data_id));
diesel::joinable!(steam_data_tag -> steam_data (steam_data_id));
diesel::joinable!(steam_data_tag -> steam_tag (steam_tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    author,
    color,
    directory,
    info_file,
    info_file_conflicts_ids,
    info_file_requires_after,
    info_file_requires_before,
    info_file_requires_ids,
    location,
    metadata,
    module,
    object_type,
    steam_data,
    steam_data_key_value_tag,
    steam_data_metadata,
    steam_data_tag,
    steam_tag,
);
