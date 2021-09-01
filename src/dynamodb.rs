use std::collections::HashMap;

use rusoto_dynamodbstreams::Record;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamoDbStreamEvent {
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
}

/// Annoyingly, dynomite can not use the dynamodb_streams AttributeValue,
/// so we need to do a convertion.
pub fn dynamodb_stream_attrs_to_dynamodb_attrs(
    attrs: HashMap<String, rusoto_dynamodbstreams::AttributeValue>,
) -> HashMap<String, dynomite::AttributeValue> {
    attrs
        .into_iter()
        .map(|(k, v)| (k, attribute_mapper(v)))
        .collect()
}

fn attribute_mapper(v: rusoto_dynamodbstreams::AttributeValue) -> dynomite::AttributeValue {
    dynomite::AttributeValue {
        s: v.s,
        bool: v.bool,
        n: v.n,
        ns: v.ns,
        null: v.null,
        m: v.m.map(dynamodb_stream_attrs_to_dynamodb_attrs),
        l: v.l.map(|x| x.into_iter().map(attribute_mapper).collect()),
        ss: v.ss,
        // Hack to convert between the two different versions of the `bytes` crate
        b: v.b.map(|bytes| bytes.into_iter().collect()),
        bs: v.bs.map(|vec_bytes| {
            vec_bytes
                .into_iter()
                .map(|bytes| bytes.into_iter().collect())
                .collect()
        }),
    }
}