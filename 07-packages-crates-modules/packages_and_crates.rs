// Este arquivo exemplifica um projeto de Rust, que inclui vários pacotes e crates.

// Crate: O menor artefato compilável em Rust.
// Existem dois tipos principais de crates: binários (main.rs) e bibliotecas (lib.rs).

// Este exemplo é de uma crate binária.
fn main() {
  println!("Hello, world!");
}

// Um pacote é uma coleção de crates. Um pacote contém um arquivo `Cargo.toml` 
// que descreve como compilar as crates do pacote.

// O arquivo `Cargo.toml` geralmente define dependências, metadados e a estrutura do projeto.

// Por exemplo, em `Cargo.toml` você pode declarar dependências de outras crates:

// [dependencies]
// rand = "0.8"

// Isso significa que seu projeto depende da crate `rand`, versão 0.8.
