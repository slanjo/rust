use std::mem::size_of_val;

fn use_slice(bytes: &[u8]) {
    eprintln!("{:#?}| len: {}", bytes, bytes.len());
} 
fn accept_connections_in_a_silly_fashion() {
    let mut connections: Vec<u32> = Vec::new();
    let mut next_connection_id = 0;
    // Loop and accept new connections
    // let connection = server.accept();
    connections.push(next_connection_id);
    next_connection_id += 1;
}
fn accept_connections() {
    let mut connections: Vec<u32> = Vec::new();
    let mut next_connection_id = 0;
    // Loop and accept new connections
    // let connection = server.accept();
    connections.push(next_connection_id);
    next_connection_id += 1;
}
fn main() {
//
//
// 
//    let vec = Vec::new();
//    let vec = Vec::with_capacity(100);
//    let vec = vec![1u32; 100];
//***************ABOVE is useles)
//  let mut vec: Vec<i16> = Vec::with_capacity(2);//create a vector
//  vec.push(34);//push a value to the vector
//  vec.push(34);
//  vec.push(314);
//      vec.push(35);
//      vec.push(34);    
//      vec.push(88);
//  eprintln!("{:#?}", vec);
//  let last_val = vec.pop();
//  eprintln!("last_val: {:?} | {:?}", last_val, vec);
//  let mut arr = [10, 20, 30];
//  arr[0] = 12;
//  eprintln!("{:?}", arr);
//***************
    let mut vec: Vec<u8> = vec![1, 2, 3];
    let arr: [u8; 3] = [1, 2, 3];

    let borrow_array: &[u8; 3] = &arr;
    let borr_vec: &Vec<u8> = &vec;
    vec.push(123);
    use_slice(&arr);
    use_slice(&vec);
    let mut slice: &[u8] = &arr;
    //slice = &vec;

    let mut vec: Vec<u8> = vec![0u8; 1024 * 1024 * 900];
    let mut slice: &[u8] = &vec;
}

