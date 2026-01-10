
fn copy(vec: &Vec<i32>, start: usize, end: usize, counter: &mut usize, result: &mut Vec<i32>) {
    for i in start..=end {
        result[*counter] = vec[i];
        *counter += 1;
    }
}

fn merge(vec: &mut Vec<i32>, start: usize, midpoint: usize, end: usize) {
    let mut first_idx = start;
    let mut second_idx = midpoint + 1;
    let mut counter = first_idx;
    let temp = vec.to_vec();

    while first_idx < (midpoint + 1) || second_idx < (end + 1) {
        // if any index is out of bound just copy the whole other vector and break
        if first_idx == (midpoint + 1) {
            copy(&temp, second_idx, end, &mut counter, vec);
            return;
        }
        if second_idx == (end + 1) {
            copy(&temp, first_idx, midpoint, &mut counter, vec);
            return;
        }
        if temp[first_idx] < temp[second_idx] {
            vec[counter] = temp[first_idx];
            first_idx += 1;
        } else {
            vec[counter] = temp[second_idx];
            second_idx += 1;
        }
        counter += 1;
    }
}

fn merge_sort(vec: &mut Vec<i32>, start: usize, end: usize) {
    if end - start == 0 {
        return;
    }
    let midpoint: usize = (end + start) / 2;
    merge_sort(vec, start, midpoint);
    merge_sort(vec, midpoint + 1, end);
    merge(vec, start, midpoint, end);
}

fn main() {
    let mut vec: Vec<i32> = vec![5, 6, 7, 9, 15, 2, 0, 4, -1];
    let len = vec.len();

    merge_sort(&mut vec, 0, len - 1);
    println!("After Sort {:?}", vec);
}
