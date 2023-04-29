use std::{fmt::Debug, ops::Add};

use super::{Convert, Coord, Size};

#[derive(Clone, Debug)]
pub struct Polygon<T> {
    points: Vec<Coord<T>>,
}

impl<T> Polygon<T> {
    pub fn new(points: &[Coord<T>]) -> Self
    where
        T: Copy + Debug + PartialEq,
    {
        assert!(!points.is_empty(), "points must not be empty");
        assert_ne!(
            points.first().unwrap(),
            points.last().unwrap(),
            "first and last point should not be equal since the polygon is automatically closed"
        );

        Self {
            points: points.to_vec(),
        }
    }

    pub fn rect(origin: Coord<T>, size: Size<T>) -> Self
    where
        T: Add<Output = T> + Copy,
    {
        Self {
            points: vec![
                origin,
                Coord {
                    x: origin.x + size.width,
                    y: origin.y,
                },
                Coord {
                    x: origin.x + size.width,
                    y: origin.y + size.height,
                },
                Coord {
                    x: origin.x,
                    y: origin.y + size.height,
                },
            ],
        }
    }

    pub fn points(&self) -> &[Coord<T>] {
        &self.points
    }

    pub fn convert<U>(self, converter: &impl Convert<T, U>) -> Polygon<U> {
        Polygon {
            points: self
                .points
                .into_iter()
                .map(|point| point.convert(converter))
                .collect(),
        }
    }

    pub fn max_x(&self) -> T
    where
        T: Copy + PartialOrd,
    {
        self.points
            .iter()
            .max_by(|point1, point2| {
                point1
                    .x
                    .partial_cmp(&point2.x)
                    .expect("Points must be comparable")
            })
            .unwrap()
            .x
    }
}
