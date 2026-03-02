def somar_lista(lista):
 total = 0
 for elemento in lista:
  total += elemento
 return total

print(somar_lista([1, 2, 3, 4]))
print(somar_lista([10, 20]))
print(somar_lista([]))

#complexidade: o(n)

#justificativa: a funcao vai somar os numeros passando um por um, se a lista tiver 5 itens ela vai fazer 5 passos, se tiver 10 itens vai ter 10 passos, ou seja quanto maior a lista, mais trabalho tem.
