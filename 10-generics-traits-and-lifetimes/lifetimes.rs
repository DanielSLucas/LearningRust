// lifetimes -> permitem que o Rust saiba quanto tempo as referências são válidas
// é um tipo de generic `<'a>`

// syntax
// &i32        // uma referencia
// &'a i32     // uma referencia com lifetime explicito
// &'a mut i32 // uma referencia mutavel com lifetime explicito

// Função que rebece parâmetros com o msm lifetime do retorno
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
// `'a` é o tempo de vida das referências, ou seja, ambas as referências 'x' e 'y' 
// devem ter o mesmo lifetime, e a referência retornada também terá esse lifetime.


// lifetime statico -> vive enquanto o programa durar
let s: &'static str = "I have a static lifetime.";

// Struct que contém uma referência, requer um lifetime
struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn main() {
  let string1 = String::from("long string is long");
  let string2 = "xyz";

  // Chamando a função com referências de mesmo lifetime
  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);

  // Exemplo com struct que tem lifetime
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt { part: first_sentence };

  println!("Important excerpt: {}", i.part);
}

// Lifetime Elision Rules (eliminação automática de lifetimes):
// O Rust tenta inferir os lifetimes em funções com base em regras predefinidas
// - 1ª: Cada parâmetro de referência tem seu próprio lifetime
// - 2ª: Se há apenas um parâmetro de referência, o lifetime de retorno é o mesmo do parâmetro
// - 3ª: Se há mais de um parâmetro, mas um deles é `&self` ou `&mut self`, 
//       o lifetime do retorno é o de `self`
// Exemplo onde o compilador inferiria automaticamente os lifetimes
fn first_word(s: &str) -> &str {
  &s[..]
}
// Nesse caso, o compilador aplica a segunda regra de eliminação.

// Lifetimes em Structs
// Para structs que possuem referências, é necessário definir um lifetime 
// para que a struct e suas referências 
// tenham uma relação clara quanto à duração de vida das referências.

