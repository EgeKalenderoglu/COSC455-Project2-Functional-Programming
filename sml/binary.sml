(* Reverses a list recursively *)
fun reverseList xs =
    let
        fun revHelper ([], acc) = acc
          | revHelper (x::rest, acc) =
                revHelper (rest, x::acc)
    in
        revHelper (xs, [])
    end;

(* Converts binary integers to Boolean values *)
fun toBool [] = []
  | toBool (0::rest) = false :: toBool rest
  | toBool (1::rest) = true :: toBool rest
  | toBool (_::_) = raise Fail "Invalid binary digit";

(* Converts Boolean values back to binary integers *)
fun toInt [] = []
  | toInt (false::rest) = 0 :: toInt rest
  | toInt (true::rest) = 1 :: toInt rest;

(* Finishes addition when one list is longer *)
fun finishBinaryAdd ([], false) = []
  | finishBinaryAdd ([], true) = [true]
  | finishBinaryAdd (x::xs, carry) =
        case (x, carry) of
            (false, false) =>
                false :: finishBinaryAdd(xs, false)
          | (true, false) =>
                true :: finishBinaryAdd(xs, false)
          | (false, true) =>
                true :: finishBinaryAdd(xs, false)
          | (true, true) =>
                false :: finishBinaryAdd(xs, true);

(* Adds two reversed Boolean bit lists *)
fun doBinaryAddition ([], [], carry) =
        if carry then [true] else []
  | doBinaryAddition (xs, [], carry) =
        finishBinaryAdd(xs, carry)
  | doBinaryAddition ([], ys, carry) =
        finishBinaryAdd(ys, carry)
  | doBinaryAddition (x::xs, y::ys, carry) =
        case (x, y, carry) of
            (false, false, false) =>
                false :: doBinaryAddition(xs, ys, false)
          | (false, false, true) =>
                true :: doBinaryAddition(xs, ys, false)
          | (false, true, false) =>
                true :: doBinaryAddition(xs, ys, false)
          | (true, false, false) =>
                true :: doBinaryAddition(xs, ys, false)
          | (true, true, false) =>
                false :: doBinaryAddition(xs, ys, true)
          | (true, false, true) =>
                false :: doBinaryAddition(xs, ys, true)
          | (false, true, true) =>
                false :: doBinaryAddition(xs, ys, true)
          | (true, true, true) =>
                true :: doBinaryAddition(xs, ys, true);

(* Main binary addition function *)
fun binaryAddition (a, b) =
    let
        val aBool = reverseList(toBool a)
        val bBool = reverseList(toBool b)
        val result =
            doBinaryAddition(aBool, bBool, false)
    in
        toInt(reverseList result)
    end;

(* Pads reversed lists with zeros until they have equal length *)
fun padBinary ([], []) = ([], [])
  | padBinary (x::xs, y::ys) =
        let
            val (px, py) = padBinary(xs, ys)
        in
            (x::px, y::py)
        end
  | padBinary (x::xs, []) =
        let
            val (px, py) = padBinary(xs, [])
        in
            (x::px, false::py)
        end
  | padBinary ([], y::ys) =
        let
            val (px, py) = padBinary([], ys)
        in
            (false::px, y::py)
        end;

(* Flips every bit *)
fun invertBits [] = []
  | invertBits (false::xs) =
        true :: invertBits xs
  | invertBits (true::xs) =
        false :: invertBits xs;

(* Adds one while keeping the same bit width *)
fun addOneFixed [] = []
  | addOneFixed (false::xs) =
        true :: xs
  | addOneFixed (true::xs) =
        false :: addOneFixed xs;

(* Creates the two's complement of a binary list *)
fun twosComplement xs =
    addOneFixed(invertBits xs);

(* Keeps only the original number of bits *)
fun keepWidth (_, []) = []
  | keepWidth (x::xs, _::ys) =
        x :: keepWidth(xs, ys)
  | keepWidth ([], _::ys) =
        false :: keepWidth([], ys);

(* Removes unnecessary zeros from the front *)
fun removeLeadingZeros [] = [0]
  | removeLeadingZeros [0] = [0]
  | removeLeadingZeros (0::xs) =
        removeLeadingZeros xs
  | removeLeadingZeros xs = xs;

(* Binary subtraction uses two's complement and binary addition *)
fun binarySubtraction (a, b) =
    let
        val aRev = reverseList(toBool a)
        val bRev = reverseList(toBool b)

        val (aPadded, bPadded) =
            padBinary(aRev, bRev)

        val negativeB =
            twosComplement bPadded

        val fullResult =
            doBinaryAddition(
                aPadded,
                negativeB,
                false
            )

        val result =
            keepWidth(fullResult, aPadded)
    in
        removeLeadingZeros(
            toInt(reverseList result)
        )
    end;


(* Addition tests *)
binaryAddition ([1,1,1,1,0], [1,0,1,1]);

binaryAddition (
    [1,0,0,1,1,0,1],
    [0,0,1,0,0,1,0]
);

binaryAddition (
    [1,0,0,1,0,0,1],
    [0,0,1,1,0,0,1]
);


(* Subtraction tests *)
binarySubtraction (
    [1,0,1,0,0,1],
    [1,0,1,1]
);

binarySubtraction (
    [1,0,1,1,1,1,1],
    [0,0,1,0,0,1,0]
);