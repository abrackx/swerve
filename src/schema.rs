table! {
    file (id) {
        id -> Int4,
        name -> Text,
        sort -> Int4,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

table! {
    image (id) {
        id -> Int4,
        name -> Text,
        sort -> Int4,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

table! {
    project (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

table! {
    project_file (id) {
        id -> Int4,
        project_id -> Int4,
        file_id -> Int4,
    }
}

table! {
    project_image (id) {
        id -> Int4,
        project_id -> Int4,
        image_id -> Int4,
    }
}

table! {
    project_tag (id) {
        id -> Int4,
        project_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    tag (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
    }
}

table! {
    user_project (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

joinable!(project_file -> file (file_id));
joinable!(project_file -> project (project_id));
joinable!(project_image -> image (image_id));
joinable!(project_image -> project (project_id));
joinable!(project_tag -> project (project_id));
joinable!(project_tag -> tag (tag_id));
joinable!(user_project -> project (project_id));
joinable!(user_project -> users (user_id));

allow_tables_to_appear_in_same_query!(
    file,
    image,
    project,
    project_file,
    project_image,
    project_tag,
    tag,
    user_project,
    users,
);
