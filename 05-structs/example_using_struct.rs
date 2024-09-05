
// struct que representa um retangulo
// structs n tem uma implementação de `Display` (trait que permite ser printado) por padrão
// ent se eu quiser printar um struct ele deve implementar a trait `Debug` dessa forma:
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let scale = 2;
  let rect1 = Rectangle {
    // se passarmos um expressão `dbg!` printa e devolve o direito de dono com o valor retornado
    width: dbg!(30 * scale),
    height: 50,
  };

  // printando o retangulo usando o especificador de `Debug`, `:?` (`:#?`, se quiser identação)
  println!("rect1 is {rect1:?}");
  
  // outra forma de printar é usando a macro `dbg!` (usa stderr e especifica linha e coluna)
  // ela vira dona do que for passado como parâmetro, por isso passamos como rect como referência
  dbg!(&rect1);

  println!(
    "The area of the rectangle is {} square pixels.",
    area(&rect1)
  );  
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}