use crate::search::*;
use crate::util::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
/// A multi-bucket value source based aggregation where buckets are dynamically built - one per unique value.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html>
pub struct NestedAggregation {
    nested: NestedAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct NestedAggregationInner {
    path: String,
}

impl NestedAggregation {
    add_aggregate!();
}

// impl  NestedAggregationInner{
//     pub fn new()
// }

impl Aggregation {
    /// foo
    pub fn nested<T>(path: T) -> NestedAggregation
    where
        T: ToString,
    {
        NestedAggregation {
            nested: NestedAggregationInner {
                path: path.to_string(),
            },
            aggs: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::nested("test_field"),
            json!({ "nested": { "path": "test_field" } }),
        );
    }
}
