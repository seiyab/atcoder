let ( $ ) f x = f x in
let h, w = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b)) in
let ss : string array = Array.init h (fun _ -> Stdlib.read_line ()) in
let cnt i j = 
	let di = [| -1; 1; 0; 0 |] in
	let dj = [| 0; 0; -1; 1 |] in
	let rec aux k acc = 
		if k = 4 then acc else
		let ni, nj = i + di.(k), j + dj.(k) in
		if ni < 0 || ni >= h || nj < 0 || nj >= w then aux (k + 1) acc else
		aux (k + 1) (if ss.(ni).[nj] = '#' then acc + 1 else acc)
	in aux 0 0
in
let map = List.init h (fun i -> List.init w (fun j -> (
	ss.(i).[j] = '.' || 
	cnt i j = 2 ||
	cnt i j = 4
))) in
let a = List.for_all (fun x -> x) $ List.flatten map in
Printf.printf "%s" $ if a then "Yes" else "No";;
