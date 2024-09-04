fn main() {
  let s1 = String::from("hello");
  
  // ao invés de passar o "direito de dono" para função emprestamos o valor (uma refência do valor)
  // algo parecido com um ponteiro
  let len = calculate_length(&s1); // o valor de s1 ainda é válido, pois foi só "emprestado"

  println!("The length of '{s1}' is {len}.");

  // referências, tal como variáveis, são imutáveis! A não ser que usemos a keyword 'mut'
  let mut s = String::from("hello");
  change(&mut s); // `&mut` refência mutável

  // se você tem uma referência mutável para um valor, 
  // você não pode ter outras referências para ele
  let r1 = &mut s;
  let r2 = &mut s; // <- Erro de compilação! Segunda ocorrencia de uma mesma '&mut'
  println!("{}, {}", r1, r2);

  // se for em escopos diferentes tudo bem ter dois '&mut'
  {
    let r1 = &mut s;
  } // aqui sai do escopo de r1, ent n tem problema criar outro `&mut`
  let r2 = &mut s;

  // n dá pra tem um '&mut' se já tem um '&'
  let r1 = &s; // sem problema, valor emprestado imutável
  let r2 = &s; // sem problema
  let r3 = &mut s; // PROBLEMA, valor emprestado mutável que já foi emprestado antes!

  // se o escopo de uso das referências '&' acabar antes do '&mut', é possível utiliza-lo
  let r1 = &s; // sem problema, só leitura
  let r2 = &s; // sem problema, só leitura
  println!("{r1} and {r2}");
  // r1 e r2 não são mais usados

  let r3 = &mut s; // sem problema
  println!("{r3}");

  // não é possível emprestar um valor que deixou de existir
  let reference_to_nothing = {
    let s = String::from("hello");
    &s 
  };// `s` deixa de existir com o fim desse escopo
  // você conseguiria transferir o "direito de dono" retornando apenas 's'
  // mas não emprestar retornando '&s'
}

// o simbolo `&` indica uma referência, 
fn calculate_length(s: &String) -> usize {
  s.len()
}// como s é apenas uma referência, que a função não é dona, a memória não é liberada

fn change(some_string: &mut String) {
  some_string.push_str(", world"); // se some_string não fosse '&mut' daria erro na compilacao
}