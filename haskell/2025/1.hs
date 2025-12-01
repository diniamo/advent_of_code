import Debug.Trace

type Input = [Int]

parse :: String -> Input
parse raw = map parseLine $ lines raw
  where
    parseLine :: String -> Int
    parseLine ('L':n) = negate (read n)
    parseLine ('R':n) = read n

part1 :: Input -> Int
part1 ts = snd (foldl pred (50, 0) ts)
  where
    pred (v, c) t =
      let turned = (v + t) `mod` 100
      in (turned, if turned == 0 then c + 1 else c)

part2 :: Input -> Int
part2 ts = snd (foldl pred (50, 0) ts)
  where
    pred (v, c) t
      | v == (negate t) = (0, c + 1)
      | v == 0 && t < 0 = let (cycles, turned) = t `divMod` 100
                          in  (turned, c + (negate cycles) - 1)
      | otherwise = let (cycles, turned) = (v + t) `divMod` 100
                    in  (turned, c + (abs cycles))
