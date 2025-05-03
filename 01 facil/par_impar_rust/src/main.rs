use std::io;

fn main() {
    println!("Digite um número:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

    let numero: i32 = input.trim().parse().expect("Por favor, insira um número válido");

    if numero % 2 == 0 {
        println!("O número {} é par.", numero);
    } else {
        println!("O número {} é ímpar.", numero);
    }
}