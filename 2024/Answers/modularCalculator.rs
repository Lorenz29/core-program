#![allow(non_snake_case)]

fn modularCalculator(op: char, num1: i32 , num2: i32, mod1: i32) -> i32 {
    if op == '+' { 
        ((num1 + num2) % mod1 + mod1) % mod1
    }
    else {
        if op == '-' {
            ((num1 - num2) % mod1 + mod1) % mod1
        }
        else {
            if op == '*' {
                ((num1 * num2) % mod1 + mod1) % mod1
            }
            else {
                0
            }
        }
    //    println!("Operación no permitida");
    }
    

}

fn main(){
    println!("{}",modularCalculator('+', 10, 15, 12)); // Should return: 1
    println!("{}",modularCalculator('-', 10, 15, 12)); // Should return: 7
    println!("{}",modularCalculator('*', 10, 15, 12)); // Should return: 6
    
    //println!("{}",modularCalculator('*', 199, 153, 12)); // Should return: 1
}
