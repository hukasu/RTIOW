use crate::vector::{Direction, Point};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point,
    direction: Direction,
}

impl Ray {
    #[must_use]
    pub fn new(origin: Point, direction: Direction) -> Self {
        Self { origin, direction }
    }

    #[must_use]
    /// Creates a Point in the position that the Ray is point towards.
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn unit_ray(self) -> Self {
        Self {
            origin: self.origin,
            direction: self.direction.unit_vector(),
        }
    }
}

impl Default for Ray {
    /// Creates a new Ray at the origin pointing towards X.
    fn default() -> Self {
        Self::new(Point::new(0., 0., 0.), Direction::new(1., 0., 0.))
    }
}
