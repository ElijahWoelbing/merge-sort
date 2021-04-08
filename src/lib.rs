fn main() {
    let mut numbers = vec![38, 27, 43, 3, 9, 82, 10];
    let last = numbers.len() - 1;
    merge_sort(&mut numbers, 0, last);
    println!("{:#?}", numbers);
    // assert_eq!(numbers, vec![3, 9, 10, 27, 38, 43, 82]);
}

fn merge_sort(arr: &mut Vec<i32>, l: usize, r: usize) {
    // find middle index
    if r > l {
        let m = l + (r - l) / 2;
        // recursive call to left half
        merge_sort(arr, l, m);
        // recursive call to right half
        merge_sort(arr, m + 1, r);
        // merge the sorted halfs
        merge(arr, l, m, r);
    }
}

fn merge(arr: &mut Vec<i32>, l: usize, m: usize, r: usize) {
    // size of temp arrays
    let l_temp_size = m - l + 1;
    println!("{}", l_temp_size.to_string());
    let r_temp_size = r - m;
    println!("{}", l_temp_size.to_string());

    let mut l_temp_arr: Vec<i32> = vec![0; l_temp_size];
    println!("{}", l_temp_arr[0]);
    let mut r_temp_arr: Vec<i32> = vec![0; r_temp_size];
    // copy values to temporary arrays
    for i in 0..l_temp_size {
        l_temp_arr[i] = arr[l + i];
    }
    for j in 0..r_temp_size {
        r_temp_arr[j] = arr[m + 1 + j];
    }

    let mut i = 0;
    let mut j = 0;
    // start of merged sub array
    let mut k = l;

    while i < l_temp_size && j < r_temp_size {
        if l_temp_arr[i] <= r_temp_arr[j] {
            arr[k] = l_temp_arr[i];
            i += 1;
        } else {
            arr[k] = r_temp_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < l_temp_size {
        arr[k] = l_temp_arr[i];
        i += 1;
        k += 1;
    }

    while j < r_temp_size {
        arr[k] = r_temp_arr[j];
        j += 1;
        k += 1;
    }
}
//            [38,27,43,3,9,82,10]
//               /              \
//      [38,27,43,3]        [9,82,10]
//        /     \             /     \
//  [38,27]   [43,3]       [9,82]   [10]
//   /    \     /  \        /  \       \
// [38] [27]  [43],[3]   [9] [82]     [10]
//     |          |          |          |
// [27, 38]   [3, 43]     [9,82]       [10]

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut numbers = vec![38, 27, 43, 3, 9, 82, 10];
        let last = numbers.len() - 1;
        merge_sort(&mut numbers, 0, last);
        println!("{:#?}", numbers);
        assert_eq!(numbers, vec![3, 9, 10, 27, 38, 43, 82]);
    }
}
