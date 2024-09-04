fn main() {
  // OWNERSHIP -> conjunto de regras que controlam como um programa Rust gerencia memória
  // - Cada valor no Rust tem um owner (dono)
  // - Só se pode ter um dono por vez
  // - Quando o dono sair do escopo, o valor será liberado (dropped)
  // *Se alguma dessas regras for violada o programa n compila

  { // um novo bloco de escopo
    let s = "hello" // 's' é valido a partir daqui
  } // fim do escopo - 's' não é mais válido e é liberado da memória

  // Memória do programa:
  // STACK -> pilha que armazena valores de tamanho fixo
  // HEAP -> dados de tamanha variável (alocação mais lenta)
  
  let s = "hello"; // string literal -> tamanho fixo, imutável, armazenado na stack
  let s = String::from("hello"); // String -> lida com dados armazenado no heap

  // movimento e cópia

  let x = 5; // valor inteiro, tamanho fixo e previamente conhecido armazenado na stack
  let y = x; // copia do valor inteiro
  println!("x: {x}, y: {y}")// tanto x quanto y são acessíveis aqui pois uma cópia é criada
  // uma cópia é criada somente para tipos que implementa a caracteristica `Copy`
  // integer, boolean, float, char, arrays, tuplas que contem esses tipos

  let s1 = String::from("hello"); // string de tamanha variável armazenada no heap 
  let s2 = s1; // s1 deixa de ser dono do endereço no heap em que essa string está armazenada
  println!("{s1}, world!"); // erro, s2 é o novo dono do endereço, ent s1 é inválido

  // para uma variável ser de fato copiada para outra ao invés de passar o endereço
  let s1 = String::from("hello");
  let s2 = s1.clone(); // clona os dados na heap, cada var com seu endereço/dados
  println!("s1 = {s1}, s2 = {s2}"); // sem erros

  // o conceito de ownership é valido tbm para funcoes
  let s = String::from("hello");
  takes_ownership(s); // depois daqui n dá mais para usar 's', memoria liberada
  let x = 5;
  makes_copy(x); // depois daqui x ainda é um valor valido, uma copia foi usada na funcao

  // para que a string possa continuar sendo utilizada 
  // a funcao pode devolver o direito de dono para esse escopo
  calculate_length(s); // 's' ainda é valida pois é devolvida

}

fn takes_ownership(some_string: String) { // a função passa a ser 'dono' da some_string
  println!("{some_string}");
} // aqui a memória usada na some_string é liberada

fn makes_copy(some_integer: i32) { // uma cópia do inteiro é recebida pela função
  println!("{some_integer}");
} // a memoria da cópia é liberada, mas a nada acontece com a variável original

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String

  (s, length) // devolve uma tupla com a string e seu tamanho
}