// Exemplo de definição de módulos em Rust
mod front_of_house {
  pub mod hosting { // publico
    pub fn add_to_waitlist() {
      println!("Adicionado à lista de espera!");
    }
  }

  mod serving { // privado por padrão
    fn take_order() {
      println!("Pedido realizado!");
    }
  }
}

// Função principal que chama uma função dentro de um módulo
fn main() {
  // Como a função `add_to_waitlist` e o módulo `hosting` são públicos, podemos acessá-los
  front_of_house::hosting::add_to_waitlist();
  
  // `take_order` é privado, então isso não funcionaria:
  // front_of_house::serving::take_order();
}
