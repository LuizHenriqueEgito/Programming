; Dado um os pesos e alturas calcule o IMC
; imc = (peso / altura²)
; peso baixo: IMC < 18,5
; peso ideal: 18,5 <= IMC <= 24,9
; acima do peso: IMC > 24,9
(require '[clojure.math :refer [pow]])

(defn array-pesos-alturas
  []
  [{:nome "Hommer" :peso 55 :altura 1.70}
   {:nome "Bart" :peso 120 :altura 1.55}
   {:nome "Lisa" :peso 89 :altura 1.88}
   {:nome "Megan" :peso 200 :altura 2.14}])

(defn- calcula-imc
  [peso altura]
  (let [imc (/ peso (pow altura 2))]
    (cond
      (< imc 18.5) "abaixo"
      (<= imc 24.9) "ideal"
      :else "acima")))

(defn processa-mapa
  [pesos-alturas]
  (map (fn [pessoa]
           {:nome (:nome pessoa)
            :imc (calcula-imc (:peso pessoa) (:altura pessoa))})
    pesos-alturas))

(println (array-pesos-alturas))
(println (processa-mapa (array-pesos-alturas)))