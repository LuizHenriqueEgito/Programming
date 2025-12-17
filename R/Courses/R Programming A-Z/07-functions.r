#?funcao → mostra a documentação
rnorm(n=5, sd=8)

x <- c(1: 5)
rep(x, times=6)

print('Olá muno', x)

# use function para criar uma função

foobar <- function(param1, param2) {
    resultado <- param1 + param2
    resultado <- resultado ** 2
    resultado  # em R a ultima expressão é retornada automaticamente
}

meu_retorno <- foobar(2, 3)
print(meu_retorno)
