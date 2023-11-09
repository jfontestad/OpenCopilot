// @generated automatically by Diesel CLI.

diesel::table! {
    chatbot_settings (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 36]
        chatbot_id -> Nullable<Char>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        value -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    chatbots (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        token -> Varchar,
        #[max_length = 255]
        website -> Nullable<Varchar>,
        #[max_length = 255]
        status -> Nullable<Varchar>,
        #[max_length = 255]
        prompt_message -> Nullable<Varchar>,
        enhanced_privacy -> Nullable<Bool>,
        smart_sync -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
        #[max_length = 255]
        swagger_url -> Nullable<Varchar>,
        is_premade_demo_template -> Nullable<Bool>,
    }
}

diesel::table! {
    failed_jobs (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        uuid -> Varchar,
        #[max_length = 255]
        connection -> Varchar,
        #[max_length = 255]
        queue -> Varchar,
        payload -> Text,
        exception -> Text,
        failed_at -> Timestamp,
    }
}

diesel::table! {
    jobs (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        queue -> Nullable<Varchar>,
        #[max_length = 255]
        payload -> Nullable<Varchar>,
        attempts -> Nullable<Tinyint>,
        reserved_at -> Nullable<Integer>,
        available_at -> Nullable<Integer>,
        created_at -> Nullable<Integer>,
    }
}

diesel::table! {
    pdf_data_sources (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        chatbot_id -> Nullable<Varchar>,
        files -> Nullable<Json>,
        files_info -> Nullable<Json>,
        #[max_length = 255]
        folder_name -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        #[max_length = 255]
        ingest_status -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chatbot_settings,
    chatbots,
    failed_jobs,
    jobs,
    pdf_data_sources,
);
