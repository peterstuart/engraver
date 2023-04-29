pub fn max<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn max() {
        assert_eq!(super::max(-1, 1), 1);
        assert_eq!(super::max(1, -1), 1);
    }
}
