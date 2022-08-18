fn main() {
    let mut arr: [i8; 8] = [5, 1, 3, 6, 4, 7, 2, -9];
    for i in 0..arr.len() {
        let mut smaller: i8 = arr[i];
        let mut smaller_index: usize = i;
        for j in i..arr.len() {
            if smaller > arr[j] {
                smaller = arr[j];
                smaller_index = j;
            }
        }
        arr[smaller_index] = arr[i];
        arr[i] = smaller;
    }

    for i in 0..arr.len() {
        print!("{} \t", arr[i]);
    }
}
