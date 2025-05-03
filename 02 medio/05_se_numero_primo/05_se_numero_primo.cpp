// filepath: /algoritmos-projeto/algoritmos-projeto/medio/algoritmo5.cpp
// Este arquivo contém a solução para o segundo algoritmo de média dificuldade.
// O código é comentado e apresenta um passo a passo da solução, com exemplos simples.

#include <iostream>
using namespace std;

// Função que verifica se um número é primo
bool isPrime(int num) {
    if (num <= 1) return false; // Números menores ou iguais a 1 não são primos
    for (int i = 2; i <= num / 2; i++) {
        if (num % i == 0) return false; // Se o número for divisível por i, não é primo
    }
    return true; // Se não for divisível por nenhum número, é primo
}

int main() {
    int number;

    // Solicita ao usuário um número
    cout << "Digite um número para verificar se é primo: ";
    cin >> number;

    // Verifica se o número é primo e exibe o resultado
    if (isPrime(number)) {
        cout << number << " é um número primo." << endl;
    } else {
        cout << number << " não é um número primo." << endl;
    }

    return 0;
}