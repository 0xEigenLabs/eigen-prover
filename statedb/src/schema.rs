// @generated automatically by Diesel CLI.

pub mod state {
    diesel::table! {
        state.nodes (hash) {
            hash -> Bytea,
            data -> Bytea,
        }
    }

    diesel::table! {
        state.program (hash) {
            hash -> Bytea,
            data -> Bytea,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(nodes, program,);
}
