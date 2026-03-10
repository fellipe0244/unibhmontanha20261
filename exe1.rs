fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    if lista.len() == 0 {
        return None;
    }

    Some(lista[0])
}

fn main() {
    println!("{:?}", verificar_primeiro(&[10, 20, 30]));
    println!("{:?}", verificar_primeiro(&[]));
}

//Analise: esse algoritimo vai verificar o tamanho da lista e procurar o 0.
//Complexidade: O(1)
//Justificativa: a funcao verifica o tamanho da lista mas pega o primeiro, 
// então mesmo que a lista seja grande, o tempo continua o mesmo.