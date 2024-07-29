/* Binary Search */
fn main() {
    let v: Vec<i32> = vec!(10,20,30,40,50,60,70,80,90,100);
    let qtd = v.len();
    let check = binary_search(v, 0,qtd, 100);
    
    println!("{}",check);
}

fn binary_search(array: Vec<i32>, mut start: usize , mut end: usize , number: i32) -> i32 {
    let mut middle: usize;

    while start < end {
        middle = (start + end) / 2;

        if number == array[middle] {
            return middle as i32
        }

        if number > array[middle] {
            start = middle + 1;
        }
        else {
            end = middle - 1;
        }
    }
    return  -1 // Retorna -1 caso o número não exista no array
}