fn main() {
  let number = 6;

  // uso de `if`, `else` e `else if`
  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3"); // vai cair aqui e nem vai checar as condições abaixo
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }


  // como o `if` é uma expression, é possível usalo dentro de um let por exemplo
  // *O tipo do valor retornado no `if` e no `else` devem ser o mesmo! Se não, Erro!
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {number}"); // vai ser 5

  // LOOPS

  loop { // igual um while(true), para só com uma interrupção ou um break
    println!("again!");
    break; // para o loop, e n executa código abaixo
    // continue; // continua o loop, e n executa código abaixo
  }


  // Retornando valor de um loop
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2; // retorna um valor
    }
  };

  println!("The result is {result}"); // result é 20


  // loop labels -> da para dar um nome a um loop, para assim conseguir para-lo dentro de outro
  let mut count = 0;

  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break; // para o loop atual
      }
      if count == 2 {
        break 'counting_up; // para o loop de fora que tem o nome 'counting_up
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");

  // WHILE LOOPs

  let mut number = 3;

  // roda enquanto number for diferente de 0
  while number != 0 {
    println!("{number}!");

    number -= 1;
  }

  println!("LIFTOFF!!!");

  // for loop
  let a = [10, 20, 30, 40, 50];

  for element in a {
    println!("the value is: {element}");
  }

  // for loop usando Range
  for number in (1..4).rev() {
    println!("{number}!");
  }
  println!("LIFTOFF!!!");
}