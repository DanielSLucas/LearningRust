fn main() {
  let config_max = Some(3u8);

  // Usando if let para simplificar o padrão match
  if let Some(max) = config_max {
      println!("O valor máximo configurado é {max}");
  }

  // Exemplo com else
  let coin = Coin::Quarter(UsState::Alaska);
  let mut count = 0;
  if let Coin::Quarter(state) = coin {
      println!("Quarter do estado de {:?}!", state);
  } else {
      count += 1;
  }
}

// Enum Coin com um dado associado à variante Quarter
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

// Enum para representar estados dos EUA
#[derive(Debug)] // permite imprimir a enum UsState
enum UsState {
  Alabama,
  Alaska,
  // outros estados
}
