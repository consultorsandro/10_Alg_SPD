// Função para calcular o fatorial de um número
fn calcular_fatorial(n: u64) -> u64 {
    // Passo 1: Verifica se o número é 0 ou 1
    if n == 0 || n == 1 {
        return 1;
    }

    // Passo 2: Inicializa a variável para o resultado
    let mut resultado = 1;

    // Passo 3: Multiplica os números de 2 até n
    for i in 2..=n {
        resultado *= i;// Multiplica o resultado pelo número atual
    }

    // Passo 4: Retorna o fatorial
    resultado
}

fn main() {
    // Exemplo com um valor simples
    let numero = 5; // Fatorial de 5 = 5 * 4 * 3 * 2 * 1 = 120

    // Exibe o resultado 
    println!(
        "O fatorial de {} é: {}",
        numero,
        calcular_fatorial(numero)
    );
}

