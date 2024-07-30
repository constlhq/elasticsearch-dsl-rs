use crate::search::*;
use crate::util::*;
use crate::{Aggregation, Aggregations};

#[derive(Debug, Clone, Serialize, PartialEq)]
/// A multi-bucket value source based aggregation where buckets are dynamically built - one per unique value.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html>
pub struct ReverseNestedAggregation {
    reverse_nested: ReverseNestedAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct ReverseNestedAggregationInner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    path: Option<String>,
}

impl ReverseNestedAggregation {
    add_aggregate!();
}

impl Aggregation {
    pub fn reversed_nested<T>(path: Option<T>) -> ReverseNestedAggregation
    where
        T: ToString,
    {
        ReverseNestedAggregation {
            reverse_nested: ReverseNestedAggregationInner { path: path.map_or(None, |t| Some(t.to_string())) },
            aggs: Aggregations::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::util::assert_serialize_aggregation;
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::reversed_nested::<String>(None),
            json!({ "reverse_nested": { } }),
        );
    }
}