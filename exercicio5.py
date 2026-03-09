def imprimir_pares_e_soma(lista):
    # Bloco 1: imprime cada elemento
    for i in range(len(lista)):
        print(lista[i])
    # Bloco 2: imprime todos os pares
    for i in range(len(lista)):
        for j in range(len(lista)):
            print(lista[i], lista[j])

lista = [1, 2, 3]
imprimir_pares_e_soma(lista)

#analise:

#complexidade: o(n)


