#lang racket

; Basic list experiments
(define alist '())
(define anotherlist '(a b c))

(cons (car anotherlist) alist)
(cons (car (cdr anotherlist)) alist)
(display alist)
(newline)

(set! alist (cons (car anotherlist) alist))
(set! alist (cons (car (cdr anotherlist)) alist))

(display alist)
(newline)

(display (reverse alist))
(newline)


; car and cdr examples
(define mylist '(10 20 30 40 50))

(display mylist)
(newline)

(display (car mylist))
(newline)

(display (cdr mylist))
(newline)


; cadr/cadar/caadar examples
(define capitals
  '((maryland (annapolis))
    (pennsylvania (harrisburg))
    (delaware (dover))
    (virginia (richmond))))

(display (car (cdr capitals)))
(newline)

(display (cadr capitals))
(newline)

(display (cadar capitals))
(newline)

(display (caadar capitals))
(newline)


; 1. first, second, third, fourth, fifth
(define first
  (lambda (x) (car x)))

(define second
  (lambda (x) (cadr x)))

(define third
  (lambda (x) (caddr x)))

(define fourth
  (lambda (x) (cadddr x)))

(define fifth
  (lambda (x) (car (cddddr x))))

(define family '(josh sara erin sandy jon))

(display (third family))
(newline)

(display (fourth family))
(newline)

(display (fifth family))
(newline)


; 2. Counts the true values in a list
(define (truecount xs)
  (cond
    [(null? xs) 0]
    [(car xs) (+ 1 (truecount (cdr xs)))]
    [else (truecount (cdr xs))]))

(display (truecount '(#t #f #t #t #f)))
(newline)


; 3. Squares every value using map and lambda
(define (squarelist xs)
  (map (lambda (x) (* x x)) xs))

(display (squarelist '(1 2 3 4)))
(newline)


; 4. Keeps values greater than 100
(define (hundreds? xs)
  (filter (lambda (x) (> x 100)) xs))

(display (hundreds? '(50 101 200 75 300 100)))
(newline)


; 5. Prints the Collatz sequence recursively
(define (collatz n)
  (display n)
  (newline)

  (cond
    [(= n 1) (void)]
    [(even? n)
     (collatz (/ n 2))]
    [else
     (collatz (+ (* 3 n) 1))]))

(collatz 6)