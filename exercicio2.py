def somar_lista(lista):
 total = 0
 for elemento in lista:
  total += elemento
 return total

print(somar_lista([1, 2, 3, 4]))
print(somar_lista([10, 20]))
print(somar_lista([]))
