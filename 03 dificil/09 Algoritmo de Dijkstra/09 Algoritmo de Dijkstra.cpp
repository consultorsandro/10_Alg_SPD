// Algoritmo de Dijkstra em C++
// Este código implementa o algoritmo de Dijkstra para encontrar o caminho mais curto
// de um vértice de origem a todos os outros vértices em um grafo ponderado.
#include <iostream>
#include <vector>
#include <queue>
#include <climits>
using namespace std;

// Estrutura para representar uma aresta
struct Edge {
    int to, weight;
};

// Algoritmo de Dijkstra
vector<int> dijkstra(const vector<vector<Edge>>& graph, int start, int n) {
    // Inicializar distâncias com infinito
    vector<int> dist(n, INT_MAX);
    dist[start] = 0;

    // Fila de prioridade para selecionar o vértice com menor distância
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<>> pq;
    pq.push({0, start});

    while (!pq.empty()) {
        // Obter vértice com menor distância atual
        int d = pq.top().first;
        int u = pq.top().second;
        pq.pop();

        // Se a distância já foi otimizada, ignorar
        if (d > dist[u]) continue;

        // Processar vizinhos
        for (const auto& edge : graph[u]) {
            int v = edge.to;
            int weight = edge.weight;

            // Se encontrar um caminho mais curto, atualizar
            if (dist[u] + weight < dist[v]) {
                dist[v] = dist[u] + weight;
                pq.push({dist[v], v});
            }
        }
    }
    return dist;
}

// Função principal para executar o exemplo
int main() {
    int n = 4; // 4 vértices
    vector<vector<Edge>> graph(n);

    // Adicionar arestas: {vértice destino, peso}
    graph[0].push_back({1, 4});
    graph[0].push_back({2, 8});
    graph[1].push_back({2, 2});
    graph[1].push_back({3, 5});
    graph[2].push_back({3, 1});

    // Executar Dijkstra a partir do vértice 0
    auto dist = dijkstra(graph, 0, n);

    // Exibir resultados
    cout << "Distancias mais curtas a partir do vértice 0:\n";
    for (int i = 0; i < n; ++i)
        cout << "Para vértice " << i << ": " << dist[i] << endl;
    return 0;
}