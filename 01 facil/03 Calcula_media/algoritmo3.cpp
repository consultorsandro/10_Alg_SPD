// filepath: algoritmos-projeto/facil/algoritmo3.cpp
// Este programa calcula a soma de todos os números inteiros de 1 até um número fornecido pelo usuário.
#include <iostream>
using namespace std;

int main() {
    int n, soma = 0;

    // Solicita ao usuário um número inteiro
    cout << "Digite um número inteiro: ";
    cin >> n;

    // Calcula a soma de todos os números de 1 até n
    for (int i = 1; i <= n; i++) {
        soma += i; // Adiciona i à soma
    }

    // Exibe o resultado
    cout << "A soma de todos os números de 1 até " << n << " é: " << soma << endl;

    return 0;
}