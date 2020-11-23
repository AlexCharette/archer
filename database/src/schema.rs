table! {
    accounts (name, number) {
        name -> Varchar,
        number -> Int4,
        balance -> Int4,
        start_block_num -> Nullable<Int8>,
        end_block_num -> Nullable<Int8>,
    }
}

table! {
    auth (public_key) {
        public_key -> Varchar,
        hashed_password -> Varchar,
        encrypted_private_key -> Varchar,
    }
}

table! {
    blocks (block_num) {
        block_num -> Int8,
        block_id -> Varchar,
    }
}

table! {
    merchants (public_key) {
        name -> Nullable<Varchar>,
        public_key -> Varchar,
        created -> Nullable<Timestamp>,
        start_block_num -> Nullable<Int8>,
        end_block_num -> Nullable<Int8>,
    }
}

joinable!(auth -> merchants (public_key));

allow_tables_to_appear_in_same_query!(accounts, auth, blocks, merchants,);
