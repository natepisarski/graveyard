import qualified Cookbook.Ingredients.Lists.Modify as Md
import System.IO
import System.Environment
main = do
  args <- getArgs
  mapM_ putStrLn $ Md.rev args
