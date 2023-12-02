; Numbers are allowed to overlap on the first and last letter.
; Instead of trying to deal with that in code, I simply made the replacement map weird.
(local number-table {:one :o1e
                     :two :t2o
                     :three :t3e
                     :four :f4r
                     :five :f5e
                     :six :s6x
                     :seven :s7n
                     :eight :e8t
                     :nine :n9e})

(lambda replace-digits [text]
  (accumulate [asdigits text k v (pairs number-table)]
    (asdigits:gsub k v)))

(lambda get-number [line]
  (let [(numbers _) (string.gsub (replace-digits line) :%D "")]
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
