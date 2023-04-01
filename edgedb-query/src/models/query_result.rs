use crate::{EdgeResult, ToEdgeQl, ToEdgeShape};
use edgedb_derive::Queryable;
use uuid::Uuid;

const STRUCT_ID: &str = "{ id }";

/// BasicResult represents the default edgeDB query result
#[derive(Default, Queryable)]
pub struct BasicResult {
    pub id: Uuid,
}

impl ToEdgeShape for BasicResult {
    fn shape() -> String {
        String::default()
    }
}

impl ToEdgeQl for BasicResult {
    fn to_edgeql(&self) -> String {
        STRUCT_ID.to_owned()
    }
}

impl EdgeResult for BasicResult {
    fn returning_fields() -> Vec<&'static str> {
        vec![]
    }
}