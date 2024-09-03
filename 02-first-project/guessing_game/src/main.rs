use std::{
    cmp::Ordering, 
    io // importa `io` (input/output) da lib padrão `std`
};
use rand::Rng; 

fn main() {
    println!("Guess the number!");

    // da para acessar as docs das crates usando `cargo doc --open` 
    // dae abre o /targed/doc/project_name/index.html
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // variáveis são imutáveis por padrão, usa-se o 'mut' para indicar que ela pode mudar
        let mut guess = String::new();

        // "&" permite acessar a referência, que tmb são imutaveis, por isso o 'mut'
        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read line");
        // read_line retorna `Result`` que pode se `Ok` ou `Err`
        // o `expect` lida com o caso de erro e para o código mostrando a msg definida
        // ele não é obrigatório, mas vai ter warning na compilação se n coloca o `expect`
        
        // parses the string guess to a number (u32 - unsigned 32-biy number)
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // as `{}` servem para indicar onde o valor da variável vai ser printado
        // daria para colocar o nome da variavel dentro das chaves, `{guess}`
        // mas se, ao invés de uma variável se tratasse de uma expressão, tipo `y + 2`
        // então essa expressão não poderia ir dentro das chaves
        println!("You guessed: {}", guess);

        // parece um switch, o resultado que der match ele executa o código
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
