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
    if p.starts_with("M") {
        return true
    }
    false
}

fn pesquisa_em_largura(nome: &str,lista: HashMap<&str,Vec<&str>>) -> String {
    let mut fila: VecDeque<HashMap<&str,Vec<&str>>> = VecDeque::new();
    let mut verificados: Vec<String> = Vec::new();

    "a".to_string()
}