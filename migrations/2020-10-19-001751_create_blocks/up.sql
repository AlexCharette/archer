-- Your SQL goes here

CREATE TABLE blocks (
    block_num BIGINT PRIMARY KEY,
    block_id  VARCHAR NOT NULL
);

CREATE TABLE auth (
    public_key VARCHAR PRIMARY KEY,
    hashed_password VARCHAR,
    encrypted_private_key VARCHAR,
);

CREATE TABLE accounts (
    name VARCHAR,
    number INTEGER,
    balance INTEGER NOT NULL DEFAULT 0,
    start_block_num BIGINT REFERENCES blocks(block_num),
    end_block_num BIGINT REFERENCES blocks(block_num),
    PRIMARY KEY(name, number)
);

CREATE TABLE merchants (
    name VARCHAR,
    public_key VARCHAR,
    timestamp BIGINT,
    start_block_num BIGINT REFERENCES blocks(block_num),
    end_block_num BIGINT REFERENCES blocks(block_num),
);
