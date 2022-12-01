(* Question 1 *)

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

let rec listmax prev_max = function 
  | [] -> raise End_of_file
  | x::[] -> max x prev_max
  | x::xs -> listmax (max x prev_max) xs

  (** Count how many times the integers from the channel increased, compared to previous. 
    Recurses over channel contents. *)
let max_weight ch = listmax 0 (weights_carried_per_elf ch [0])

let run filename = max_weight (open_in filename) |> string_of_int |> print_endline

(* Main *)
let _ =
  run "input.txt"