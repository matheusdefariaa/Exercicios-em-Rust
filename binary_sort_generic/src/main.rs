fn main() {
    let arr_1 = [10,20,30,40,50,60,70,80,90];
    let b_1 = binary_sort(&arr_1,60).unwrap();
    println!("B1 = {b_1}");

    let arr_2 = ['a','b','c','d','e','f','g','h','i'];
    let b_2 = binary_sort(&arr_2,'h').unwrap();
    println!("B2 = {b_2}");
}

fn binary_sort<T: std::cmp::PartialEq + std::cmp::PartialOrd>(arr: &[T],n: T) -> Option<usize> {
    
    let mut start: usize = 0;
    let mut end = arr.len();

    while start <= end {
        let middle = (start + end) / 2;

        if arr[middle] == n {
            return Some(middle);
        }

        if arr[middle] < n {
            start = middle + 1;
        }

        else {
            end = middle - 1;
        }
    }
    None
}