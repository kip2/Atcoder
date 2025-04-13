(defn solve [a b c]
  (cond (< (+ (* a a) (* b b)) (* c c)) "Yes"
        :else "No"))

(let [[a b c] (map #(Integer/parseInt %)
                   (clojure.string/split (read-line) #" "))]
  (println (solve a b c)))