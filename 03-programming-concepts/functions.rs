fn main() {
  // chamada da função
  print_labeled_measurement(5, 'h');

  // STATEMENT  -> instruções que performam alguma ação e não retornam um valor
  let x = 6; // uma statement que declara uma variável
  // let x = (let y = 6); // dá erro pois statements n retornam nenhum valor
  
  // EXPRESSION -> resulta em um valor
  let y = { // um novo bloco de escopo é criado com as chaves
    let x = 3; // uma statement
    x + 1 // uma expression que retorna x + 1 (4) *note que expressões não tem ';'
  }; // fim do bloco, o valor 4 é retornado

  let x = five(); // chamar uma função é um expression (retorna um valor)
  let x = plus_one(x);

  println!("The value of x is {x}");
}

// função com parâmetros e sem retorno
fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

// função com retorno do tipo i32
fn five() -> i32 {
  5
}

// com parâmetro e retorno
fn plus_one(x: i32) -> i32 {
  x + 1 // expression no fim da função -> retorno implicito (é possível explicitar com o 'return')
  // se colocar `;` aqui vai dar erro, pq a função n vai retornar nada
}