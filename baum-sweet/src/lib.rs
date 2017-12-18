mod bs {
    fn baum_sweet(x: u64) -> u8 {
        let mut bits = x;

        loop {
            if bits == 0 {
                return 1;
            };
            let n = bits.trailing_zeros();
            if n % 2 == 1 {
                return 0;
            };
            bits >>= n + 1;
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_some() {
            let op = baum_sweet(4);
            assert_eq!(op, 1);

            let op2 = baum_sweet(19611206);
            assert_eq!(op2, 0);
        }
    }

}
