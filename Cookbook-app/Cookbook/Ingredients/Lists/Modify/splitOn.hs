import qualified Cookbook.Ingredients.Lists.Modify as Md
import qualified Cookbook.Essential.Common as Cm

import System.IO
import System.Environment
main = do
  (it:str) <- getArgs
  mapM_ putStrLn $ Md.splitOn (Cm.flt str) (head it)
