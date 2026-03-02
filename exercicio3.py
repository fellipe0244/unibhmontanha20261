def busca_binaria(lista, alvo):
    esquerda, direita = 0, len(lista) - 1
    while esquerda <= direita:
        meio = (esquerda + direita) // 2
        if lista[meio] == alvo:
            return meio
        elif lista[meio] < alvo:
            esquerda = meio + 1
        else:
            direita = meio - 1
    return -1

lista = [1, 3, 5, 7, 9, 11, 13, 15]

print(busca_binaria(lista, 7))
print(busca_binaria(lista, 1))
print(busca_binaria(lista, 8))

#analise: vai sempre pegar o elemento do meio. Decide se vai procurar na metade da direita e esquerda. Descarta metade da lista a cada passo.

#complexidade: o(log n)

#justificativa: quando o algoritimo roda o loop, vai cortar a lista pela metade. Se voce continua dividindo algo por 2 nao vai precisar de muita divisao pra chegar no 1. Por isso o tempo de execucacao cresce.
