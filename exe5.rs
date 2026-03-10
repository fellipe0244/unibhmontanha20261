fn imprimir_pares_e_soma(lista: &[i32]) {

    for i in 0..lista.len() {
        println!("{}", lista[i]);
    }

    for i in 0..lista.len() {
        for j in 0..lista.len() {
            println!("{} {}", lista[i], lista[j]);
        }
    }
}

fn main() {
    let lista = [1, 2, 3];
    imprimir_pares_e_soma(&lista);
}

//Analise: o algaritimo percorre primeiro todos os elementos da lista
// o segundo usa loop para pegar todas as combinações de pares.
//Complexidade: O(n2)
//Justificativa: na primeira parte vai ser executado n vezes, a segunda 
//utiliza de loop para combinar os pares
