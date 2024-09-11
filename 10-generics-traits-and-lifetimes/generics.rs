use std::cmp::PartialOrd;

// Função genérica que aceita qualquer tipo de argumento que implemente o trait PartialOrd
// PartialOrd -> trait (caracteristica) que indica que um tipo é ordenavel
fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

// Struct genérica com dois campos de tipos diferentes
struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn x(&self) -> &T {
    &self.x
  }
}

fn main() {
  // Usando uma função genérica
  // *isso também funcionaria com um array de char pois ele tbm implementa PartialOrd
  let numbers = vec![34, 50, 25, 100, 65];
  let result = largest(&numbers);
  println!("O maior número é {}", result);

  // Usando uma struct genérica
  let p = Point { x: 5, y: 10.4 };
  println!("p.x = {}", p.x());
}
