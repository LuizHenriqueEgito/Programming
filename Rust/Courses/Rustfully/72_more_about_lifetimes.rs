/*
Se o programador não escreveu lifetimes, eu vou tentar inferir assim:
1. Cada referencia de entrada ganha um lifetime
2. Se existe apenas um parâmetro por referência, o retorno usa o mesmo lifetime
3. Se existe &self ou &mut self o lifetime do retorno é o de self
*/