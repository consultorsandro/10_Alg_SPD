use std::io;

fn main() {
    let mut input = String::new();
    
    // Pede o primeiro número
    println!("Digite o primeiro número:");
    io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let num1 = input.trim().parse::<i32>().expect("Por favor, digite um número válido");
    input.clear(); // Limpa o buffer

    // Pede o segundo número
    println!("Digite o segundo número:");
    io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let num2 = input.trim().parse::<i32>().expect("Por favor, digite um número válido");

    // Calcula e exibe a soma
    let soma = num1 + num2;
    println!("A soma de {} e {} é {}", num1, num2, soma);
}
