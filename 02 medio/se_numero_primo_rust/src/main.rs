// filepath: /algoritmos-projeto/algoritmos-projeto/medio/algoritmo5.rs
// Este arquivo contém a solução para o segundo algoritmo de média dificuldade.
// O código é comentado e apresenta um passo a passo da solução, com exemplos simples.

fn is_prime(num: i32) -> bool { // Função que verifica se um número é primo
    // A função recebe um número inteiro e retorna um booleano
    if num <= 1 {
        return false; // Números menores ou iguais a 1 não são primos
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false; // Se o número for divisível por i, não é primo
        }
    }
    true // Se não for divisível por nenhum número, é primo
}

fn main() {
    use std::io;

    let mut entrada = String::new();

    // Solicita ao usuário um número
    println!("Digite um número para verificar se é primo:");
    io::stdin().read_line(&mut entrada).expect("Erro ao ler a entrada.");
    let number: i32 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, insira um número válido.");
            return;
        }
    };

    // Verifica se o número é primo e exibe o resultado
    if is_prime(number) {
        println!("{} é um número primo.", number);
    } else {
        println!("{} não é um número primo.", number);
    }
}

