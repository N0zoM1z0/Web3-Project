use  std::io::{self, Write};

fn main() {
    println!("Hello, world!");
    let s = get_user_input();
    let mut buffer = s.as_bytes().to_vec();
    buffer.extend_from_slice(b"\n");

    io::stdout()
        .write_all(&buffer)
        .expect("error");
    basicTypeLearning();
    sliceTest();
}

fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

fn basicTypeLearning(){
    let a = 10;
    let b:i32 = 20;
    let mut c = 30i32;
    let d = 30_i32;
    let e = add(add(a,b),add(c,d));
    println!("( a + b ) + ( c + d ) = {}", e);
}

fn add(a:i32, b:i32) -> i32{
    return a + b;
}

fn sliceTest(){
    let (a,b,c,d,e);
    (a,b) = (1,2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    println!("[+] PASSED");

}
struct Struct{
    e: i32
}