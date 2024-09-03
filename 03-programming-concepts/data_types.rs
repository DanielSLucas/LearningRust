fn main() {
  // SCALAR TYPES -> representa um único valor
  // Existem 4 scalar types: integers, floating-point, numbers, Booleans e characters

  // INTERGERS -> número inteiro sem casas decimais

  // podem ter sinal (signed) ou não (unsigned)
  // indicado por `i` ou `u` respectivamente
  // e podem ocupar uma determinada quantidade de bits
  // exemplos

  // tamanho  com sinal   sem sinal
  // 8-bit    i8          u8
  // 16-bit   i16         u16    
  // 32-bit   i32         u32
  // 64-bit   i64         u64
  // 128-bit  i128        u128
  // arch     isize       usize (depende da arquiteture 64 ou 32 bits)

  // também é possível escrever o inteiro de maneira literal
  // Number literals	Example
  // Decimal	        98_222
  // Hex	            0xff
  // Octal	          0o77
  // Binary	          0b1111_0000
  // Byte (u8 only)	  b'A'

  // FLOATING-POINT -> Números com casas decimais significativas
  // pode ser f32 ou f64, sendo 64 bits o padrão dado a arquitetura das cpus modernas
  // todos os números float tem sinal

  let x = 2.0; ; // f64

  let y: f32 = 3.0; // f32

  // NUMERIC OPERATIONS

  let sum = 5 + 10; // adição
  let difference = 95.5 - 4.3; // subtração
  let product = 4 * 30; // multiplicação
  let quotient = 56.7 / 32.2; // divisão
  let truncated = -5 / 3; // Resulta em -1, pois divisão de inteiro é arredondada
  let remainder = 43 % 5; // resto

  // BOOLEAN -> verdadeiro ou falso

  let t = true;
  let f: bool = false;

  // CHAR -> caracter, usa aspas simples
  let c = 'z';
  let z: char = 'ℤ'; // with explicit type annotation
  let heart_eyed_cat = '😻';

  // COUPOND TYPES -> agrupam varios tipos em um

  // TUPLE
  // tuplas tem tamanho fixo, n podem mudar
  // pode ter vários tipos dentro de si
  // criam-se tuplas usando parenteses
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  // é possível desestruturar os valores de uma tupla em outras variaveis
  let (x, y, z) = tup;
  // também é possível fazer isso acessando pelo index
  let x = tup.0;
  let y = tup.1;
  let z = tup.2;
  // tuplas sem qualquer valor tem um nome especial, `unit`
  let tup: () = ();
  // expressões implicitamente retornam o valor 'unit' se não retonarem nenhum outro

  // ARRAY
  // coleção de vários valores do mesmo tipo
  // tem tamanho fixo, diferente de um `vector` por exemplo
  // preferível quando se sabe o tamanho que a coleção de itens deve ter
  let a = [1, 2, 3, 4, 5];
  // declarando o tipo do array explicitamente
  let a [i32, 5] = [1, 2, 3, 4, 5]
  // inicializando o array com um valor
  let a = [3; 5]; // contém 5 elementos, todos incialmente com o valor 3
  // acessando elementos pelo index
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  // se um index maior que o tamanho do array tentar ser acessado
  // um erro será lançado
}