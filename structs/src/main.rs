struct Computer {
    name: String,
    cpu_count: u16,
    ram: usize, 
}
//struct ServerRack {
//    slot_1: Computer;
//    slot_1: Computer;
//    slot_1: Computer;       
//}

fn new_create_computer(a_name: String, cpu_count: u16, ram: usize) -> Computer {
    Computer {
        name: a_name, 
        cpu_count: cpu_count,
        ram: ram, 
    }
}
//fn create_computer(a_name: String, cpu_count: u16, ram: usize) -> Computer {
//   Computer {
//       name: a_name, 
//       cpu_count: cpu_count,
//       ram: ram, 
//   }
//
fn main() {
let computer = create_computer("Bob".to_string(), 3,  16_000_000);
eprintln!("{:?}", computer.ram);
}
