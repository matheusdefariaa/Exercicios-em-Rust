// Quick Sort

fn main() {
    let v1 = vec!(9,0,6,7,2,3,4,1,10,20);
    println!("{v1:?}");

    let v2 = quick_sort(v1);
    println!("{v2:?}");
}


fn quick_sort(arr: Vec<i32>) -> Vec<i32> {

    if arr.len() <= 1 {
        return arr;
    }

    let pivo = arr.len() / 2;

    let mut menor: Vec<i32> = Vec::new();
    let mut maior: Vec<i32> = Vec::new();

    for indx in 0..arr.len() {
        if arr[indx] == arr[pivo] {
            continue
        }

        if arr[indx] < arr[pivo] {
            menor.push(arr[indx]);
            continue
        }

        maior.push(arr[indx]);
    }
    let mut resultado = quick_sort(menor);
    resultado.push(arr[pivo]);
    resultado.append(&mut quick_sort(maior));
    resultado
}
