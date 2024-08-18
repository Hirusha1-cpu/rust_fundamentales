fn main() {
    println!("Hello, world!");
    let mut _v:Vec<i32> =Vec::new();
    let mut _vec:Vec<i32>= vec![1, 2, 3, 4, 5];
    _vec.push(4);
    print!("{:?}",_vec);
    let third:&i32 = &_vec[0];
    print!("{third}");
    let one = _vec.get(0);
    match one{
        Some(one)=>print!("oneee{}",one),
        None => println!(""),
    }
    // print!("{:?}",one);

    let mut s = String::from("Hello, world!");
    s.push_str("seoo");
    print!("{:?}",s);

    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(String::from("ddd"),12);
    let onemap = String::from("ddd");
    let score = map.get(&onemap).copied().unwrap_or_default();

    for(key, val) in &map{
        print!("{key}{val}");
    }

}
