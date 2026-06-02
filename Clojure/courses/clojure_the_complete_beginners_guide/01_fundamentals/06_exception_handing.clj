(defn ex-handling
  []
  (try 
    (inc x)
    (catch ClassCastException e (println "Caught exception:" (.getMessage e)))
    (catch Exception e (println "Caught generic exception"))
    (finaly (println "Clenup and move on"))))
(ex-handling "hello")