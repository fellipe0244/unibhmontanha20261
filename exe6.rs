fn potencias_de_dois(n: i32) {
    let mut i = 1;

    while i < n {
        println!("{}", i);
        i *= 2;
    }
}

fn main() {
    potencias_de_dois(20);
}

//Analise: a variavel começa com 1 e a cada interação ela dobra até chegar em n
//Complexidade: O(log n)
//Justificativa: o valor de i começa em 1 e dobra no loop e vai dobrando até chegar em n
