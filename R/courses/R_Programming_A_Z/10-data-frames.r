# Criando um dataframe
set.seed(42)

df <- data.frame(
    id= 1: 10,
    age = c(23, 35, 29, 41, 33, 27, 38, 45, 31, 26),
    wage = c(3000, 5200, 4100, 6800, 5900, 3600, 6200, 7500, 4800, 3400),
    approved = c(TRUE, FALSE, TRUE, TRUE, FALSE, TRUE, FALSE, TRUE, TRUE, FALSE)
)

print(df)

# usdando $
df[3, 3]
df[3, "age"]
# pega uma coluna
df$id
df$age
df$wage
df$approved

# operações básicas com dataframes
# cria nova coluna
df$nova_coluna <- df$age * df$wage
print(df)

# filtrando dados
filter <- df$age < 30
cat("\n")
print(df[filter & df$approved == F,])