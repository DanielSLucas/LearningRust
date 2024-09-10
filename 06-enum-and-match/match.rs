fn main() {
  // Usando o 'match' com enums
  let coin = Coin::Penny;
  let value_in_cents = value_in_cents(coin);
  println!("O valor da moeda é {value_in_cents} centavos");

  // Usando o 'match' com o enum Option<T>
  // Option <T> pode ser Some(T) ou None
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  // Exemplo de match exaustivo
  let dice_roll = 9;
  match dice_roll {
      3 => println!("Você acertou um 3!"),
      7 => println!("Número da sorte 7!"),
      other => println!("Você rolou um {other}"),
  }
}

// Enum representando diferentes moedas
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

// Função que usa 'match' para converter enum Coin em valores
fn value_in_cents(coin: Coin) -> u32 {
  match coin {
      Coin::Penny => {
          println!("Penny é sortudo!");
          1
      },
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
  }
}

// Função que usa 'match' com Option<i32>
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
      Some(i) => Some(i + 1),
      None => None,
  }
}
