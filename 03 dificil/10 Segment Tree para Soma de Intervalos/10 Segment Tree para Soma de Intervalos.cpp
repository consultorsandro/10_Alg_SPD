// 
#include <iostream>
#include <vector>
using namespace std;

// Estrutura da Segment Tree
class SegmentTree {
    vector<int> tree;
    int n;

    // Construir a árvore a partir do array
    void build(const vector<int>& arr, int node, int start, int end) {
        if (start == end) {
            tree[node] = arr[start];
            return;
        }
        int mid = (start + end) / 2;
        build(arr, 2 * node + 1, start, mid);
        build(arr, 2 * node + 2, mid + 1, end);
        tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
    }

    // Atualizar um valor no índice idx
    void update(int node, int start, int end, int idx, int val) {
        if (start == end) {
            tree[node] = val;
            return;
        }
        int mid = (start + end) / 2;
        if (idx <= mid)
            update(2 * node + 1, start, mid, idx, val);
        else
            update(2 * node + 2, mid + 1, end, idx, val);
        tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
    }

    // Consultar soma no intervalo [l, r]
    int query(int node, int start, int end, int l, int r) {
        if (r < start || l > end) return 0;
        if (l <= start && r >= end) return tree[node];
        int mid = (start + end) / 2;
        return query(2 * node + 1, start, mid, l, r) +
               query(2 * node + 2, mid + 1, end, l, r);
    }

public:
    SegmentTree(const vector<int>& arr) {
        n = arr.size();
        tree.resize(4 * n);
        build(arr, 0, 0, n - 1);
    }

    void update(int idx, int val) {
        update(0, 0, n - 1, idx, val);
    }

    int query(int l, int r) {
        return query(0, 0, n - 1, l, r);
    }
};

// Função principal para executar o exemplo
int main() {
    // Array de exemplo
    vector<int> arr = {1, 3, 5, 7, 9};

    // Criar Segment Tree
    SegmentTree st(arr);

    // Consultar soma no intervalo [1, 3]
    cout << "Soma no intervalo [1, 3]: " << st.query(1, 3) << endl;

    // Atualizar valor no índice 2 para 10
    st.update(2, 10);

    // Consultar soma novamente no intervalo [1, 3]
    cout << "Soma no intervalo [1, 3] após atualização: " << st.query(1, 3) << endl;
    return 0;
}