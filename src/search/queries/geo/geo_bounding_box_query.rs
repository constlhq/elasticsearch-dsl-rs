use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Matches [geo_point](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
/// and [geo_shape](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html)
/// values that intersect a bounding box.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-bounding-box-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct GeoBoundingBoxQuery {
    #[serde(flatten)]
    pair: KeyValuePair<String, GeoBoundingBox>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    validation_method: Option<ValidationMethod>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}
impl Query {
    /// Creates an instance of [`GeoBoundingBoxQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - A series of vertex coordinates of a geo bounding box
    pub fn geo_bounding_box<T, U>(field: T, value: U) -> GeoBoundingBoxQuery
    where
        T: Into<String>,
        U: Into<GeoBoundingBox>,
    {
        GeoBoundingBoxQuery {
            pair: KeyValuePair::new(field.into(), value.into()),
            validation_method: None,
            boost: None,
            _name: None,
        }
    }
}

impl GeoBoundingBoxQuery {
    /// Set to `IGNORE_MALFORMED` to accept geo points with invalid latitude or longitude, set to
    /// `COERCE` to also try to infer correct latitude or longitude. (default is `STRICT`).
    pub fn validation_method(mut self, validation_method: ValidationMethod) -> Self {
        self.validation_method = Some(validation_method);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for GeoBoundingBoxQuery {
    fn should_skip(&self) -> bool {
        self.pair.key.should_skip()
    }
}

serialize_query!("geo_bounding_box": GeoBoundingBoxQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::geo_bounding_box(
                "pin.location",
                GeoBoundingBox::WellKnownText {
                    wkt: "BBOX (-74.1, -71.12, 40.73, 40.01)".into(),
                },
            ),
            json!({
                "geo_bounding_box": {
                    "pin.location": {
                        "wkt": "BBOX (-74.1, -71.12, 40.73, 40.01)"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::geo_bounding_box(
                "pin.location",
                GeoBoundingBox::MainDiagonal {
                    top_left: GeoPoint::Coordinates {
                        longitude: -74.1,
                        latitude: 40.73,
                    },
                    bottom_right: GeoPoint::Coordinates {
                        longitude: -71.12,
                        latitude: 40.01,
                    },
                },
            )
            .validation_method(ValidationMethod::Strict)
            .name("test_name"),
            json!({
                "geo_bounding_box": {
                    "validation_method": "STRICT",
                    "_name": "test_name",
                    "pin.location": {
                        "top_left": [ -74.1, 40.73 ],
                        "bottom_right": [ -71.12, 40.01 ]
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::geo_bounding_box(
                "pin.location",
                GeoBoundingBox::Vertices {
                    top: 40.73,
                    left: -74.1,
                    bottom: 40.01,
                    right: -71.12,
                },
            )
            .validation_method(ValidationMethod::Strict)
            .name("test_name")
            .boost(1),
            json!({
                "geo_bounding_box": {
                    "validation_method": "STRICT",
                    "_name": "test_name",
                    "boost": 1,
                    "pin.location": {
                        "top": 40.73,
                        "left": -74.1,
                        "bottom": 40.01,
                        "right": -71.12
                    }
                }
            }),
        )
    }
}
