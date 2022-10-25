use std::ops::Sub;

pub fn min<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1 < v2 {
        v1
    } else {
        v2
    }
}

pub fn max<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}

pub fn difference<T: PartialOrd + Sub<Output = T>>(v1: T, v2: T) -> T {
    if v1 > v2 {
        v1 - v2
    } else {
        v2 - v1
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    #[rstest]
    #[case(-1, 1, -1)]
    #[case(1, -1, -1)]
    fn min(#[case] v1: i64, #[case] v2: i64, #[case] expected: i64) {
        assert_eq!(super::min(v1, v2), expected);
    }

    #[rstest]
    #[case(-1, 1, 1)]
    #[case(1, -1, 1)]
    fn max(#[case] v1: i64, #[case] v2: i64, #[case] expected: i64) {
        assert_eq!(super::max(v1, v2), expected);
    }

    #[rstest]
    #[case(-1, 1, 2)]
    #[case(1, -1, 2)]
    #[case(1, 1, 0)]
    fn difference(#[case] v1: i64, #[case] v2: i64, #[case] expected: i64) {
        assert_eq!(super::difference(v1, v2), expected);
    }
}
