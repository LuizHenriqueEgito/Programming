(defn my-seq []
  (let [colors ["red" "green" "blue"]]
    (println colors)
    (println (cons "yellow" colors))
    (println (conj colors "yellow"))
    (println (concat colors ["black" "white"]))
    (println (distinct [1 2 3 5 3 5 2 4]))
    (println (reverse colors))
    (println (first colors))
    (println (rest colors))
    (println (last colors))
    (println (sort [1 2 3 5 3 5 2 4]))))

(my-seq)