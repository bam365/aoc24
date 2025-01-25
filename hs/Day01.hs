import Data.List (sort)
import Text.Parsec
import Text.Parsec.String
import Text.Parsec.Char

pairParser :: Parser (Integer, Integer)
pairParser = do 
    i <- integer
    many1 space
    j <- integer
    return (i, j)
  where
    integer = read <$> many1 digit


part1 :: [(Integer, Integer)] -> Integer
part1 pairs = sum $ zipWith distance sortedLeft sortedRight
  where
    distance x y = abs (x - y)
    sortedLeft = sort . map fst $ pairs
    sortedRight = sort . map snd $ pairs

main = do
    input <- getContents
    let (Right parsedInput) = parse (many (pairParser <* endOfLine)) "input" input
    print $ part1 parsedInput 
    
