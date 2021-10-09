struct Alice{
}
impl Alice {
    fn new() -> Self{
        Self {
        }
    }
}

struct Bob {
}
impl Bob {
    fn new() -> Self {
        Self {
        }
    }
    fn inst_of_bob(&self) {
        println!("hi");
    }
}



fn main() {
    let bob = Bob::new();    
    Bob::inst_of_bob(&bob);
    bob.inst_of_bob();
    let alice = Alice::new();
}

trait Describer {
    fn describe(&self); 
    }
}
imple Describer for Bob{
    ifn descrbe(&self) {
        eprintln!("This is a Bob");
    }
}
