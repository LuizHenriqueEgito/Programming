/*
Devemos usar String, &String, &str ou &'static str?
*/

// essa função passa o ownership
fn recebe_String(s1: String, mut s2: String) {
    println!("s1: {s1}");
    println!("s2: {s2}");
    // s1.push_str("_foobar");  // cannot mutate immutable variable 's1'
    s2.push_str("_foobar");
    println!("s2: {s2}");
    println!("APAGANDO s1 e s2");
    println!("Finalizando a função recebe_String\n");
    // ao final dessa função s1 e s2 são destruidos
}

// essa função só EMPRESTA a nossa string (borrow) ela não passa o ownership!
fn recebe_referencia_String(s1: &String, s2: &mut String) {
    println!("s1: {s1}");
    println!("s2: {s2}");
    // s1.push_str("_foobar");  // cannot borrow '*s1' as mutable, as it is behind a '&' reference
    s2.push_str("_foobar");
    println!("Finalizando a função recebe_referencia_String\n");
    // Essa função recebe um emprestimo, faz o que tem que fazer
    // e devolve o emprestimo feito!
}

fn recebe_str_slice(s1: &str, s2: &str, s3: &mut str, s4: &mut str) {
    println!("s1: {s1}");
    println!("s2: {s2}");
    println!("s3: {s3}");
    println!("s4: {s4}");

    // s1.make_ascii_uppercase();  // cannot borrow '*s1' as mutable, as it is behind a '&' reference
    s3.make_ascii_lowercase();  // você pode mudar desde que não MUDE o tamanho dele
    s4.make_ascii_uppercase();  // você pode mudar desde que não MUDE o tamanho dele
    println!("s3.make_ascii_uppercase(): {s3}");
    println!("s4.make_ascii_uppercase(): {s4}");
     println!("Finalizando a função recebe_str_slice\n");
}


fn recebe_str_literal(s1: &str, s2: &'static str) {
    println!("s1: {s1}");
    println!("s2: {s2}");
}

fn main() {
    // String
    let sa = String::from("aaa");
    let mut sb = String::from("bbb");
    recebe_String(sa, sb);
    // sa e sb foram movidos para a função recebe_String e não existem mais!

    // ---
    // &String
    let sc = String::from("ccc");
    let mut sd = String::from("ddd");
    recebe_referencia_String(&sc, &mut sd);
    println!("Nossas variaveis ainda existem!");
    println!("sc: {sc} | sd: {sd}");

    // ---
    // &str
    // criando &str
    let se = String::from("eee");
    let se_str = se.as_str();
    let sf = String::from("fff");
    let sf_str = &sf[0..3];
    let mut sg = String::from("ggg");
    let sg_str = sg.as_mut_str();
    let mut sh = String::from("hhh");
    let sh_str: &mut str = &mut String::from("hhh");
    println!("se_str: {} | sf_str: {} | sg_str: {} | sh_str: {}", se_str, sf_str, sg_str, sh_str);
    // &str são mais abrangentes
    recebe_str_slice(se_str, sf_str, sg_str, sh_str);
    // ele também aceita isso
    recebe_str_slice(&se, "qualquer coisa", sg_str, &mut sh);

    // ---
    // &'static str
    let s_literal = "iii";
    let mut s_literal_mut = "jjj";
    recebe_str_literal(s_literal, s_literal_mut);
}