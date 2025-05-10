// filepath: /algoritmos-projeto/algoritmos-projeto/medio/algoritmo4.rs
// Este programa calcula a soma dos números pares em um intervalo fornecido pelo usuário.
// Exemplo: Se o usuário fornecer 1 e 10, a soma dos números pares (2, 4, 6, 8, 10) será 30.

use std::io;

fn main() {
    let mut entrada = String::new();

    // Solicita ao usuário o início do intervalo
    println!("Digite o início do intervalo: ");
    io::stdin().read_line(&mut entrada).expect("Erro ao ler a entrada.");
    let inicio: i32 = match entrada.trim().parse() { // Tenta converter a entrada para um número inteiro
        // Se a conversão falhar, exibe uma mensagem de erro e encerra o programa
        // Se a conversão for bem-sucedida, armazena o número na variável `inicio`
        // `trim()` remove espaços em branco no início e no fim da string
        Ok(num) => num, //
        Err(_) => {
            println!("Por favor, insira um número válido.");
            return;
        }
    };

    entrada.clear(); // Limpa a string de entrada para reutilizá-la

    // Solicita ao usuário o fim do intervalo
    println!("Digite o fim do intervalo: ");
    io::stdin().read_line(&mut entrada).expect("Erro ao ler a entrada.");
    let fim: i32 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, insira um número válido.");
            return;
        }
    };

    // Verifica se o intervalo é válido
    if inicio > fim {
        println!("O início do intervalo deve ser menor ou igual ao fim.");
        return;
    }

    // Calcula a soma dos números pares no intervalo
    let soma: i32 = (inicio..=fim)
        .filter(|&x| x % 2 == 0) // Filtra apenas os números pares
        .sum(); // Soma os números pares

    // Exibe o resultado
    println!(
        "A soma dos números pares entre {} e {} é: {}",
        inicio, fim, soma
    );
}

