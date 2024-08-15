---

# <a href="https://en.wikipedia.org/wiki/Insertion_sort">Insertion Sort</a>

## Introdução

O **Insertion Sort** é um algoritmo de ordenação simples e eficiente para pequenas listas de elementos. Ele constrói a lista final ordenada de forma incremental, uma vez que percorre a entrada e insere cada elemento em sua posição correta. Este algoritmo é similar à forma como as pessoas normalmente organizam cartas em um jogo de baralho: começando com uma mão vazia, uma carta é retirada de cada vez e inserida na posição correta.

## Funcionamento do Insertion Sort

1. **Iteração sobre o array**: Começa no segundo elemento e assume que o primeiro elemento é uma lista ordenada de um único elemento.
2. **Inserção**: Para cada elemento da lista, o algoritmo o insere na posição correta dentro da parte já ordenada.
3. **Comparação**: O elemento atual é comparado com os elementos já ordenados, e é deslocado para a posição correta, deslocando os elementos maiores para a direita.
4. **Repetição**: Este processo é repetido para cada elemento, até que todos os elementos estejam na posição correta.

### Exemplo de Funcionamento

Dado o array: `[23, 5, 17, 89, 12]`

- Começa com o segundo elemento (5) e compara com o primeiro (23), insere 5 antes de 23.
- Continua com o próximo elemento (17), insere entre 5 e 23.
- Repete o processo para todos os elementos restantes até que o array esteja ordenado: `[5, 12, 17, 23, 89]`.

## Comparação com Outros Algoritmos

### Bubble Sort
- **Similaridade**: Ambos são algoritmos simples e intuitivos.
- **Diferença Principal**: O Bubble Sort compara pares adjacentes e faz múltiplas passagens sobre a lista, enquanto o Insertion Sort move elementos para sua posição correta em uma única passagem.

### Selection Sort
- **Similaridade**: Ambos têm complexidade de tempo O(n²).
- **Diferença Principal**: O Selection Sort seleciona o menor elemento e o move para o início da lista, enquanto o Insertion Sort insere cada elemento na posição correta em uma lista já ordenada.

### Merge Sort
- **Similaridade**: Ambos podem ser usados para ordenar listas.
- **Diferença Principal**: O Merge Sort é um algoritmo de ordenação por divisão e conquista, mais eficiente em grandes listas com complexidade O(n log n), enquanto o Insertion Sort é mais eficiente para listas pequenas ou quase ordenadas.

### Quick Sort
- **Similaridade**: Ambos podem ser implementados de forma recursiva.
- **Diferença Principal**: O Quick Sort é geralmente mais rápido para grandes listas, mas tem pior caso O(n²), enquanto o Insertion Sort é estável e eficiente para pequenas listas.

## Complexidade

- **Tempo no melhor caso**: O(n) — ocorre quando a lista já está ordenada.
- **Tempo no caso médio e pior caso**: O(n²) — ocorre quando a lista está em ordem inversa.
- **Espaço**: O(1) — é um algoritmo em tempo constante (in-place), pois não requer espaço adicional além da lista original.

## Uso Ideal

- Pequenas listas ou listas que já estão quase ordenadas.
- Aplicações onde a simplicidade e a facilidade de implementação são mais importantes do que a eficiência em grandes listas.

