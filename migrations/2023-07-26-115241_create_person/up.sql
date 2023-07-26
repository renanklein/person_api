-- Your SQL goes here
CREATE TABLE person (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    age INTEGER NOT NULL,
    address_id INTEGER NOT NULL,
    document_id INTEGER NOT NULL,
    CONSTRAINT fk_address FOREIGN KEY(address_id) REFERENCES address(id),
    CONSTRAINT fk_document FOREIGN KEY(document_id) REFERENCES document(id)
)