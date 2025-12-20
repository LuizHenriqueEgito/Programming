/*
Pilha (Stack): Coloco e tiro coisas do topo da pilha
    - Existe uma pilha para cada thread
    - Pilha é mais rápida para alocar e acessar
    - É onde são colocados os parâmetros e as variáveis locais das funções
    fn f(i: i32, s: String) { // i, s e x estão na stack mas o String não
        let x: f64            // mas o s não é o String? Não, s é uma referencia um ponteiro para o String
    }

Heap: É alocado dinamicamente (como o malloc/free do C implícitos)
    - A Alocação é mais demorada, gestão das áreas livres e ocupadas
    - Para encontrar o valor na Heap é preciso de um pointer
    - Como o pointer tem tamanho fixo e conhecido, ele pode ficar na stack
    String::from("qwerty");  // Um String é criado no Heap

---
Isso evita bugs no Rust, hello é sobrescrito por ahoy e deixa de existir
s = String::from("hello")
s = String::from("ahoy")
*/