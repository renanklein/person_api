-- Your SQL goes here
CREATE TABLE "address"(
    id SERIAL PRIMARY KEY NOT NULL,
    state VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    zip_code VARCHAR NOT NULL,
    neighborhood VARCHAR DEFAULT NULL,
    complement VARCHAR DEFAULT NULL,
    number VARCHAR NOT NULL,
    person_id INTEGER NOT NULL,
    CONSTRAINT fk_address_person FOREIGN KEY(person_id) REFERENCES person(id)
)