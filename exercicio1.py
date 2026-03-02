def verificar_primeiro(lista):
    if len(lista) == 0:
        return None
    return lista[0]

print(verificar_primeiro([10, 20, 30]))
print(verificar_primeiro([]))

#complexidade: o(1)

#justificava: a funcao verifica o tamanho da lista mas pega o primeiro, então mesmo que a lista seja grande, o tempo continua o mesmo.
