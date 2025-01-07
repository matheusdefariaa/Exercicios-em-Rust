// Selection Sort
fn main() {
    let mut numeros_lista: Vec<i32> = vec![6,5,4,3,2,1];
    println!("{numeros_lista:?}");

    let nova_lista = selection_sort(&mut numeros_lista);
    println!("{nova_lista:?}");
}

fn selection_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    let mut nova_lista_numeros: Vec<i32> = Vec::new();

    for elem in 0..arr.len() {
        let menor_index: usize = menor_numero(arr);
        nova_lista_numeros.push(arr[menor_index]);
        arr.remove(menor_index);
    }
    nova_lista_numeros

}

fn menor_numero(arr: &Vec<i32>) -> usize {
    let mut menor: i32 = arr[0];
    let mut menor_index = 0 as usize;

    for elem in 1..arr.len() {
        if arr[elem] < menor {
            menor = arr[elem];
            menor_index = elem;
        }
    };

    menor_index
}