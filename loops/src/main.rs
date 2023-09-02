fn main() {
//  loop {
//      println!("again!");
//  }
//  let mut counter = 0;
//  
//  let result = loop {
//      counter += 1;

//      if counter == 10 {
//          break counter * 2;
//      }
//  }; //<-- this comma is used to end the statement and assign the value to 
       //result

//    println!("The result is {result}");
//---------  Loop Labels ------
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
            }
        count += 1;
    }
    println!("End count = {count}");
}

