use std::io;

fn convert_to_int(data_input: &String) -> i32 {
  let x: i32 = data_input.trim().parse::<i32>().unwrap();

  return x;
}

fn main() {
  let mut entrada_fatorial: String = String::new();

  io::stdin()
    .read_line(&mut entrada_fatorial)
    .expect("Erro ao ler entrada_fatorial");

  let mut fatorial: i32 = 1;
  let mut entrada_int: i32 = convert_to_int(&entrada_fatorial);

  while entrada_int > 1 {
    fatorial = fatorial * entrada_int;
    entrada_int = entrada_int - 1;
  }

  println!("O fatorial é: {}", fatorial);
}
