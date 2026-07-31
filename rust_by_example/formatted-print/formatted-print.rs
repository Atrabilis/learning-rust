fn main(){
    //this is an unformatted print, it will be printed as is
    println!("hello world, this is an unformatted print");

    //we can pass raw data to the placeholder, and it will be printed as is
    println!("{}, {} and {} goes to a bar", 
    "Alice", 
    "Bob", 
    "Charlie");

    // or we can use positional arguments to specify the order of the placeholders
    println!("{0}, {2} and {1} goes to a bar", 
    "Alice", 
    "Bob", 
    "Charlie");

    // or we can use named arguments to specify the order of the placeholders
    println!("{name1}, {name2} and {name3} goes to a bar",
    name1="Alice", 
    name2="Bob", 
    name3="Charlie");

    // we can use diffrent formatting options for the placeholders

    println!("{} is {:b} in binary, {:o} in octal and {:x} in hexadecimal",
        42, 42, 42, 42
    );

    // you can right-justify text

    println!("justified text : {:>20}", "PLACEHOLDER");

    //or you can use named arguments:

    println!("justified text : {placeholder:>width$}", 
    placeholder="12345",
    width=20
    );    
}   