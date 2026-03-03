def pares_com_soma(lista, alvo):
    for i in range(len(lista)):
        for j in range(i + 1, len(lista)):
            if lista[i] + lista[j] == alvo:
                print(lista[i], lista[j])

lista = [1, 2, 3, 4]

pares_com_soma(lista, 5)

#analise: o loop executa n vezes. o algoritimo vai tentar procurar os pares que somam o alvo, e nao repete pares.

#complexidade: o(n2) 

#justificativa: esse algoritimo executa loop externo e interno quase n vezes a fim de retornar pares que somam o resultado de acordo com o alvo.
