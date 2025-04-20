(defun read-line-as-string ()
  (read-line))

(defun solve (input)
  (let ((first-string (subseq input 0 1))
        (rest-string (subseq input 1)))
    (concatenate 'string rest-string first-string)))

(defun main ()
  (let ((input (read-line-as-string)))
    (write-line (solve input))))

(main)
