// filepath: algoritmos-projeto/facil/algoritmo1.cpp
// Este programa calcula a soma de dois números inteiros e exibe o resultado.
#include <iostream>
using namespace std;

int main() {
    int num1, num2, soma;

    // Solicita ao usuário para digitar dois números
    cout << "Digite o primeiro número: ";
    cin >> num1;
    cout << "Digite o segundo número: ";
    cin >> num2;

    // Calcula a soma
    soma = num1 + num2;

    // Exibe o resultado
    cout << "A soma de " << num1 << " e " << num2 << " é: " << soma << endl;

    return 0;
}