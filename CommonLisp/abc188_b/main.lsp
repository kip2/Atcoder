(defun read-line-split-by-space ()
  (let ((result '())
        (line (read-line)))
    (with-input-from-string (in line)
      (loop for num = (read in nil)
            while num
            do (push num result)))
    (nreverse result)))

(defun read-line-as-string ()
  (read-line))

(defun solve (a b)
  (let ((sum (reduce #'+ (mapcar #'* a b))))
    (cond ((zerop sum) "Yes")
          (t "No"))))

(defun main ()
  (let* ((_n (read-line-split-by-space))
         (a (read-line-split-by-space))
         (b (read-line-split-by-space)))
    (format t "~A~%" (solve a b))))

(main)
