-- Your SQL goes here
CREATE TABLE document(
    id SERIAL PRIMARY KEY NOT NULL,
    doc_type VARCHAR NOT NULL,
    doc_number VARCHAR NOT NULL,
    person_id INTEGER NOT NULL,
    CONSTRAINT fk_document_person FOREIGN KEY(person_id) REFERENCES person(id) ON DELETE CASCADE
)