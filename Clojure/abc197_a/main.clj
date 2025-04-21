(defn solve [s]
  (let [head (subs s 0 1)
        tail (subs s 1)]
    (str tail head)))

(defn main []
  (let [line (read-line)]
    (println (solve line))))

(main)