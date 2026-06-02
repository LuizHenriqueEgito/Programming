(defstruct coupon :Name :Discount)

(def valid-coupon
  (struct coupon "20Percent" 0.8))

(defn is-code-valid
  [code]
  (= (:Name valid-coupon) code))

(defn get-car-prices
  [budget code]

  (let [cars {"bmw" 60000
              "ferrari" 100000
              "fiat" 20000}]

    (if (is-code-valid code)

      (do
        (println "The code is valid")

        (let [discount (:Discount valid-coupon)]

          (doseq [[car-type price] cars]
            (let [price-discount (* price discount)]
              (when (<= price-discount budget)
                (println "The" car-type "costs" price-discount))))))

      (do
        (println "The code is invalid")

        (doseq [[car-type price] cars]
          (when (<= price budget)
            (println "The" car-type "costs" price)))))))

(get-car-prices 50000 "20Percent")