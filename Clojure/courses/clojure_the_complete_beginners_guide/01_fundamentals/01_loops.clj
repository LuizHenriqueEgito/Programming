(defn my-loop
  []
  (println "LOOP")
  (loop [x 0]
    (when (< x 10)
      (println x)
      (recur (inc x)))))
(my-loop)


(defn do-times
  []
  (println "Do-TIMES")
  (dotimes [x 10]
    (println x)))
(do-times)

(defn while-do
  [count]
  (println "while DO")
  (def x (atom 0))
  (while (< @x count)
    (do 
      (println @x)
      (swap! x inc))))
(while-do 10)

(defn do-seq
  [seq]
  (println "Do SEQ")
  (doseq [x seq])
    (println (inc x)))
(do-seq [4 3 2 1 4 6 7])