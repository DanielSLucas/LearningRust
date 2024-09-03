// cargo -> gerenciador de pacotes
// criar projeto `cargo new <project_name>`
// pacotes de código são conhecidos como crates

// para compilar o projeto usando o cargo
// `cargo build`
// para rodar o projeto usando o cargo
// `./target/debug/hello_cargo`
// para compilar e rodar
// `cargo run`
// para checar que o progama compila, mas se gerar um executável
// `cargo check`
// para compilar o código com optimizações e gerar o executável para release
// `cargo build --release` *executável na pasta target/release


// sem o cargo
// compilar com o comando `rustc main.rs`
// roda executando o arquivo, `./main.rs`

fn main() {
    println!("Hello, world!");
}
