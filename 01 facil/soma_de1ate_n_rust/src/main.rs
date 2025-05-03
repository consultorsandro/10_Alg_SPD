use std::io;

fn main() {
    println!("Digite um número inteiro:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

    let n: i32 = input.trim().parse().expect("Por favor, insira um número válido");
    let mut soma = 0;

    for i in 1..=n { // O operador ..= é usado para incluir o último número no intervalo
        // O operador .. é usado para criar um intervalo exclusivo do último número
        soma += i;
    }

    println!("A soma de todos os números de 1 até {} é: {}", n, soma);
}
