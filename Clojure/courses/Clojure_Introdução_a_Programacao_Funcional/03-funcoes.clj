(defn imprimir-ola [] (println "Olá mundo"))  ; toda função em clojure recebe um vetor, mesmo que seja VAZIO
(imprimir-ola)

(defn parabenizar [nome] (println "Parabéns" nome))
(parabenizar "Luiz")

(defn parabenizar
  [nome]
  (println "Seja bem vindo")
  (println "Parabéns" nome)
  (println "volte smepre"))  ; semper feche na ultima linha todos os parenteses!

(parabenizar "Nuna")

(defn valor-multa-fn
  [valor taxa]
  (* valor taxa))

(def valor-da-multa (valor-multa-fn 100 0.1))
(println valor-da-multa)