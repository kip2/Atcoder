(defun read-line-as-string ()
  (read-line))

(defun solve (input)
  (concatenate 'string (subseq input 1) (subseq input 0 1)))

(defun main ()
  (let ((input (read-line-as-string)))
    (write-line (solve input))))

(main)
