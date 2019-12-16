/*
    This program showcases the functionality of the 'ownership' concept
    In the Rust programming language
*/
fn main() {
    //This is a string literal, and cannot be changed. Stored on the stack.
    let str1 = "Hello, World!";
    println!("{}", str1);
    //This other string type is stored on the heap, and thus can be changed.
    let mut str2 = String::from("Hello,");
    str2.push_str(" World!");
    println!("{}", str2);

    {
        //Let's allocate on the heap here
        let mut str3 = String::from("Hi, ");
        str3.push_str("again!");
    } // The function 'drop' is run, and the memory to the pointer associated to str3 is now free.
    println!("{}", str2);

    //In many programming languages, the two lones below would mean that 'str5' copies the
    //pointer, length, and the capacity of 'str4' (The stack data), and not the value (heap data)
    //same value.
    let str4 = String::new();

    //In Rust, this means that 'str4' is no longer valid, which helps prevent memory corruption
    //when trying to drop memory twice for example. This is also called 'shadow copy'
    let str5 = str4;

    let str6 = String::new();
    //This line copies the heap data aswell, and allocates memory in the heap.
    //This is called 'deep copy'
    let str7 = str6.clone();

    //References, pass a raference to a variable instead of moving the owner.
    let str8 = String::from("foobar");
    let str9 = calc_str_len(&str8);


    //This is not allowed, only one mut reference per poiner is allowed, to prevent 'race'
    let mut s = String::from("hello");

    //This is allowed though, since 'r1' is dropped inside the brackets.
    let r1 = &mut s;
    {
        let r2 = &mut s;
    }

    //This produces a dangling pointer, which is not allowed in Rust. Thank the gods!!
    let reference_to_nothing = dangle();
}

fn calc_str_len(x: &String) {
    x.len();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
