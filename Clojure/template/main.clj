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

(defn solve []
  ;; Todo: implemented your code!
  nil)

(defn main []
  ;; Todo: implemented your code!
  (println (solve)))

(main)