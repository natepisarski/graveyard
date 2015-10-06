import qualified Cookbook.Essential.Common as Cm
import System.IO
import System.Environment

main = do
  (it:args) <- getArgs
  mapM_ putStrLn $ map show$  Cm.positions args it
