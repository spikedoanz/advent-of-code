(defn read-file [filepath]
  (with-open [reader (java.io.BufferedReader. (java.io.FileReader. filepath))]
    (doall (line-seq reader))))

(defn fuel [x]
  (- (quot x 3) 2))

(defn iterone [acc, arr]
  (if (seq arr)
  (let [x (Integer/parseInt (first arr)) 
        xs (rest arr)]
    (iterone (+ acc (fuel x)) xs))
    acc)) 


(defn process [x]
  (if (< x 6) 0
  (- (quot x 3) 2)))

(defn calc [acc x]
  (if (= x 0) acc
    (calc (+ acc (process x)) (process x))))

(defn iter [acc arr] 
  (if (seq arr)
  (let [x (Integer/parseInt (first arr)) xs (rest arr)]
        (iter (+ acc (calc 0 x)) 
        xs))
    acc))

(println (format "Part 1: %s" (iterone 0 (read-file "inputday1.txt"))))
(println (format "Part 2: %s" (iter 0 (read-file "inputday1.txt"))))