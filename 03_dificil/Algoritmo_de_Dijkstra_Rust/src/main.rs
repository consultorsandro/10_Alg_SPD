use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Estrutura para representar uma aresta com destino e peso
#[derive(Clone)]// Adiciona Clone para permitir cópias
struct Edge {
    to: usize,
    weight: i32,
}

// Algoritmo de Dijkstra
fn dijkstra(graph: &Vec<Vec<Edge>>, start: usize, n: usize) -> Vec<i32> {
    // Inicializar vetor de distâncias com "infinito" (i32::MAX)
    let mut dist = vec![i32::MAX; n];
    dist[start] = 0;

    // Fila de prioridade (min-heap) com tuplas (distância, vértice)
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = pq.pop() {
        // Ignorar se a distância atual já é maior que a registrada
        if d > dist[u] {
            continue;
        }

        // Explorar vizinhos do vértice atual
        for edge in &graph[u] {
            let v = edge.to;
            let weight = edge.weight;

            // Relaxamento da aresta: se achou um caminho mais curto, atualiza
            if dist[u] + weight < dist[v] {
                dist[v] = dist[u] + weight;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }

    dist
}

fn main() {
    let n = 4; // Número de vértices
    let mut graph = vec![vec![]; n];

    // Adicionar arestas: Edge { destino, peso }
    graph[0].push(Edge { to: 1, weight: 4 });
    graph[0].push(Edge { to: 2, weight: 8 });
    graph[1].push(Edge { to: 2, weight: 2 });
    graph[1].push(Edge { to: 3, weight: 5 });
    graph[2].push(Edge { to: 3, weight: 1 });

    // Executar Dijkstra a partir do vértice 0
    let dist = dijkstra(&graph, 0, n);

    // Imprimir distâncias mínimas
    println!("Distâncias mais curtas a partir do vértice 0:");
    for (i, d) in dist.iter().enumerate() {
        println!("Para vértice {}: {}", i, d);
    }
}
