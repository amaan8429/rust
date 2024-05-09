fn main() {
    //main is the entry point of the program. you don't have to call main unlike in C or C++ or Java
    let sentence : String = String::from("Hello, World!"); // String::from() is used to create a new String
    let first_word : String = get_first_word(sentence);
    println!("{}", first_word);
    looping_numbers();
    moved_values();
    my_new_function();

    
}

// Function to get the first word of a sentence
fn get_first_word(sentence: String) -> String { 
    //every variable in Rust is immutable by default. To make it mutable, you have to use the mut keyword
    let mut ans = String::new(); // String::new() is used to create a new empty String
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

// Function to loop through numbers
fn looping_numbers() {

    let _i = 4;  // putting _ in front of a variable name will suppress the warning of unused variable
    for i in 0..10 {
        println!("{}", i);
    }
}



//concept of ownership in Rust
//ownership is a concept in Rust that helps to manage memory efficiently
//anything on the heap is owned by a variable and when the variable goes out of scope, the memory is deallocated
//ownership rules:
// 1. Each value in Rust has a variable that is called its owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped
// 4. When a value is assigned to another variable, the ownership is moved to the new variable
// 5. If you want to create a copy of a value, you have to use the clone() method
// 6. Rust will never automatically create deep copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance

fn moved_values() {
    let s1 = String::from("hello");
    print!("{}", s1);
    let s2 = s1;
    print!("{}", s2); // this will print hello
    // println!("{}", s1); // this will give an error because s1 has been moved to s2
}



//transferring ownership
fn my_new_function(){
    let my_string : String = String::from("Hello, World!");
    tranfer_ownership(my_string);
    // println!("{}", my_string); // this will give an error because my_string has been moved to the function
    //once the variable is passed to the function, the ownership is transferred to the function
    //reason is my_string and some_string can not point to the same string in memory

    //you can clone the value and pass it to the function but cloning is expensive and it creates a deep copy and should be avoided
}

fn tranfer_ownership(some_string: String){
    print!("{}", some_string)
}


//you can return ownership from a function by returning the value

