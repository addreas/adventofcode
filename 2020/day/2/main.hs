import Text.Regex.Posix

main = interact search

search input =  unwords [show count1, show count2] where
        cases = map parseline (lines input)
        count1 = length . filter evaluatecase1 $ cases
        count2 = length . filter evaluatecase2 $ cases


evaluatecase1 (lower, higher, char, text) = res where
    res = count >= lower && count <= higher
    count = length $ filter (==char) text

evaluatecase2 (lower, higher, char, text) = (a && not b) || (not a && b) where
    enumerate = zip [1..]
    enumerated = enumerate text
    a = case lookup lower enumerated of
            Just c -> c == char
            Nothing -> False
    b = case lookup higher enumerated of
            Just c -> c == char
            Nothing -> False
        

pattern = "([0-9]+)-([0-9]+) ([a-z]): ([a-z]*)"

parseline :: String -> (Int, Int, Char, String)
parseline line = (lower, higher, char, text) where
        [[_, l, h, [char], text]] = line =~ pattern
        lower = read l
        higher = read h

