// panic! deve ser usado para erros irrecuperáveis, 
// situações onde o programa não pode continuar de forma segura e deve parar imediatamente.

fn main() {
  // panic! -> macro que sinaliza um erro irrecuperável
  panic!("Esse é um erro irrecuperável!"); // Causa um pânico e encerra o programa
  
  // Exemplo de pânico com assertiva
  let x = 5;
  assert!(x == 5); // Não causa pânico, pois a condição é verdadeira
  assert!(x != 5); // Causa pânico, pois a condição é falsa

  // Exemplo de pânico com erro personalizado
  let s = String::from("hello");
  let index = 10;
  let _char = s.chars().nth(index).expect("Índice fora dos limites!");
}
