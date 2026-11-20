#lang racket

; Chinese and English number words
(define chinese
  '(ling yi er san si wu liu qi ba jiu shi))

(define english
  '(zero one two three four five six seven eight nine ten))

; Finds the position of a word in a list
(define (find-index word xs index)
  (cond
    [(null? xs) #f]
    [(eq? word (car xs)) index]
    [else (find-index word (cdr xs) (+ index 1))]))

; Translates either a Chinese or English number word
(define (translate word)
  (let ([chinese-index (find-index word chinese 0)]
        [english-index (find-index word english 0)])
    (cond
      [chinese-index chinese-index]
      [english-index english-index]
      [else #f])))

; Removes words that cannot be translated
(define (translate-list xs)
  (filter number?
          (map translate xs)))

; Adds every number recursively
(define (add-list xs)
  (cond
    [(null? xs) 0]
    [else (+ (car xs)
             (add-list (cdr xs)))]))

; Multiplies every number recursively
(define (multiply-list xs)
  (cond
    [(null? xs) 1]
    [else (* (car xs)
             (multiply-list (cdr xs)))]))

; Displays a list separated by a given symbol
(define (display-expression xs separator)
  (cond
    [(null? xs) (void)]
    [(null? (cdr xs))
     (display (car xs))]
    [else
     (display (car xs))
     (display separator)
     (display-expression (cdr xs) separator)]))

; Main translation function
(define (go input)
  (define numbers (translate-list input))

  (display "Translation: ")
  (display-expression numbers " ")
  (newline)

  (display "Addition: ")
  (display-expression numbers " + ")
  (display " = ")
  (display (add-list numbers))
  (newline)

  (display "Multiplication: ")
  (display-expression numbers " * ")
  (display " = ")
  (display (multiply-list numbers))
  (newline))

; Tests
(go '(yi nine six ba))

(newline)

(go '(yi josh three si))