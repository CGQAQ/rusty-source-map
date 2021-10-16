pub const GREATEST_LOWER_BOUND: i32 = 1;
pub const LEAST_UPPER_BOUND: i32 = 2;

///
/// Recursive implementation of binary search.
///
/// @param aLow Indices here and lower do not contain the needle.
/// @param aHigh Indices here and higher do not contain the needle.
/// @param aNeedle The element being searched for.
/// @param aHaystack The non-empty array being searched.
/// @param aCompare Function which takes two elements and returns -1, 0, or 1.
/// @param aBias Either 'binarySearch.GREATEST_LOWER_BOUND' or
///     'binarySearch.LEAST_UPPER_BOUND'. Specifies whether to return the
///     closest element that is smaller than or greater than the one we are
///     searching for, respectively, if the exact element cannot be found.
/// ref: https://github.com/mozilla/source-map/blob/58819f09018d56ef84dc41ba9c93f554e0645169/lib/binary-search.js#L24
fn recursive_search<T>(
    low: i32,
    high: i32,
    needle: T,
    hay_stack: Vec<T>,
    compare: &impl Fn(&T, &T) -> i32,
    bias: i32,
) -> i32
where
    T: Clone,
{
    // This function terminates when one of the following is true:
    //
    //   1. We find the exact element we are looking for.
    //
    //   2. We did not find the exact element, but we can return the index of
    //      the next-closest element.
    //
    //   3. We did not find the exact element, and there is no next-closest
    //      element than the one we are searching for, so we return -1.

    let mid: i32 = (high - low) / 2 + low;
    let cmp = compare(&needle, &hay_stack[mid as usize]);

    if cmp == 0 {
        // Found the element we are looking for.
        return mid;
    } else if cmp > 0 {
        // Our needle is greater than aHaystack[mid].
        if high - mid > 1 {
            // The element is in the upper half.
            return recursive_search(mid, high, needle, hay_stack.clone(), compare, bias);
        }

        // The exact needle element was not found in this haystack. Determine if
        // we are in termination case (3) or (2) and return the appropriate thing.
        if bias == LEAST_UPPER_BOUND {
            return if high < hay_stack.len() as i32 {
                high
            } else {
                -1
            };
        }
        return mid;
    }

    // Our needle is less than aHaystack[mid].
    if mid - low > 1 {
        return recursive_search(low, mid, needle, hay_stack.clone(), compare, bias);
    }

    // we are in termination case (3) or (2) and return the appropriate thing.
    if bias == LEAST_UPPER_BOUND {
        return mid;
    }

    return if low < 0 { -1 } else { low };
}

pub fn search<T>(
    needle: T,
    hay_stack: Vec<T>,
    compare: impl Fn(&T, &T) -> i32,
    bias: Option<i32>,
) -> i32
where
    T: Clone,
{
    if hay_stack.len() == 0 {
        return -1;
    }

    let mut index = recursive_search(
        -1,
        hay_stack.len() as i32,
        needle,
        hay_stack.clone(),
        &compare,
        bias.unwrap_or(GREATEST_LOWER_BOUND),
    );
    if index < 0 {
        return -1;
    }

    // We have found either the exact element, or the next-closest element to
    // the one we are searching for. However, there may be more than one such
    // element. Make sure we always return the smallest of these.
    while index - 1 >= 0 {
        if compare(&hay_stack[index as usize], &hay_stack[(index - 1) as usize]) != 0 {
            break;
        }
        index -= 1;
    }

    return index;
}

#[cfg(test)]
mod test {
    use super::*;

    fn number_cmp(a: &i32, b: &i32) -> i32 {
        a - b
    }

    #[test]
    fn test_too_high_with_default_bias() {
        let needle = 30;
        let hay_stack = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];

        assert_eq!(
            hay_stack[search(needle, hay_stack.clone(), number_cmp, None) as usize],
            20
        );
    }

    #[test]
    fn test_too_low_with_default_bias() {
        let needle = 1;
        let hay_stack = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
        assert_eq!(
            hay_stack[search(
                needle,
                hay_stack.clone(),
                number_cmp,
                Some(LEAST_UPPER_BOUND)
            ) as usize],
            2
        )
    }

    #[test]
    fn test_exact_search() {
        let needle = 4;
        let hay_stack = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];

        assert_eq!(
            hay_stack[search(needle, hay_stack.clone(), number_cmp, None) as usize],
            4
        )
    }

    #[test]
    fn test_fuzzy_search_with_glb_bias() {
        let needle = 19;
        let hay_stack = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];

        assert_eq!(
            hay_stack[search(needle, hay_stack.clone(), number_cmp, None) as usize],
            18
        )
    }

    #[test]
    fn test_fuzzy_search_with_lub_bias() {
        let needle = 19;
        let hay_stack = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];

        assert_eq!(
            hay_stack[search(
                needle,
                hay_stack.clone(),
                number_cmp,
                Some(LEAST_UPPER_BOUND)
            ) as usize],
            20
        )
    }

    #[test]
    fn test_multiple_matches() {
        let needle = 5;
        let hay_stack = vec![1, 1, 2, 5, 5, 5, 13, 21];
        assert_eq!(
            search(
                needle,
                hay_stack.clone(),
                number_cmp,
                Some(LEAST_UPPER_BOUND)
            ),
            3
        )
    }

    #[test]
    fn test_multiple_matches_at_beginning() {
        let needle = 1;
        let hay_stack = vec![1, 1, 2, 5, 5, 5, 13, 21];

        assert_eq!(
            search(
                needle,
                hay_stack.clone(),
                number_cmp,
                Some(LEAST_UPPER_BOUND)
            ),
            0
        )
    }
}
