// Função auxiliar para alocar uma matriz quadrada n x n preenchida com zeros
fn allocate_matrix(n: usize) -> Vec<Vec<i32>> {
    vec![vec![0; n]; n]
}

// Função auxiliar para somar ou subtrair duas matrizes A e B
fn add_subtract_matrix(
    a: &Vec<Vec<i32>>,
    b: &Vec<Vec<i32>>,
    add: bool, // true para soma, false para subtração
    n: usize,
) -> Vec<Vec<i32>> {
    let mut result = allocate_matrix(n);
    for i in 0..n {
        for j in 0..n {
            result[i][j] = if add {
                a[i][j] + b[i][j]
            } else {
                a[i][j] - b[i][j]
            };
        }
    }
    result
}

// Algoritmo de Strassen para multiplicação de matrizes quadradas A e B de ordem n
fn strassen(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>> {
    // Caso base: matrizes 1x1
    if n == 1 {
        return vec![vec![a[0][0] * b[0][0]]];
    }

    // Dividir A e B em 4 submatrizes cada
    let mid = n / 2;
    let mut a11 = allocate_matrix(mid);
    let mut a12 = allocate_matrix(mid);
    let mut a21 = allocate_matrix(mid);
    let mut a22 = allocate_matrix(mid);
    let mut b11 = allocate_matrix(mid);
    let mut b12 = allocate_matrix(mid);
    let mut b21 = allocate_matrix(mid);
    let mut b22 = allocate_matrix(mid);

    // Preencher submatrizes com base nas posições em A e B
    for i in 0..mid {
        for j in 0..mid {
            a11[i][j] = a[i][j];
            a12[i][j] = a[i][j + mid];
            a21[i][j] = a[i + mid][j];
            a22[i][j] = a[i + mid][j + mid];
            b11[i][j] = b[i][j];
            b12[i][j] = b[i][j + mid];
            b21[i][j] = b[i + mid][j];
            b22[i][j] = b[i + mid][j + mid];
        }
    }

    // Computar os 7 produtos intermediários de Strassen usando recursão
    let p1 = strassen(
        &add_subtract_matrix(&a11, &a22, true, mid),
        &add_subtract_matrix(&b11, &b22, true, mid),
        mid,
    );
    let p2 = strassen(&add_subtract_matrix(&a21, &a22, true, mid), &b11, mid);
    let p3 = strassen(&a11, &add_subtract_matrix(&b12, &b22, false, mid), mid);
    let p4 = strassen(&a22, &add_subtract_matrix(&b21, &b11, false, mid), mid);
    let p5 = strassen(&add_subtract_matrix(&a11, &a12, true, mid), &b22, mid);
    let p6 = strassen(
        &add_subtract_matrix(&a21, &a11, false, mid),
        &add_subtract_matrix(&b11, &b12, true, mid),
        mid,
    );
    let p7 = strassen(
        &add_subtract_matrix(&a12, &a22, false, mid),
        &add_subtract_matrix(&b21, &b22, true, mid),
        mid,
    );

    // Usar os 7 produtos para montar os quadrantes da matriz resultante C
    let mut c11 = allocate_matrix(mid);
    let mut c12 = allocate_matrix(mid);
    let mut c21 = allocate_matrix(mid);
    let mut c22 = allocate_matrix(mid);

    for i in 0..mid {
        for j in 0..mid {
            c11[i][j] = p1[i][j] + p4[i][j] - p5[i][j] + p7[i][j];
            c12[i][j] = p3[i][j] + p5[i][j];
            c21[i][j] = p2[i][j] + p4[i][j];
            c22[i][j] = p1[i][j] - p2[i][j] + p3[i][j] + p6[i][j];
        }
    }

    // Juntar os quatro quadrantes (submatrizes) em uma matriz única C
    let mut c = allocate_matrix(n);
    for i in 0..mid {
        for j in 0..mid {
            c[i][j] = c11[i][j];
            c[i][j + mid] = c12[i][j];
            c[i + mid][j] = c21[i][j];
            c[i + mid][j + mid] = c22[i][j];
        }
    }

    c
}

// Função principal que executa um exemplo de multiplicação
fn main() {
    // Matrizes 2x2 de exemplo
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];
    let n = 2;

    // Realiza a multiplicação usando o algoritmo de Strassen
    let c = strassen(&a, &b, n);

    // Exibe o resultado
    println!("Resultado da multiplicação de matrizes (Strassen):");
    for row in c {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}
