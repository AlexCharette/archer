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
    blocks (block_num) {
        block_num -> Int8,
        block_id -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    blocks,
);
