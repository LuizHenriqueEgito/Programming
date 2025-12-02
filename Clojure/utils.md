# Clojure
## Como dar nome as variáveis:
- Use `def` para variáveis e `defn` para funções;
- Clojure usa `kebab-case` usa hífens:
```clj
;; Variáveis comuns
(def lista-de-numeros [10 20 30 40])
;; funções
(defn somar-valores
  [numeros-lista]
  (reduce + numeros-lista))

(println "Soma dos valores:" (somar-valores lista-de-numeros))
```


## Origem do Clojure
- Dialeto do `LISP` → **List Processig**;
- Embasamento forte `matemático`;
- Clojure foi criada em 2007 pelo Rich Hickey;
- O nome da linguagem `(Clojure)` vem da palavra `Closure` e possui 3 Letras C (de C#), L (de Lisp) e J (de Java) dai .clj;

## Para rodar um arquivo Clojure (.clj)
```bash
clojure -M ./nome-do-seu-arquivo.clj
```

## Sintaxe Básica
- Em `Clojure` SEMPRE abra parenteses () e coloque a instrução;
- Clojure é uma linguagem dinamica permite redefinir as coisas;
- Clojure usa a notação polonesa (é como a HP12C)
- Semper feche na ultima linha todos os parenteses!
- O espaçamento é de 2 não de 4 como no Python
- O retorno é implicito nem que seja `nil` que é como null → ela retorna o último valor
```txt
OOP torna o código compreensível ao encapsular as partes móveis. FP (Programação Funcional)
torna o código compreensiível minimizando as partes móveis.
  - Michael Feathers
```
- Partes móveis → 