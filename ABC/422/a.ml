let () =
  let i, j = Scanf.scanf "%d-%d" (fun a b -> (a, b)) in
  let k, l = if 8 = j then (i + 1, 1) else (i, j + 1) in
  Printf.printf "%d-%d\n" k l
