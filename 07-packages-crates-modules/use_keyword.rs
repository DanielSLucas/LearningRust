mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  // Usando o atalho criado com 'use' para chamar a função
  hosting::add_to_waitlist();
}

// Alias para renomear ao importar
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn function1() -> FmtResult {
  // Implementação
  Ok(())
}

fn function2() -> IoResult<()> {
  // Implementação
  Ok(())
}

// Trazendo o nome de uma enum para o escopo
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}