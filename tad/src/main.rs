// TAD

struct Notas {
    n1: f32,
    n2: f32,
    n3: f32,
    n4: f32,
}

impl Notas {

    fn new() -> Self {
        Self {
            n1: 0.0,
            n2: 0.0,
            n3: 0.0,
            n4: 0.0,
        }
    }

    fn append(&mut self,n_1:f32,n_2:f32,n_3:f32,n_4:f32) {
        self.n1 = n_1;
        self.n2 = n_2;
        self.n3 = n_3;
        self.n4 = n_4;
    }

    fn approved(&self) -> bool {
        let media = (self.n1 + self.n2 + self.n3 + self.n4) / 4.0;

        media >= 7.0
    }
}

fn main() {

    let mut p1 = Notas {
        n1: 1.0,
        n2: 1.0,
        n3: 1.0,
        n4: 1.0,
    };
    p1.append(p1.n1, p1.n2, p1.n3, p1.n4);
    println!("{}",p1.approved());

    let mut p2 = Notas::new();
    p2.append(8.0, 4.4, 6.1, 2.3);
    println!("{}",p2.approved())
    
}