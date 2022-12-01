(* Question 2 *)

let rec weights_carried_per_elf ch = function
| [] -> []
| curr :: others ->
  (* Recursive case *)
  try
    let line = input_line ch in
    let weights = if (String.length line) == 0
      then 0::curr::others
      else ((int_of_string line)+curr)::others
    in weights_carried_per_elf ch weights
  (* Base case *)
  with End_of_file -> (curr::others)


let weights ch = weights_carried_per_elf ch [0]

(* Tail-recursive sum, with accumulator. *)
let rec take n taken = function
| [] -> taken
| x::xs -> if n == 0 
  then taken
  else take (n-1) (x::taken) xs

(* Tail-recursive sum, with accumulator. *)
let rec sum acc = function
| [] -> acc
| x::xs -> sum (acc + x) xs


let run filename = weights (open_in filename) 
  |> List.sort (fun x y -> (-1) * compare x y) (* Sort list in reverse order *)
  |> take 3 []
  |> sum 0
  |> string_of_int
  |> print_endline

(* Main *)
let _ =
  run "input.txt"