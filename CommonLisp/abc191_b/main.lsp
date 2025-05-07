(defun read-line-split-by-space (line)
  (let ((result '()))
    (with-input-from-string (in line)
      (loop for num = (read in nil)
            while num
            do (push num result)))
    (nreverse result)))

(defun read-line-as-string ()
  (read-line))

(defun println (x)
  (format t "~a~%" x))

(defun println-list (lst)
  (format t "~{~a~^ ~}~%" lst))

(defun solve (x a)
  (remove-if (lambda (b) (eql b x)) a))

(defun main ()
  (let* ((numbers (read-line-split-by-space (read-line)))
         (x (cadr numbers))
         (a (read-line-split-by-space (read-line))))
    (println-list (solve x a))))

(main)
