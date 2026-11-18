
mod vetor;

use vetor::Vetor;

fn main() {
    let a = Vetor::novo(4.0, 6.0);
    let b = Vetor::novo(2.0, 3.0);

    println!("Vetor A: {:?}", a);
    println!("Vetor B: {:?}", b);

    println!("Soma: {:?}", a.somar(b));
    println!("Subtração: {:?}", a.subtrair(b));
    println!("Inverso de A: {:?}", a.inverso());
    println!("A multiplicado por 2: {:?}", a.multiplicar(2.0));

    println!("Produto escalar: {}", a.produto_escalar(b));
    println!("Produto vetorial: {}", a.produto_vetorial(b));

    println!("Projeção de A sobre B: {:?}", a.projetar_sobre(b));

    println!("A deslizado por B: {:?}", a.deslizar(b));

    println!("Reflexão de A no eixo X: {:?}", a.refletir_eixo_x());
    println!("Reflexão de A no eixo Y: {:?}", a.refletir_eixo_y());
}
