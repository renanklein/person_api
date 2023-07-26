-- Your SQL goes here
CREATE TABLE document(
    id SERIAL PRIMARY KEY NOT NULL,
    doc_type VARCHAR NOT NULL,
    doc_number VARCHAR NOT NULL
);