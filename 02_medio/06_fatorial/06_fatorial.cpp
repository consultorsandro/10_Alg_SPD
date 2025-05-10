#include <iostream>

// Função para calcular o fatorial de um número
unsigned long long calcularFatorial(int n) {
    // Passo 1: Verifica se o número é 0 ou 1
    if (n == 0 || n == 1) {
        return 1;
    }
    
    // Passo 2: Inicializa a variável para o resultado
    unsigned long long resultado = 1;//
    
    // Passo 3: Multiplica os números de 2 até n ; Utiliza iteração para reduzir o esforço computacional
    for (int i = 2; i <= n; i++) {
        resultado *= i;
    }
    
    // Passo 4: Retorna o fatorial
    return resultado;
}

int main() {
    // Exemplo com um valor simples
    int numero = 5; // Fatorial de 5 = 5 * 4 * 3 * 2 * 1 = 120
    
    // Exibe o resultado
    std::cout << "O fatorial de " << numero << " eh: " 
              << calcularFatorial(numero) << std::endl;
    
    return 0;
}