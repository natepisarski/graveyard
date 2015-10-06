import qualified Cookbook.Ingredients.Lists.Modify as Md
import System.IO
import System.Environment
main = do
  (it:args) <- getArgs
  mapM_ putStrLn $ Md.rm args it
