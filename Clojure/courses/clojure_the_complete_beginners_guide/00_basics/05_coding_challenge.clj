;; Function that gives us the age of a pet in human years
;; dog: x7, cat: x5, fish: x10

(defn pet-to-human-age
  "This function returns the age of a pet in human years"
  [x]
  (def pet-store {'dog 7, 'cat 5, 'fish 10})
  (get pet-store x))

(defn age
  "This function returns the age of a pet"
  [pet-name pet-type pet-age]
  (def ration (pet-to-human-age pet-type))
  (prinln pet-name "is" (* ratio pet-age) "years old in human years"))

(age "Fido" 'dog 4)
(age "Fifo" 'cat 2)
(age "Nemo" 'fish 10)
