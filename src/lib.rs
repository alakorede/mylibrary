
pub mod calc_with_1 {
/// # This function sum X to Y and add plus 1
/// 
///  #exemple
/// 
/// ```rust
/// use mylibrary::calc_with_1::sum_plus_one;
/// 
/// assert_eq!(42, sum_plus_one(41, 0));
/// assert_eq!(4, sum_plus_one(1, 2));
/// assert_eq!(1, sum_plus_one(0, 0));
/// ```
    pub fn sum_plus_one(x: u8, y: u8) -> u8 {
        x + y +1
    }
/// # This function removes Y from X and less 1
/// 
/// - If first parameter is shorter or equal second, returns 0
/// 
///  #exemple
/// 
/// ```rust
/// use mylibrary::calc_with_1::sub_less_one;
/// 
/// assert_eq!(40, sub_less_one(41, 0));
/// assert_eq!(0, sub_less_one(6, 6));
/// assert_eq!(0, sub_less_one(5, 50));
/// ```
    pub fn sub_less_one(x: u8, y: u8) -> u8 {
        if x <= (y +1) {
            return 0;
        }
        x - y -1
    }
}

#[cfg(test)]

mod test{
    use super::calc_with_1;

    #[test]
    fn test_sum() {
        let result = calc_with_1::sum_plus_one(5, 6);
        let expected = 12;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_fail() {
        let result = calc_with_1::sub_less_one(5, 6);
        let expected = 0;
        assert_eq!(result, expected);
    }

     #[test]
    fn test_sub() {
        let result = calc_with_1::sub_less_one(7, 5);
        let expected = 1;
        assert_eq!(result, expected);
    }

}