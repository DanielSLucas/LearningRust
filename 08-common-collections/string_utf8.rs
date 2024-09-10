fn main() {
  // Criando uma String usando `String::new()`
  // * internamente uma String é um wrapper sobre um Vec<u8> (vetor de inteiros)
  let mut s = String::new();

  // Adicionando texto à String
  s.push_str("Hello, "); // add um string slice
  s.push('w'); // add um caracter

  // Concatenando Strings
  let s1 = String::from("Hello,");
  let s2 = "world!".to_string();
  let s3 = s1 + &s2; // Note que s1 é movido e não pode ser usado depois

  // Usando a macro `format!` para concatenar Strings
  let s4 = format!("{s1} {s2}");

  // Acessando caracteres de uma String
  let first_char = s4.chars().next().unwrap(); // Obtendo o primeiro caractere
  println!("O primeiro caractere é: {}", first_char);
  // dá para iterar pelos caracteres de uma string usando:
  // s.chars() -> retorna cada char
  // s.bytes() -> retonar cada byte (num do unicode de cada char, que pode ser feito por mais de 1 char)

  // Fatiamento de Strings
  // let h = s1[0]; -> n funcionaria, tem que usar index ou ent .char()
  let hello = &s4[0..5]; // Fatiamento da string de 0 a 5 (não inclusivo)
  println!("Fatiamento: {}", hello);

  // Strings em Rust são UTF-8, portanto, operações com caracteres Unicode funcionam
}
