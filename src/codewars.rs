/*
Write a function that takes a string of parentheses, and determines if the order of the parentheses is valid. The function should return true if the string is valid, and false if it's invalid.

"()"              =>  true
")(()))"          =>  false
"("               =>  false
"(())((()())())"  =>  true

0 <= input.length <= 100

Along with opening (() and closing ()) parenthesis, input may contain any valid ASCII characters. Furthermore, the input string may be empty and/or not contain any parentheses at all. Do not treat other forms of brackets as parentheses (e.g. [], {}, <>).
*/

fn valid_parentheses(s: &str) -> bool {
    let mut count:i32 = 0;

    for c in s.chars() {
        if c == '(' { count += 1 }
        if c == ')' { count -= 1 }
        if count < 0 { return false }
    }

    count == 0
}


fn main() {
    println!("Hola {}", valid_parentheses("()(()"));
}