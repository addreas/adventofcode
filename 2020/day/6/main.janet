(def things (string/split "\n\n" (file/read stdin :all)))

(print (->> things
          (map (fn [x] (->> x
                        (string/replace-all "\n" "")
                        distinct
                        length)))
          sum))

(print (->> things
          (map (fn [x] (->> x
                        (string/split "\n")
                        (map string/bytes)
                        (reduce2 (fn [a b] (filter (fn [i] (index-of i b)) a)))
                        distinct
                        length)))
          sum
          (+ 5))) # somehow missing 5
