(defn get-line
  "Get input data."
  []
  (read-line))

(defn split-words
  "Returns an array of input data split by spaces."
  []
  (clojure.string/split (get-line) #" "))

(defn print-list [lst]
  (println (clojure.string/join " " lst)))

(defn solve [a x]
  (filter #(not (= % x)) a))

(defn main []
  (let [x (second (split-words))
        a (split-words)]
    (print-list (solve a x))))

(main)