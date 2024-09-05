// STRUCT -> tipo de dado customizável, que te permite agrupar e nomear valores relacionados
// valores nele podem ser de vários tipos
// valores são nomeados, podemos utilizar esse nomes para acessar os dados

// declaração do struct
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
 
// Tuple Struct -> uma "tupla nomeada"
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like Struct -> struct que não tem campos/valores
struct AlwaysEqual;

fn main() {
  // uso do struct (mutavel nesse caso)
  let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
  };

  // alterando o valor do email
  user1.email = String::from("anotheremail@example.com");

  let user2 = User {
    email: String::from("another@example.com"),
    ..user1 // os demais campos, fora o email, serao o msm de user1 (algo parecido com `...` do js)
    // como o username é uma string, ela foi movida para user2 e já n é valida em user1
  };

  // Ambos são tuplas, mas de tipos diferentes já que tem nome
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  // um tipo sem nenhum dado
  let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username, // o mesmo que escrever username: username
    email, // como tem o msm nome, n precisa escrever duas vezes
    sign_in_count: 1,
  }
}