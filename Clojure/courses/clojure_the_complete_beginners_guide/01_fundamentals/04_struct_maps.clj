(defn pets
  []
  (defstruct pet :PetType :PetName)
  (def my-pet (struct pet "dog" "Fido"))
  (println my-pet)
  (def my-other-pet (struct-map pet :PetName "Fifi" :PetType "Cat"))
  (println my-other-pet)
  (def my-new-pet (assoc my-pet :PetName "Max"))
  (println my-new-pet)
  (def my-new-other-pet (assoc my-other-pet :PetAge 10))
  (println my-new-other-pet)
  )
(pets)