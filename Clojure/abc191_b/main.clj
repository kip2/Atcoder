(defn get-line
  "Get input data."
  []
  (read-line))

(defn split-words
  "Returns an array of input data split by spaces."
  []
  (clojure.string/split (get-line) #" "))

(defn solve [a x]
  (filter #(not (= % x)) a))

(defn main []
  (let [x (second (split-words))
        a (split-words)]
    (println (clojure.string/join " " (solve a x)))))

(main)