fn main() {
    let mut vec = vec![9,3,1,6,8,2,0,-1,100,-2,200,300];
    let n = merge_sort(&mut vec);
    println!("{:?}",n);
}


fn menor(arr: &[i32]) -> usize {
    let mut menor = arr[0];
    let mut ind: usize = 0;

    for indx in 1..arr.len() {
        if arr[indx] < menor {
            menor = arr[indx];
            ind = indx;
        }
    }

    ind
}


fn merge_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    let mut novo_vetor: Vec<i32> = Vec::new();

    for indx in 0..arr.len() {
        let menor = menor(&arr);
        novo_vetor.push(arr.remove(menor));
    }

    novo_vetor
}