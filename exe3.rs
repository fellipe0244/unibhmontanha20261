fn busca_binaria(lista: &[i32], alvo: i32) -> i32 {
    let mut esquerda: i32 = 0;
    let mut direita: i32 = lista.len() as i32 - 1;

    while esquerda <= direita {
        let meio = (esquerda + direita) / 2;

        if lista[meio as usize] == alvo {
            return meio;
        } else if lista[meio as usize] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }

    -1
}

fn main() {
    let lista = [1, 3, 5, 7, 9, 11, 13, 15];

    println!("{}", busca_binaria(&lista, 7));
    println!("{}", busca_binaria(&lista, 1));
    println!("{}", busca_binaria(&lista, 8));
}

//Analise: vai sempre pegar o elemento do meio. Decide se vai procurar na 
// metade da direita e esquerda. Descarta metade da lista a cada passo.
//Complexidade: O(log n)
//Justificativa: quando o algoritimo roda o loop, vai cortar a lista pela metade. 
// Se voce continua dividindo algo por 2 nao vai precisar de muita divisao pra chegar no 1. 
// Por isso o tempo de execucacao cresce.