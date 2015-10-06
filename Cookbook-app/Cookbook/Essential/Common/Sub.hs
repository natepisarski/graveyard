import qualified Cookbook.Essential.Common as Cm
import System.IO
import System.Environment

main = do
  (num:args) <- getArgs
  mapM_ putStrLn $ Cm.sub args 1
