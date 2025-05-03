#include <iostream>

// Função para ordenar um vetor usando Bubble Sort otimizado
void bubbleSort(double arr[], int tamanho) {
    // Passo 1: Itera sobre o vetor
    for (int i = 0; i < tamanho - 1; i++) {
        bool trocou = false; // Flag para otimização
        
        // Passo 2: Compara e troca elementos adjacentes
        for (int j = 0; j < tamanho - 1 - i; j++) {
            if (arr[j] > arr[j + 1]) {
                // Troca os elementos
                double temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                trocou = true; // Marca que houve troca
            }
        }
        
        // Passo 3: Se não houve trocas, o vetor está ordenado
        if (!trocou) {
            break;
        }
    }
}

int main() {
    // Exemplo com valores decimais simples
    double vetor[] = {5.2, 3.1, 7.8, 1.5, 4.9};
    int tamanho = 5;
    
    // Exibe o vetor original
    std::cout << "Vetor original: ";
    for (int i = 0; i < tamanho; i++) {
        std::cout << vetor[i] << " ";
    }
    std::cout << std::endl;
    
    // Chama a função de ordenação
    bubbleSort(vetor, tamanho);
    
    // Exibe o vetor ordenado
    std::cout << "Vetor ordenado: ";
    for (int i = 0; i < tamanho; i++) {
        std::cout << vetor[i] << " ";
    }
    std::cout << std::endl;
    
    return 0;
}