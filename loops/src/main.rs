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
//  let mut count = 0;
//  'counting_up: loop{
//      println!("count = {count}");
//      let mut remaining = 10;

//      loop {
//          println!("remaining = {remaining}");
//          if remaining == 9 {
//              break;
//          }
//          if count == 2 {
//              break 'counting_up;
//          }
//          remaining -= 1;
//          }
//      count += 1;
//  }
//  println!("End count = {count}");
//---------  while loops  ------
//  let mut number = 3;

//  while number != 0 {
//      println!("{number}");

//      number -= 1;
//  }
//  println!("LIFTOF!!!");
    let a = [ 10, 20, 30, 40, 50];
    let mut index = 0;
    for element in a {
        println!("the value is: {element}");
    }
    while index < 5 {
        println!("the valeu is: {}", a[index]);
        index += 1;
    }
    
    for number in (1..20).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

