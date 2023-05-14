// fn main() {
    //Criando um variavel mutavel
//     let mut x = 5;

//     println!("O valor de x é {x}");

//     x = 6;
    
//     println!("O valor de X é {x}");


    // criando uma variavel e sombreando a mesma

//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
 
    // criando sombreamento para utilizar tipos diferentes na variavel com mesmo nome
    // let spaces = "   ";
    // let spaces = spaces.len();

    // println!("The value of spaces is: {spaces}");


    //operações
    
    // addition
    // let sum = 5 + 10;
    
    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1

    // // remainder
    // let remainder = 43 % 5;

    // println!("The value of spaces is: {truncated}");

    //booleanos

    // let t = true;

    // let f: bool = false; // with explicit type annotation

    //char
    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

    //tupla
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    //destructuring
    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of y is: {y}");

    // acessando itens da tupla
    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

    // array
    // let a = [1, 2, 3, 4, 5];
    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //           "August", "September", "October", "November", "December"];

    //definindo tipo e tamanho
    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    //inicializando com valor predefinido
    // let a = [3; 5];


    //acessando elementos do array

    // let a = [1, 2, 3, 4, 5];

    // let first = a[0];
    // let second = a[1];

// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}