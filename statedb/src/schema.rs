// @generated automatically by Diesel CLI.

pub mod state {
    diesel::table! {
        state.nodes (hash) {
            hash -> Text,
            data -> Text,
        }
    }

    diesel::table! {
        state.program (hash) {
            hash -> Text,
            data -> Text,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(
        nodes,
        program,
    );
}
