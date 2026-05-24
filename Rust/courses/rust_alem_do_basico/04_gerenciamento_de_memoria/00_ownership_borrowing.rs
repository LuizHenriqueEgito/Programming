fn main() {
    let nome = "Egito";
    let nome_copy = nome;
    println!("{}, {}", nome, nome_copy);  // Isso funciona pois &str implementa o copy
    
    let x: u8 = 8;
    let y = x;
    println!("{}, {}", x, y);  // Isso funciona pois tabém implementa copy
    // Em geral tipos primitivos implementam copy, é mais fácil copiar na Stack
    // Por isso eles não possuem ownership

    let nome_empresa: String = String::from("It");
    // referencia
    let empresa = &nome_empresa;
    println!("{} | {}", nome_empresa, empresa);
    println!("{}", *empresa);
    /*
    A variavel empresa o conteudo dele não é um copy
    ele copiou o endereçamento da variavel nome_empresa
    isso significa que a variavel empresa tem o mesmo
    endereço da variavel nome_empresa.
    */
}