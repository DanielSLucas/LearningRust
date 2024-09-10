use std::collections::HashMap;

fn main() {
  // Criando um novo HashMap
  let mut scores = HashMap::new();

  // Inserindo valores no HashMap
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  // Acessando valores com uma chave
  let team_name = String::from("Blue");
  let score = scores.get(&team_name); // retorna um Option
  match score {
    Some(&value) => println!("O time {} tem {} pontos.", team_name, value),
    None => println!("O time {} não foi encontrado.", team_name),
  }
  // OU
  // let score = scores.get(&team_name).copied().unwrap_or(0);
  // .copied() -> faz uma copia do ao invés de pegar o valor emprestado
  // .unwrap_or(0) -> valor padrão caso seja None (tipo .orElse() do java)

  // Iterando sobre o HashMap
  for (key, value) in &scores {
    println!("{key}: {value}");
  }

  // Atualizando valores

  scores.insert(String::from("Blue"), 25); // Atualiza o valor associado à chave "Blue"
  scores.entry(String::from("Green")).or_insert(30); // Adiciona "Green" com valor 30 se não existir

  // Usando `entry` para atualizar valores
  let count = scores.entry(String::from("Blue")).or_insert(0); 
  // or_insert retorna &mut T, usamos o "*" para de-referenciar (n da para fz atribuição numa ref)
  *count += 1; // Incrementa o valor associado à chave "Blue"

  // Exibindo os valores finais
  for (key, value) in &scores {
    println!("{key}: {value}");
  }
}
