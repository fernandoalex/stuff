// TODO:
// - Get comment tests working
// - improvements to bubble_sort

/// ```
/// 
/// use bubble_sort::bsort;
/// assert_eq!(bsort(vec![3,2,1]), [1,2,3]);
/// ```
pub fn bsort(mut list: Vec<i32>) -> Vec<i32> {


    // Test this:
    // for i in 0..(list.len() - 1) {
    //     for j in i..(list.len() - 1)  {
    for _ in 0..(list.len() - 1) {
        for i in 0..(list.len() - 1)  {
            if list[i] > list[i+1] {
                let buff = list[i];
                list[i] = list[i+1];
                list[i+1] = buff;
            }
        }
    }

    list
}

pub fn ins_sort(mut list: Vec<i32>) -> Vec<i32> {
    for i in 1..list.len() {
        let mut a = i.clone();
        for j in (0..i).rev() {
            if list[a] < list[j] {
                list.swap(a, j);
            }
            a -= 1;
        }
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsort1() {
        let result = bsort(vec![1,3,2,5,4]);
        assert_eq!(result, [1,2,3,4,5])
    }
    #[test]
    fn bsort2() {
        let result = bsort(vec![5,4,1,2,3]);
        assert_eq!(result, [1,2,3,4,5])
    }
    #[test]
    fn bsort_worst_case() {
        let result = bsort(vec![5,4,3,2,1]);
        assert_eq!(result, [1,2,3,4,5])
    }

    #[test]
    fn ins_sort1() {
        let result = ins_sort(vec![1,3,2,5,4]);
        assert_eq!(result, [1,2,3,4,5])
    }
    #[test]
    fn ins_sort2() {
        let result = ins_sort(vec![5,4,1,2,3]);
        assert_eq!(result, [1,2,3,4,5])
    }
    #[test]
    fn ins_sort_worst_case() {
        let result = ins_sort(vec![5,4,3,2,1]);
        assert_eq!(result, [1,2,3,4,5])
    }
}
