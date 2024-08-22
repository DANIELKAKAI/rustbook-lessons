fn main() {
    
    let mut a = String::from("hello");
    
    let mut x = &mut a;
    
    add(&mut x);
    
    println!("{x}"); // goes out of scope after printing
    
    
    //because x is mutable it must go out of scope to use a
    println!("{a}");
    
    let s = String::from("hello");
    
    let size = str_len(&s);
    
    println!("{size}");
}

fn add(s : &mut String){
    s.push_str(" world");
}

fn str_len(s: &String) -> usize{
    s.len()
}