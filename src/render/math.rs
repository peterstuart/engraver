pub fn max<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    #[rstest]
    #[case(-1, 1, 1)]
    #[case(1, -1, 1)]
    fn max(#[case] v1: i64, #[case] v2: i64, #[case] expected: i64) {
        assert_eq!(super::max(v1, v2), expected);
    }
}
