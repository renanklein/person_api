// @generated automatically by Diesel CLI.

diesel::table! {
    address (id) {
        id -> Int4,
        state -> Varchar,
        city -> Varchar,
        country -> Varchar,
        zip_code -> Varchar,
        neighborhood -> Nullable<Varchar>,
        complement -> Nullable<Varchar>,
        number -> Varchar,
    }
}

diesel::table! {
    document (id) {
        id -> Int4,
        doc_type -> Varchar,
        doc_number -> Varchar,
    }
}

diesel::table! {
    person (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
        address_id -> Int4,
        document_id -> Int4,
    }
}

diesel::joinable!(person -> address (address_id));
diesel::joinable!(person -> document (document_id));

diesel::allow_tables_to_appear_in_same_query!(
    address,
    document,
    person,
);
