fn somar_lista(lista: &[i32]) -> i32 {
    let mut total = 0;

    for elemento in lista {
        total += elemento;
    }

    total
}

fn main() {
    println!("{}", somar_lista(&[1, 2, 3, 4]));
    println!("{}", somar_lista(&[10, 20]));
    println!("{}", somar_lista(&[]));
}

//Analise: percorre todos os elementos da lista para retornar um inteiro
//Complexidade: O(n)
//Justificativa: a funcao vai somar os numeros passando um por um, se a lista tiver 5 itens ela vai fazer 5 passos, 
// se tiver 10 itens vai ter 10 passos, ou seja quanto maior a lista, mais trabalho tem.