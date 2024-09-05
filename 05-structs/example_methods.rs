#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// implementation block para o `Rectangle`
impl Rectangle {
  // Método, semelhante a função, com a diferença de estar no contexto de um struct, enum ou trait obj
  // primeiro parametro é smp `self`, nesse caso `&self` já que estamos emprestando imutavelmente
  fn area(&self) -> u32 {
    self.width * self.height
  }

  // da para ter métodos e propriedades com o msm nome
  fn width(&self) -> bool {
    self.width > 0
  }

  // checa se a instancia de rectangle consegue conter outro
  // recebe outro retangulo como parâmetro
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  // Associated Function -> associado com o struct, mas n depende da instancia (self)
  // mts vezes utilizado como contrutor, e acessado com `::` (e.g.: `Rectangle::square(10)`)
  fn square(size: u32) -> Self {
    Self { // 'Self' é um apelido para o nome que aparece dps de 'impl' (Rectangle)
      width: size,
      height: size,
    }
  }

  // Daria para ter mais de um bloco de impl para o Rectangle, mas nesse caso n faria mt sentido
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    // chamando o método
    rect1.area()
  );

  // se o rect1 tiver uma largura maior que zero, printa ela
  if rect1.width() {
    println!("The rectangle has a nonzero width; it is {}", rect1.width);
  }

  // dá no mesmo! (n precisa usar um `->` quando se tratar de refs como acontece no C)
  rect1.area();
  (&rect1).area();


  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };

  // chamando um método com parâmetro
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}