fn main() {
    //main is the entry point of the program. you don't have to call main unlike in C or C++ or Java
    let sentence : String = String::from("Hello, World!"); // String::from() is used to create a new String
    let first_word : String = get_first_word(sentence);
    println!("{}", first_word);
    looping_numbers();
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
