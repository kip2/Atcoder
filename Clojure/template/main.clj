(defn solve []
;; Todo: implemented your code!
  nil)

(let [[a] (map #(Integer/parseInt %)
               (clojure.string/split (read-line) #" "))]
  (println (solve)))