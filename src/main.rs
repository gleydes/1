
fn main() {
/*Dado o ano inicial, subtraia o ano de nascimento para obter a idade da pessoa */

    let nome:&str = "Batman";
    let ano_de_nascimento:u16 =1985;
    let ano_atual:u16 =2024;

    let idade: u16 = ano_atual-ano_de_nascimento;
    println!("O funcionário ({}) nasceu no ano {} e sua idade é: {}",  nome, ano_de_nascimento, idade);
}
