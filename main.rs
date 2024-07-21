// Enterprise level hello world program in Rust

#[derive(PartialEq)]
enum StringType {
    HelloWorld
}

struct OutString {
   r#type: StringType,
    message: String
}

fn new_out_string(message: &str, r#type: StringType) -> Box<OutString> {
    return Box::new(OutString {
        r#type,
        message: String::from(message),
    });
}

// print_hello function prints "Hello, World!" to the console
fn hello_world(outstring: Box<OutString>) {
    if outstring.r#type == StringType::HeloWorld {`
        println!("{}", outstring.message);
    } else {
        println!("Hello World!");
    }
}

fn main() {
    let outstring = new_out_string("Hello World!", StringType::HelloWorld);
    hello_world(outstring);
}
