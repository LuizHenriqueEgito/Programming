;; set: immutable, efficient, #{}, #{1 42 1.5 "PET" 'CAT}
;; map: key value: {:key1 value1, key2 value2}, immutable, efficient, #{:key1 "value1"}, #{1 42, 2 1.5, "PET" 'CAT}
;; vector: arrays, immutable, efficient, indexed by position [1 2 3 4 5] [1 "two" "three"] []
;; list: make up the code, immutable, efficient (1 2 3 4) (1 "two" 'three (1 2 3 4)) (defn foo [] (prinln "hello")) (foo)
;; EVERY THING IN CLOJURE IS AN LIST!