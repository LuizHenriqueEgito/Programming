(defn destruct
  []
  (def my-vec [1 2 3 4])
  (let [[a b c] my-vec] (println a b c))
  (let [[a b & rest] my-vec] (println a b rest))

  (def my-map {'name "John" 'lastname "Smith"})
  (let [{a 'name b 'lastname} my-map] (println a b))
  (let [{a 'name b 'lastname c 'noname} my-map] (println a b))
  )

(destruct)