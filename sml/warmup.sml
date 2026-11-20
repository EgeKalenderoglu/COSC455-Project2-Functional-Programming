(* Searches the list recursively for the number 42 *)
fun hitchhiker [] = false
  | hitchhiker (42::_) = true
  | hitchhiker (_::xs) = hitchhiker xs;

(* Takes every second element from the list *)
fun everyother [] = []
  | everyother [_] = []
  | everyother (_::y::rest) = y :: everyother rest;

(* Checks if n has a divisor starting at d *)
fun hasDivisor (n, d) =
    if d * d > n then false
    else if n mod d = 0 then true
    else hasDivisor (n, d + 1);

(* Determines whether a number is prime *)
fun isPrime n =
    n >= 2 andalso not (hasDivisor (n, 2));

(* Recursively searches for two primes that add to n *)
fun findGoldbach (n, x) =
    if x >= n then NONE
    else if isPrime x andalso isPrime (n - x)
         then SOME (x, n - x)
         else findGoldbach (n, x + 1);

(* Prints a Goldbach pair for valid even numbers *)
fun goldbach n =
    if n <= 2 orelse n mod 2 <> 0 then
        print "Error: number must be even and greater than 2\n"
    else
        case findGoldbach (n, 2) of
            SOME (a, b) =>
                print (Int.toString a ^ " + " ^
                       Int.toString b ^ " = " ^
                       Int.toString n ^ "\n")
          | NONE =>
                print "No solution found\n";

hitchhiker [1, 5, 42, 10];
hitchhiker [1, 5, 10];

everyother [3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43];

goldbach 28;
goldbach 27;