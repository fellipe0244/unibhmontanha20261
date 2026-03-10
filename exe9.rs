fn produto_de_matrizes(A: &Vec<Vec<i32>>, B: &Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>> {

    let mut C = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                C[i][j] += A[i][k] * B[k][j];
            }
        }
    }

    C
}

fn main() {

    let A = vec![
        vec![1, 2],
        vec![3, 4],
    ];

    let B = vec![
        vec![5, 6],
        vec![7, 8],
    ];

    let resultado = produto_de_matrizes(&A, &B, 2);

    println!("{:?}", resultado);
}

//Analise: ele calcula elementos de matrizes e a cada posicao
// e calculada e soma A com B e usa 3 loops
//Complexidade: O(n2)
//Justificativa: o algoritimo usa 3 loops para calcular
//o produto entre 2 matrizes. o loop percorre as posicoes
//da matriz e a execucao cresce proporcionalmente 