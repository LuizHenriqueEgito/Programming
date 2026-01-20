; DESAFIO: precisamos criar uma função que, dado um conjunto (vetor) de disciplinas e o semestre do discente, deve:
; - filtrar a lista para exibir disciplinas restantes (que sejam do semestre atual ou superior)
; - transformar o nome da disciplina para maiusculo e descartar demais informações
; - criar uma String concatenando o nome de todas as disciplinas filtradas
; Ex: [{:nome "DSA" :semestre 2} {:nome "Algoritmos" :semestre 1} {:nome "IA" :semestre 3}] 2 -> segundo semestre
; Saída Esperada: "DSA, IA"
(require '[clojure.string :refer [join upper-case]])

(defn disciplinas
  []
  [{:nome "Estrutura de Dados" :semestre 2}
   {:nome "Algoritmos" :semestre 1}
   {:nome "Inteligencia Artificial" :semestre 3}])

(defn nomes-disciplinas-restantes
  [disciplinas semestre-atual]
  (join ", " (map upper-case (map :nome (filter #(>= (:semestre %) semestre-atual) disciplinas)))))

(println (nomes-disciplinas-restantes (disciplinas) 2))

; refatorando com threading -> não é o multi threading convencional
; threading em clojure é uma forma de encadear funções
(defn nomes-disciplinas-restantes-threading
  [disciplinas semestre-atual]
  (->> disciplinas
       (filter #(>= (:semestre %) semestre-atual)) ; disciplinas entra aqui inplicitamente
       (map :nome) ; o resultado de cima entra como ultimo parametro implicitamente
       (map upper-case) ; o resultado de cima entra como ultimo parametro implicitamente
       (join ", "))) ; o resultado de cima entra como ultimo parametro implicitamente
       ; até finalizar todo o pipeline de transformação

(println (nomes-disciplinas-restantes-threading (disciplinas) 2))

; para mais detalhes: https://clojure.org/guides/threading_macros
; É possivel usar threading first ao invés do resultado de cima entrar como ultimo parametro agora ele entra como primeiro
; seu sinal é -> já o threading last é ->>