fn main() {
  // Slices -> permite você referênciar um sequencia contigua de elementos em uma collection
  // é um tipo de referencia, então "n tem dono"

  // exemplo da utilidade de slices

  let mut s = String::from("hello world");
  let word = first_word_v1(&s); // valor 5
  s.clear(); // esvazia a string, deixando-a igual a ""
  // se tentássemos usar o 'word' para acessar 's' a partir daqui, teriamos problema
  // visto que 's' é uma string vazia diferente do que era quando a fn foi executada
  // uma forma de evitar isso são String slices (&str)
  // exemplo de string slice:
  let s = String::from("hello world");
  let hello = &s[0..5]; // string slice só com a palavra hello
  // ao invés de ser só uma referência a uma string inteira, 
  // é uma referência a uma porção da string, especificado pelo [0..5]

  
  // Evitando o problema com string slice
  let mut s = String::from("hello world");
  let word = first_word_v2(&s); // valor 5
  s.clear(); // dá erro na compilação! pois 's' foi emprestado para fn e não pode ser alterado!

  // Agora podemos entender como string literals funcionam
  let s = "Hello, world!"; // é um string slice (&str) que aponta pro endereço em que está
  // por isso string leterals são imutáveis!


  // dá para usar slices em outros tipos também
  let a = [1, 2, 3, 4, 5];

  let slice = &a[1..3]; // tipo: &[i32], funciona do msm jeito que string slices
}

// uma função que retorna o index final da primeira palavra de uma string
fn first_word_v1(s: &String) -> usize {
  let bytes = s.as_bytes(); // converte a string em um array de bytes
  
  // itera sobre o array de bytes enumerado
  // (enumerado signigica que cada elemento virou uma tupla com o index e o valor)
  for (i, &item) in bytes.iter().enumerate() {
    // b' ' -> byte literal se referindo ao caracter de espaço
    if item == b' ' {
      return i; // retorna o index
    }
  }

  s.len() // retorna o tamanho da string
}

// retorna uma string slice com a porção da string referente a primeira palavra
fn first_word_v2(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i]; // retona de 0 até o index
    }
  }

  &s[..] // retorna a string slice inteira
}

// se você quiser pode usar &str como parâmetro que funcionará tanto para `String` como para `&str`
fn first_word_v3(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}
