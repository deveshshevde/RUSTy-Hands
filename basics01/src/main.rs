// This contains all the basic implemenation like loos , conditionals etc etc...

fn main() {
    // funtions without an argument !!
    hello_world();
    for_loop();
    while_loop();
    print_vartypes();
}

fn hello_world(){
    println!("Hello world i am new to rust !");
}

fn for_loop(){
    for i in 0..11 {
        println!("Number: {}", i);
    }
}

fn while_loop(){
    //hence counter can change the value we are assigning as mut keyword!!
    let mut counter = 0;
    while counter <= 5 {
        println!("Counter is: {}", counter);
        counter += 1;
    }
    println!("Loop finished!");
}

fn print_vartypes(){
    let int: i32 = 69;
    let float: f64 = 69.69;
    let boolean: bool = false;
    let character: char = 'D';

    println!("Integer: {}", int);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);

}