use std::collections::HashMap;

use rusoto_dynamodbstreams::Record;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamoDbStreamEvent {
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
}

/// Annoyingly, dynomite can not use the dynamodb_streams AttributeValue,
/// so we need to do a convertion.
pub fn dynamodb_stream_attrs_to_dynamodb_attrs(
    attrs: &HashMap<String, rusoto_dynamodbstreams::AttributeValue>,
) -> HashMap<String, dynomite::AttributeValue> {
    attrs
        .into_iter()
        .map(|(k, v)| (k.clone(), attribute_mapper(v)))
        .collect()
}

fn attribute_mapper(v: &rusoto_dynamodbstreams::AttributeValue) -> dynomite::AttributeValue {
    dynomite::AttributeValue {
        s: v.s.clone(),
        bool: v.bool,
        n: v.n.clone(),
        ns: v.ns.clone(),
        null: v.null,
        m: v.m
            .clone()
            .map(|x| dynamodb_stream_attrs_to_dynamodb_attrs(&x)),
        l: v.l
            .clone()
            .map(|x| x.into_iter().map(|a| attribute_mapper(&a)).collect()),
        ss: v.ss.clone(),
        // Hack to convert between the two different versions of the `bytes` crate
        b: v.b.clone().map(|bytes| bytes.into_iter().collect()),
        bs: v.bs.clone().map(|vec_bytes| {
            vec_bytes
                .into_iter()
                .map(|bytes| bytes.into_iter().collect())
                .collect()
        }),
    }
}
