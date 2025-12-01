import Data.List
import qualified Data.Map.Strict as M

type Input = ([Int], [Int])

parse :: String -> Input
parse raw = unzip $ map parseLine (lines raw)
  where parseLine l = case words l of [x, y] -> (read x, read y)

part1 :: Input -> Int
part1 (l, r) = sum $ zipWith (\lv rv -> abs (lv - rv)) (sort l) (sort r)

part2 :: Input -> Int
part2 (l, r) =
  let frequency = M.fromListWith (+) $ map (, 1) r
  in sum $ map (\lv -> lv * M.findWithDefault 0 lv frequency) l
