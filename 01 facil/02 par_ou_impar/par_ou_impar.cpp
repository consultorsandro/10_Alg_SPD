#include <iostream>
using namespace std;

int main() {
    int numero;

    cout << "Digite um número: ";
    cin >> numero;

    if (numero % 2 == 0) {
        cout << "O número " << numero << " é par." << endl;
    } else {
        cout << "O número " << numero << " é ímpar." << endl;
    }

    return 0;
}
