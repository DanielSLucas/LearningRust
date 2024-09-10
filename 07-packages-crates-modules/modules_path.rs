mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }

  mod serving {
      fn take_order() {}
  }
}

pub fn eat_at_restaurant() {
  // Caminho absoluto: começa a partir da raiz do crate
  crate::front_of_house::hosting::add_to_waitlist();

  // Caminho relativo: começa a partir do módulo atual
  front_of_house::hosting::add_to_waitlist();
}
