fn main() {
  // Criando um vetor vazio
  let mut v: Vec<i32> = Vec::new();

  // Adicionando elementos ao vetor
  v.push(1);
  v.push(2);
  v.push(3);

  // Acessando elementos por índice
  let third: &i32 = &v[2];
  println!("O terceiro elemento é {}", third);

  // Acessando elementos com `get` (retorna `Option`)
  match v.get(2) {
      Some(third) => println!("O terceiro elemento é {}", third),
      None => println!("Não há terceiro elemento"),
  }

  // Iterando sobre os elementos do vetor
  for i in &v {
      println!("{}", i);
  }

  // Vetores são redimensionáveis e podem armazenar diferentes tipos de dados.
  // A seguir, criamos um vetor de Strings.
  let mut strings = vec![String::from("hello"), String::from("world")];
  strings.push(String::from("rust"));

  // Imprimindo os vetores
  for s in &strings {
      println!("{}", s);
  }

  {
    let v = vec![1, 2, 3, 4];
    // ...
  } // <- v sai do escopo aqui e ele e seus elementos são liberados da memória


  let vectorWithDifferentTypes = [
    MyVectorTypes::Int(3),
    MyVectorTypes::Text("blue".to_string()),
    MyVectorTypes.Float(10.12)
  ]
}


enum MyVectorTypes {
  Int(i32),
  Float(f64),
  Text(String),
}