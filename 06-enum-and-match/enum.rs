fn main() {
  // Definindo uma enumeração para diferentes tipos de IP
  let home = IpAddrKind::V4;
  let loopback = IpAddrKind::V6;

  // Utilizando uma enum com valores associados
  let home = IpAddr::V4(String::from("127.0.0.1"));
  let loopback = IpAddr::V6(String::from("::1"));

  // Exemplo com a enum Message e seus métodos
  let m = Message::Write(String::from("hello"));
  m.call();
}

// Enum simples para representar tipos de endereços IP
enum IpAddrKind {
  V4,
  V6,
}

// Enum com dados associados para cada variante
enum IpAddr {
  V4(String),
  V6(String),
}

// Enum com variantes diferentes, incluindo uma que armazena dados
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

// Implementando métodos para a enum
impl Message {
  fn call(&self) {
      // Implementação do método
  }
}
