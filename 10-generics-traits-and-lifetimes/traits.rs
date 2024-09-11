// trait (caracteristica) -> algo parecido com uma interface em outras linguagens
// Definindo um trait chamado Summary
pub trait Summary {
  fn summarize(&self) -> String; // pode ser só a assinatura
  // OU então uma implementação padrão:
  // fn summarize(&self) -> String {
  //   String::from("(Read more...)")
  // }
}


pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

// Implementando o trait Summary para o struct NewsArticle
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

// Implementando o trait Summary para o struct Tweet
impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

// Usando traits como parâmetros de função
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
// &impl Summary é uma abreviação de pub 'fn notify<T: Summary>(item: &T)' (chamado trait bound)
// se precisasse implementar mais de uma trait dá para usar o '+'
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32;
// OU então uma where clause
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {}
// *como retornar uma trait `fn func() -> impl Summary;`


fn main() {
  let article = NewsArticle {
    headline: String::from("Rust 1.60 Released"),
    location: String::from("The Internet"),
    author: String::from("Jane Doe"),
    content: String::from("The Rust team has announced a new release."),
  };

  let tweet = Tweet {
    username: String::from("user123"),
    content: String::from("Rust is amazing!"),
    reply: false,
    retweet: false,
  };

  println!("{}", article.summarize());
  println!("{}", tweet.summarize());
  notify(&article);
}
