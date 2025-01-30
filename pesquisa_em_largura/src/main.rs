use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let mut pessoas: HashMap<&str,Vec<&str>> = HashMap::new();

    pessoas.insert("Matheus", vec!["Ana","Julia"]);
    pessoas.insert("Ana", vec!["Carlos"]);
    pessoas.insert("Julia", vec![]);
    pessoas.insert("Carlos", vec!["Lara"]);
    pessoas.insert("Lara", vec![]);

    let res = pesquisa_em_largura("Matheus", pessoas);
    println!("{res}");
}

fn comeca_com_m(p: &str) -> bool {
    p.starts_with("L")
}

fn pesquisa_em_largura(nome: &str,lista: HashMap<&str,Vec<&str>>) -> bool {
    // let mut fila: VecDeque<HashMap<&str,Vec<&str>>> = VecDeque::new();
    // let mut fila: VecDeque<&Vec<&str>> = VecDeque::new();
    let mut fila: VecDeque<&str> = VecDeque::new();
    let mut verificados: Vec<&str> = Vec::new();

    fila.push_back(nome);

    while !fila.is_empty() {
        let p1 = fila.pop_front().unwrap();

        if !verificados.contains(&p1) {
            if comeca_com_m(p1) {
                println!("{:?}",p1);
                return true
            }

            verificados.push(&p1);

            if let Some(amigos) = lista.get(p1) {
                for &amigo in amigos {
                    if !verificados.contains(&amigo) {
                        fila.push_back(amigo);
                    }
                }
            }
        }
    }

    false
}
