import qualified Cookbook.Essential.Continuous as Ct
import System.IO
import System.Environment

main = do
  (it:args) <- getArgs
  mapM_ putStrLn $ Ct.before args it
