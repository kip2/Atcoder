(defn solve [a b]
  (let [sum (reduce + (map * a b))]
    (if (zero? sum) "Yes"
        "No")))

(defn read-input []
  (let [_ (Integer/parseInt (read-line))
        a (mapv #(Integer/parseInt %) (clojure.string/split (read-line) #" "))
        b (mapv #(Integer/parseInt %) (clojure.string/split (read-line) #" "))]
    [a b]))

(defn main []
  (let [[a b] (read-input)]
    (println (solve a b))))

(main)

