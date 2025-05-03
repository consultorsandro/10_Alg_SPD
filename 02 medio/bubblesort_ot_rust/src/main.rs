// Função para ordenar um vetor usando Bubble Sort otimizado
fn bubble_sort(arr: &mut [f64]) {
    let tamanho = arr.len();

    // Passo 1: Itera sobre o vetor
    for i in 0..tamanho - 1 {
        let mut trocou = false; // Flag para otimização

        // Passo 2: Compara e troca elementos adjacentes
        for j in 0..tamanho - 1 - i {
            if arr[j] > arr[j + 1] {
                // Troca os elementos
                arr.swap(j, j + 1);// Troca os elementos
                trocou = true; // Marca que houve troca
            }
        }

        // Passo 3: Se não houve trocas, o vetor está ordenado
        if !trocou {
            break;
        }
    }
}

fn main() {
    // Exemplo com valores decimais simples
    let mut vetor = [5.2, 3.1, 7.8, 1.5, 4.9];

    // Exibe o vetor original
    println!("Vetor original: {:?}", vetor);

    // Chama a função de ordenação
    bubble_sort(&mut vetor);

    // Exibe o vetor ordenado
    println!("Vetor ordenado: {:?}", vetor);
}
