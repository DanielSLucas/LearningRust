use std::fs::File;
use std::io::{self, Read};

// Result<T, E> é um enum que pode ser:
// Ok(T) -> Indica sucesso e contém um valor do tipo T.
// Err(E): Indica falha e contém um valor do tipo E, que descreve o erro.

// Result é mais adequado para erros recuperáveis, 
// onde o programa pode continuar a execução após tratar o erro de forma adequada.

fn main() -> Result<(), io::Error> {
  // Retorna Result
  let greeting_file_result = File::open("hello.txt");

  // Lida com o Result, retorna o arquivo ou entra em panico se der erro
  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    // Dá para tratar mais de um tipo de erro
    Err(error) => match error.kind() {
      // tenta criar arquivo se ele n existir
      ErrorKind::NotFound => match File::create("hello.txt") {
          Ok(fc) => fc,
          Err(e) => panic!("Problem creating the file: {e:?}"),
      },
      // entra em panico
      other_error => {
        panic!("Problem opening the file: {other_error:?}");
      }
    },
  };

  // Daria para reescrever esse código assim, semelhante ao que dá pra fazer 
  // com o optional do java
  let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
          panic!("Problem creating the file: {error:?}");
      })
    } else {
      panic!("Problem opening the file: {error:?}");
    }
  });

  // Desse jeito, se o arquivo n existir o unwrap() vai chamar o panic!
  let greeting_file = File::open("hello.txt").unwrap();
  // OU então daria para definir uma msg de erro para caso o arquivo n exista
  // (vai entrar me panico igual o anterior, mas com uma msg melhor)
  let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");

  
  let greeting_file_result = File::open("hello.txt");
  // também é possível só repassar o erro (propaga-lo)
  let mut greeting_file_result = match greeting_file_result {
    Ok(file) => file,
    Err(e) => Err(e),
  };

  // Equivalente ao código acima seria usar o operador `?`, que também só vai repassar o Ok ou Err
  // o `?` só funciona para funções que retornam 'Result' ou 'Option' ou algo que implemente 'FromResidual'
  let greeting_file_result = File::open("hello.txt")?;

  let mut s = String::new();
  f.take(10).read_to_string(&mut s)?;

  println!("Conteúdo do arquivo: {}", s);
  
  Ok(())
}

// Função que retorna um Result para lidar com possíveis erros
fn read_username_from_file() -> Result<String, io::Error> {
  // File::open tbm retorna Result, mas com o operador `?` repassa o erro para cima
  let f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}
