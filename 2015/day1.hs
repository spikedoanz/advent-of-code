import Text.Printf

countbr :: Char -> Char -> String -> Int
countbr br1 br2 str = count1 - count2
    where
    count1 = length $ filter (==br1) str
    count2 = length $ filter (==br2) str

iterateC :: Int -> Int -> String -> Int
iterateC count inp [] = -1 
iterateC count inp (x:xs) = do
    let ret | x == '(' = inp + 1
            | x == ')' = inp - 1
            | otherwise= inp
    let newCount = count + 1
    if ret < 0 then newCount 
    else iterateC newCount ret xs 


main :: IO ()
main = do
    content <- readFile "inputday1.txt"
    let count = countbr '(' ')' content 
    printf "Part 1: %d\n" count
    let floor = iterateC 0 0 content
    printf "Part 2: %d\n" floor 