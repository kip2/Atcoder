(defun split-by-space (line)
  (let ((result '()))
    (with-input-from-string (in line)
      (loop for num = (read in nil)
            while num
            do (push num result)))
    (nreverse result)))

(defun solve (a b c)
  (if (< (+ (* a a) (* b b)) (* c c))
      (format t "Yes~%")
      (format t "No~%")))

(defun main ()
  (let* ((line (read-line))
         (nums (split-by-space line))
         (a (first nums))
         (b (second nums))
         (c (third nums)))
    (solve a b c)))

(main)
