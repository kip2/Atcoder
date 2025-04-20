(defun read-line-split-by-space (line)
  (let ((result '()))
    (with-input-from-string (in line)
      (loop for num = (read in nil)
            while num
            do (push num result)))
    (nreverse result)))

(defun read-line-as-string ()
  (read-line))

(defun solve ()
  nil)

(defun main ()
  (let* ((line (read-line-split-by-space)))
    (solve)))

(main)
