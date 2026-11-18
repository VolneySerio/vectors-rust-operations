#[derive(Debug, Copy, Clone)]
pub struct Vetor {
    x: f32,
    y: f32
}

impl Vetor {
    pub fn novo(x: f32, y: f32) -> Self{
    Self { x, y }
    }

    pub fn somar(self, outro: Vetor) -> Self{
    Vetor::novo(self.x + outro.x, self.y + outro.y)
    }

    pub fn subtrair(self, outro:Vetor) -> Self {
    Vetor::novo(self.x - outro.x, self.y - outro.y)
    } 

    pub fn inverso(self) -> Self {
    Vetor::novo(- self.x , - self.y)
    }

    pub fn multiplicar(self, escalar: f32) -> Self {
    Vetor::novo(escalar * self.x , escalar * self.y)
    }

    pub fn produto_escalar(self, outro: Vetor) -> f32 {
            self.x * outro.x + self.y * outro.y
    }
    
    pub fn produto_vetorial(self, outro: Vetor) -> f32 {
        self.x * outro.y - self.y * outro.x
    }

    pub fn projetar_sobre(self, outro: Vetor) -> Option<Vetor> {
        let denominador = outro.produto_escalar(outro);

        if denominador == 0.0 {
            return None;
        }

        let fator = self.produto_escalar(outro) / denominador;
        Some(outro.multiplicar(fator))
    }

    pub fn deslizar(self, deslocamento: Vetor) -> Vetor {
            self.somar(deslocamento)
        }

    pub fn refletir_eixo_x(self) -> Vetor {
        Vetor::novo(self.x, -self.y)
    }

    pub fn refletir_eixo_y(self) -> Vetor {
        Vetor::novo(-self.x, self.y)
    }

}