fn fibonacci_recursivo(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}

fn main() {
    println!("{}", fibonacci_recursivo(10));
}

//Analise: a funcao faz recursao e calcula o valor de fibonacci
// e a cada chamada gera outras 2 chamadas formando uma arvore de chamadas.
//Complexidade: O(2n)
//Justificativa: como cada chamada gera mais 2 chamadas o número cresce conforme n aumenta.