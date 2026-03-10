fn pares_com_soma(lista: &[i32], alvo: i32) {
    for i in 0..lista.len() {
        for j in (i + 1)..lista.len() {
            if lista[i] + lista[j] == alvo {
                println!("{} {}", lista[i], lista[j]);
            }
        }
    }
}

fn main() {
    let lista = [1, 2, 3, 4];

    pares_com_soma(&lista, 5);
}

//Analise: o loop executa n vezes. o algoritimo vai tentar 
// procurar os pares que somam o alvo, e nao repete pares.
//Complexidade: O(n2) 
//Justificativa: esse algoritimo executa loop externo e interno quase n vezes a 
// fim de retornar pares que somam o resultado de acordo com o alvo.