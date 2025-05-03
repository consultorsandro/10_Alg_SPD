//Algoritmo de Strassen para multiplicação de matrizes
#include <iostream>
#include <vector>
using namespace std;

// Função auxiliar para alocar matriz
vector<vector<int>> allocateMatrix(int n) {
    return vector<vector<int>>(n, vector<int>(n, 0));
}

// Função auxiliar para somar ou subtrair matrizes
vector<vector<int>> addSubtractMatrix(const vector<vector<int>>& A, const vector<vector<int>>& B, bool add, int n) {
    vector<vector<int>> result = allocateMatrix(n);
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < n; ++j)
            result[i][j] = add ? A[i][j] + B[i][j] : A[i][j] - B[i][j];
    return result;
}

// Algoritmo de Strassen para multiplicação de matrizes
vector<vector<int>> strassen(const vector<vector<int>>& A, const vector<vector<int>>& B, int n) {
    // Caso base: matriz 1x1
    if (n == 1) {
        vector<vector<int>> C = allocateMatrix(1);
        C[0][0] = A[0][0] * B[0][0];
        return C;
    }

    // Dividir matrizes em 4 submatrizes
    int mid = n / 2;
    vector<vector<int>> A11(mid, vector<int>(mid)), A12(mid, vector<int>(mid)),
                        A21(mid, vector<int>(mid)), A22(mid, vector<int>(mid)),
                        B11(mid, vector<int>(mid)), B12(mid, vector<int>(mid)),
                        B21(mid, vector<int>(mid)), B22(mid, vector<int>(mid));

    for (int i = 0; i < mid; ++i) {
        for (int j = 0; j < mid; ++j) {
            A11[i][j] = A[i][j];
            A12[i][j] = A[i][j + mid];
            A21[i][j] = A[i + mid][j];
            A22[i][j] = A[i + mid][j + mid];
            B11[i][j] = B[i][j];
            B12[i][j] = B[i][j + mid];
            B21[i][j] = B[i + mid][j];
            B22[i][j] = B[i + mid][j + mid];
        }
    }

    // Calcular os 7 produtos de Strassen
    auto P1 = strassen(addSubtractMatrix(A11, A22, true, mid), addSubtractMatrix(B11, B22, true, mid), mid);
    auto P2 = strassen(addSubtractMatrix(A21, A22, true, mid), B11, mid);
    auto P3 = strassen(A11, addSubtractMatrix(B12, B22, false, mid), mid);
    auto P4 = strassen(A22, addSubtractMatrix(B21, B11, false, mid), mid);
    auto P5 = strassen(addSubtractMatrix(A11, A12, true, mid), B22, mid);
    auto P6 = strassen(addSubtractMatrix(A21, A11, false, mid), addSubtractMatrix(B11, B12, true, mid), mid);
    auto P7 = strassen(addSubtractMatrix(A12, A22, false, mid), addSubtractMatrix(B21, B22, true, mid), mid);

    // Calcular submatrizes da matriz resultante
    vector<vector<int>> C11(mid, vector<int>(mid)), C12(mid, vector<int>(mid)),
                        C21(mid, vector<int>(mid)), C22(mid, vector<int>(mid));
    for (int i = 0; i < mid; ++i) {
        for (int j = 0; j < mid; ++j) {
            C11[i][j] = P1[i][j] + P4[i][j] - P5[i][j] + P7[i][j];
            C12[i][j] = P3[i][j] + P5[i][j];
            C21[i][j] = P2[i][j] + P4[i][j];
            C22[i][j] = P1[i][j] - P2[i][j] + P3[i][j] + P6[i][j];
        }
    }

    // Combinar submatrizes em C
    vector<vector<int>> C = allocateMatrix(n);
    for (int i = 0; i < mid; ++i) {
        for (int j = 0; j < mid; ++j) {
            C[i][j] = C11[i][j];
            C[i][j + mid] = C12[i][j];
            C[i + mid][j] = C21[i][j];
            C[i + mid][j + mid] = C22[i][j];
        }
    }
    return C;
}

// Função principal para executar o exemplo
int main() {
    // Matrizes 2x2 de exemplo
    vector<vector<int>> A = {{1, 2}, {3, 4}};
    vector<vector<int>> B = {{5, 6}, {7, 8}};
    int n = 2;

    // Multiplicação
    auto C = strassen(A, B, n);

    // Exibir resultado
    cout << "Resultado da multiplicação de matrizes (Strassen):\n";
    for (const auto& row : C) {
        for (int val : row) cout << val << " ";
        cout << endl;
    }
    return 0;
}