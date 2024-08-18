fn main() {
    // immutable - cannot be modified
    //primitive data types -> int,float, bool,char
    println!("Hello, hi im from cargo!");
    let x: i32 =  -42;
    let y: u32 = 56;
    let z: f64 = 1.11;
    let w: bool = false;
    let l: char = 'a';
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    println!("float: {}", z);
    println!("Bool: {}", w);
    println!("Char: {}", l);
    
    //compount data types
    let numbers: [i32; 10]= [1,2,3,4,5,6,7,8,9,10];
    // let mix = [1,2,3,4,5,6,7,"wwsw", true]
    println!("Array: {:?}", numbers);
    let fruits: [&str; 3]= ["apple","orange","banana"];
    println!("Array: {:?}", fruits);
    println!("Array: {}", fruits[0]);

    //tuples
    let human: (&str, i32, bool)=("apple",78,true);
    let human1: (String, i32, bool)=("apple".to_string(),78,true);
    println!("Array: {:?}", human);
    println!("Array: {:?}", human1);
    let my_mix_tuple = ("Kratoes",23,true,[1,2,3,4,5,6,7,8,9,10]);
    println!("Array: {:?}", my_mix_tuple);
    
    //slices Slices: [1,2,3,4]
    let number_slices:&[i32] = &[1,2,3,4,5,6,7]; //&[i32],&[&str]
    println!("Number_slices: {:?}", number_slices);

    //strings and string slices
    //strings store in heap
    let mut stone_col: String = String::from("Hello, world!");
    println!("{}",stone_col);
    //stored in heap memory-> permanent memory, stack -> temporary memory
    //when mutable datatype then can be push
    stone_col.push_str("yeah");
    
    //B- &str(String slice)
    //slices store in stack
    let string: String = String::from("Hello, world!");
    //get a reference to slice
    let slice: &str = &string;
    let slice1: &str = &string[0..5];
    println!("{}",slice);
    println!("{}",slice1);
    hello();
    tell_height(122);
    human_id("world",23,21.00);

    let _x:i32={
        let price: i32 = 5;
        let qty: i32 = 51;
         price * qty
    };


    //expressions and statements
    //expressions: Anything that returns a value
    //statements: Anything that not returns a value

    println!("{}",_x);
    let y = add(12, 2);
    println!("{}",y);
    println!("{:.2}", calculate_bmi(70.0, 34.0));//0.06

    let mut _m = 5;
    let _r= &mut _m;

    *_r += 1;
    println!("{}",_r);

    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 160.00
    };
    //immutable borrow to check the balance
    account.check_balance();
    //mutable withdraw money to account
    account.withdraw(50.4);
    account.check_balance();

    let mut _q: i32 = 6;
    println!("immutable defaultly {}",_q);
    _q = 20;
    println!("immutable defaultly {}",_q);

    //constants(const)
    let mut _q1 = 6;
    const P1: i32 = 0; //const is immutable and letter should capital, type should mention
    println!("{},{}",_q1,P1);
    println!("{}",PI);

    //shadowing-reasign
    let z = 9;
    let z = z + 2;

    {
        let z = z * 2;
        print!("{}",z);
    }
 

}

//you can declare constants
const PI: f64 = 90.00222;



//final
fn calculate_bmi(weight: f64,height : f64) -> f64 {
    weight/(height*height)
}

//function returning function
fn add(a:i32, b:i32) -> i32 {
    a+b
}

fn hello() {
    println!("Hello, hi im from cargo!");
}

fn tell_height(height: u32) {
    println!("Height {}", height);
}

fn human_id(name: &str, age: u32,height: f32) {
    println!("My name is {},my age {},my height {} ",name,age,height)
}

struct BankAccount{
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64){
        println!("Account {}, owner {}", self.owner,amount);
        self.balance -= amount;
    }
    fn check_balance(&self){
        println!("Account {}, balance {}", self.owner,self.balance);

    }
}

