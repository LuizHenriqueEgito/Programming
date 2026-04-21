/*
Cargo Workspace:
    Conmjunto de pacotes inter-relacionados

Package:
    Pode conter vários 'crates binários' e opcionalmente apenas um 'crate biblioteca'.
    Decrito pelo arquivo Cargo.toml

Crate:
    Uma árvore de moódulos que produzem um executável ou uma biblioteca.

Módulo:
    Uma parte do programa cujos copmponentes (funções, structs, enums, constantes, etc) cooperam
    para o mesmo propósito (forte coesão).

Tudo que for criado dentro do um Módulo é privado você precisa deixar publico usando
pub antes de funções, structs, etc

- crate é o modulo raiz
- super é o modulo de "fora"
==================================================================================================
📁Crate root 
📄 (main.rs)
    - main()
    - outra_funcao()
📄 (front_of_house.rs)
    - init_front_house()
    - chamadas_caminhos()
📁 front_of_house
    📄 (hosting.rs)
        - add_to_waitlist()
        - seat_at_table()
    
    📄 (serving.rs)
        -take_order()
        - server_order()
        - take_take_payment()
*/