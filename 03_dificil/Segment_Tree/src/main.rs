// Estrutura para representar a Segment Tree
struct SegmentTree {
    tree: Vec<i32>, // Vetor que armazena a árvore segmentada
    n: usize,       // Tamanho do array original
}

impl SegmentTree {
    // Construtor da Segment Tree: recebe um slice com os dados iniciais
    fn new(arr: &[i32]) -> Self {
        let n = arr.len(); // Número de elementos no array original
        let mut tree = vec![0; 4 * n]; // Inicializa a árvore com zeros (tamanho 4 * n para garantir espaço)
        let mut st = SegmentTree { tree, n };
        st.build(arr, 0, 0, n - 1); // Constrói a árvore a partir dos dados
        st
    }

    // Função recursiva para construir a árvore a partir do array original
    fn build(&mut self, arr: &[i32], node: usize, start: usize, end: usize) {
        if start == end {
            // Caso base: folha da árvore
            self.tree[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            // Construir o filho esquerdo
            self.build(arr, 2 * node + 1, start, mid);
            // Construir o filho direito
            self.build(arr, 2 * node + 2, mid + 1, end);
            // Armazena a soma dos filhos no nó atual
            self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
        }
    }

    // Interface pública para atualizar um valor no array original
    fn update(&mut self, idx: usize, val: i32) {
        self.update_rec(0, 0, self.n - 1, idx, val);
    }

    // Função recursiva para atualizar a árvore quando um valor do array muda
    fn update_rec(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i32) {
        if start == end {
            // Atualiza o valor na folha
            self.tree[node] = val;
        } else {
            let mid = (start + end) / 2;
            // Decide se vai atualizar o filho esquerdo ou direito
            if idx <= mid {
                self.update_rec(2 * node + 1, start, mid, idx, val);
            } else {
                self.update_rec(2 * node + 2, mid + 1, end, idx, val);
            }
            // Após atualizar, recalcula a soma no nó atual
            self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
        }
    }

    // Interface pública para consultar a soma em um intervalo [l, r]
    fn query(&self, l: usize, r: usize) -> i32 {
        self.query_rec(0, 0, self.n - 1, l, r)
    }

    // Função recursiva para consultar a soma no intervalo [l, r]
    fn query_rec(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        if r < start || l > end {
            // Intervalo fora do alcance da consulta
            0
        } else if l <= start && end <= r {
            // Intervalo totalmente dentro do intervalo de consulta
            self.tree[node]
        } else {
            // Intervalo parcialmente sobreposto: divide e conquista
            let mid = (start + end) / 2;
            let left_sum = self.query_rec(2 * node + 1, start, mid, l, r);
            let right_sum = self.query_rec(2 * node + 2, mid + 1, end, l, r);
            left_sum + right_sum
        }
    }
}

// Função principal que demonstra o uso da Segment Tree
fn main() {
    // Array original
    let arr = vec![1, 3, 5, 7, 9];

    // Cria a Segment Tree a partir do array
    let mut st = SegmentTree::new(&arr);

    // Consulta a soma no intervalo [1, 3] (3 + 5 + 7 = 15)
    println!("Soma no intervalo [1, 3]: {}", st.query(1, 3));

    // Atualiza o valor no índice 2 para 10 (altera o 5 para 10)
    st.update(2, 10);

    // Consulta novamente após a atualização (3 + 10 + 7 = 20)
    println!("Soma no intervalo [1, 3] após atualização: {}", st.query(1, 3));
}
