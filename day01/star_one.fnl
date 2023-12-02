(lambda get-number [line]
  (let [(numbers _) (line:gsub :%D "")]
    (tonumber (.. (numbers:sub 1 1) (numbers:sub (- 1) (- 1))))))

(lambda calculate_sum [content]
  (accumulate [sum 0 num (content:gmatch "[^\r?\n]+")]
    (+ sum (get-number num))))

(lambda first-solution [filename]
  (match (io.open filename)
    f (do (print (calculate_sum (f:read :*all))) (f:close))
    (nil err-msg) (print "Could not open file:" err-msg)))

(lambda main [args]
  (match args
    [f] (first-solution f)
    _ (print "Please provide an input file as argument")))

(main arg)
