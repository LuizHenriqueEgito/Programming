(def buyer-account (ref 100))
(def merchant-account (ref 0))
(def prices {'pen 1 'notebook 5 'backpack 10})

(defn print-info
  []
  (println "\n[INFO]")
  (println "Buyer Account:" @buyer-account)
  (println "Merchant Account:" @merchant-account)
  (println "items:" @items))

(defn buy
  [item]
  (def item-price (get prices item))
  (if (<= item-price @buyer-account)
    (dosync
      (ref-set merchant-account (+ @merchant-account item-price))
      (ref-set buyer-account (- @buyer-account item-price))
      (def new-items (cons item @items))
      (ref-set items new-items))
    (println "Insufficient funds")) 
  (print-info)
  )

(buy 'pen)
(buy 'notebook)
(buy 'backpack)
(buy 'notebook)
