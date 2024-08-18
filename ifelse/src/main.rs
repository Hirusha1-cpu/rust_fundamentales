use core::num;
use std::f64;

//if else
#[derive(Debug)]
enum IpAddr1{
    V4(u8,u8,u8,u8),
    V6(String)
}
#[derive(Debug)]

struct Color1(i32, i32, i32);

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
}
fn main() {
    println!("Hello, world!");
    let age: u16 = 12;
    if age >= 19{
        println!("you can get drive {}",age)
    }else{
        println!("you cannot get drive {}",age)
    }

    let condition = true;
    let _number = if condition { 1 } else { 0 };
    println!("{_number}");

    //loop
    let mut counter = 0;
    let _result = loop{
        counter += 1;

        if counter == 10{
            break counter * 2;
        };
    };
    print!("{counter}");

    //while loop
    let mut _counter1 = 10;
    while _counter1 != 0{
        print!("{_counter1}");
        _counter1 -= 1;
        // break;
    }
    println!("Heyyy");

    //for loop
    let a = [1,2,3,4,5,6,7,8];
    for element in a {
        println!("{}", element);
    }

    let rect= (200,300);
    //struct
    struct Rect{
        title:String,
        author:String,
        pages:u32,
        available:bool
      
    }

    

    let mut user1 = User{
        active: false,
        username: String::from("hirushafernando"),
    };
    user1.username = String::from("hirushaf");
    println!("{}",user1.username);

    fn build_user(email: String, username: String)-> User {
        User{
            active: false,
            username: String::from("hirushafernando"),
        }
    }

    //create instances from other instances
    let mut user2 = User{
        username: String::from("hirushafqqqq"),
        ..user1 // user 1 gen anit ewa aran meken witrak gnnw me reasigned value eka
    };

    print!("{}",user2.active);
    print!("{:?}",user2);

    // tuple structs
    
    let _black = Color1(0,0,0);
    println!("{:?}",_black);

    //unit-like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // print!("{subject}")
    enum IpAddrKind{
        V4,
        V6
    }
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind){}
        route(IpAddrKind::V4);
        route(IpAddrKind::V6);


    struct IpAddr{
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("127.0.0.1"),
    };

 
    let home1 = IpAddr1::V4(127,0,0,1);
    let loopbaks = IpAddr1::V6(String::from("::1"));

    print!("{:?}",home1);
    print!("{:?}",loopbaks);


   enum Option<T>{ //if t exists
    Some(T),
    None
   }
   enum Result<T,E>{ //if t exists
    Ok(T),
    Err(E)
   }

   let results = divide(23.0, 6.0);
   match results {
       Some(x) => print!("{:?}",x),
       None => print!("cannot devide"),
   }
   let _results_divide = divideErr(23.0, 6.0);
   match _results_divide {
      Ok(x) => print!("{}",x),
      Err(e)=>println!("{}",e)
   }

   
   
 
}
fn divide(numerator:f64, denominator:f64)-> Option<f64>{
    if denominator == 0.0{
        None
    }else{
        Some(numerator/denominator)
    }
}
fn divideErr(numerator1:f64, denominator2:f64)-> Result<f64,String>{
    if denominator2 == 0.0{
        Err("Cannot divide".to_string())
    }else{
        Ok(numerator1/denominator2)
    }
}
