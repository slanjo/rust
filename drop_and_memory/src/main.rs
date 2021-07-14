fn make_and_use_vec() {
    { 
    let mut vec = vec![1, 2, 3];   
    eprintln!("{:?}", vec);
    } // <-- memory ov 'vec' is marked as free
//    eprintln!("{:?}", vec);    
}// <-- memory ov 'vec' is marked as free 

fn main() {
    make_and_use_vec();
}

