fn ordenacao_bolha(lista: &mut [i32]) {

    let n = lista.len();

    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}

fn main() {

    let mut lista = [5, 3, 8, 4, 2];

    ordenacao_bolha(&mut lista);

    println!("{:?}", lista);
}

//Analise: compara elementos vizinhos da lista e 
// troca se tiver na ordem errada
//Complexidade: O(n log n)
//Justificativa: ele utiliza dois loops, o loop percorre uma lista inteira
//o outro percorre os elementos e faz as trocas