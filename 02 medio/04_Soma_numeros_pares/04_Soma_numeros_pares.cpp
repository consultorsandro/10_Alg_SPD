// filepath: /algoritmos-projeto/algoritmos-projeto/medio/algoritmo4.cpp
// Este programa calcula a soma dos números pares em um intervalo fornecido pelo usuário.
// Exemplo: Se o usuário fornecer 1 e 10, a soma dos números pares (2, 4, 6, 8, 10) será 30.

#include <iostream>
using namespace std;

int main() {
    int inicio, fim, soma = 0;

    // Solicita ao usuário o intervalo
    cout << "Digite o início do intervalo: ";
    cin >> inicio;
    cout << "Digite o fim do intervalo: ";
    cin >> fim;

    // Verifica se o intervalo é válido
    if (inicio > fim) {
        cout << "O início do intervalo deve ser menor ou igual ao fim." << endl;
        return 1; // Encerra o programa com erro
    }

    // Calcula a soma dos números pares no intervalo
    for (int i = inicio; i <= fim; i++) {
        if (i % 2 == 0) { // Verifica se o número é par
            soma += i; // Adiciona o número par à soma
        }
    }

    // Exibe o resultado
    cout << "A soma dos números pares entre " << inicio << " e " << fim << " é: " << soma << endl;

    return 0;
}