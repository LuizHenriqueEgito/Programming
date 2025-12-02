# GO
## Como dar nome as variáveis:
- GO usa `camelCase` para *variáveis/funções* exportado do pacote `PascalCase` não exportado do pacote `camelCase` 
minha-var

## Como rodar um código em GO
``` bash
go run nome_arquivo.go
```

## Como compilar um código em GO
``` bash
go build nome_arquivo.go

# compilando com nome personalizado
go build -o meu_programa.exe nome_arquivo.go
```

## Como rodar um arquivo compilado
``` bash
./nome_arquivo  # nome do arquivo gerado ao ser compilado
```