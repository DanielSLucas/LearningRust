fn main() {
  // código abaixo dá erro pois variáveis são imutáveis por padrão
  // let x = 5;
  // println!("The value of x is: {x}");
  // x = 6;
  // println!("The value of x is: {x}");

  // versão mutável
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");

  // outra forma de declarar variáveis é com const
  // mas esse tipo de variável é sempre imutável, não é possível usar 'mut' com ela
  // consts podem ser declaradas em qualquer escopo
  // na sua declaração é necessário explicitar o tipo da variável
  // e o valor deve ser uma expressão constante, 
  // nada que só possa ser computado em tempo de execução
  // *funciona de maneira semelhante ao #define do C, o compilador vai substituir pelo valor 
  // e está disponível o tempo inteiro enquanto o programa roda, no escopo em que foi definido
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


  // SHADOWING - pode-se declarar uma variável com o mesmo nome que uma anterior
  // exemplo
  let x = 5;
  let x = x + 1;

  {
    let x = x * 2;
    // x vai ser 12 aqui
    println!("The value of x in the inner scope is: {x}");
  }

  // x vai ser 6 aqui
  println!("The value of x is: {x}");

  // dessa maneira estamos efetivamente criando uma nova variável,
  // por isso não há problema e mudar seu tipo
  // exemplo
  let spaces = "   ";
  let spaces = spaces.len();

  // o que já não funcionaria caso fosse uma variável mutavel
  // exemplo
  // let mut spaces = "   ";
  // spaces = spaces.len();

}