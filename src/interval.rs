use std::f64::INFINITY;

#[derive(Debug)]
pub(crate) struct Interval {
    min: f64,
    max: f64,
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl Interval {
    pub(crate) fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    fn default() -> Self {
        Self {
            min: INFINITY,
            max: INFINITY,
        }
    }

    fn size(&self) -> f64 {
        self.max - self.min
    }

    fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub(crate) fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub(crate) fn max(&self) -> f64 {
        self.max
    }

    pub(crate) fn min(&self) -> f64 {
        self.min
    }
}

#[cfg(test)]
mod tests {
    use std::f64::INFINITY;

    use crate::interval::Interval;

    #[test]
    fn test_basic_interval() {
        assert_eq!(Interval::default(), Interval::new(INFINITY, INFINITY));

        let interval = Interval::new(1.0, 4.0);
        assert_eq!(interval.size(), 3.0);
        assert_eq!(interval.min(), 1.0);
        assert_eq!(interval.max(), 4.0);

        assert!(interval.contains(1.0));
        assert!(interval.contains(4.0));
        assert!(!interval.contains(5.0));
        assert!(!interval.contains(-2.0));

        assert!(interval.surrounds(3.0));
        assert!(!interval.surrounds(1.0));
        assert!(!interval.surrounds(4.0));
    }
}
