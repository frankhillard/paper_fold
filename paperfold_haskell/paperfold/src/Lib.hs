module Lib
    ( testPaperFold1,
      testPaperFold2
    ) where

import Data.Char  
import Prelude 

testPaperFold1 :: IO ()
testPaperFold1 = --do
    -- print(fusionCell ['a','b'] ['c'])
    -- print(fusionCell ['a','b'] ['c','d'])
    -- print(splitlines 2 [[['A'],['B'],['C'],['D']],[['E'],['F'],['G'],['H']],[['I'],['J'],['K'],['L']]])
    -- print(foldLineUp 2 [[['A'],['B'],['C'],['D']],[['E'],['F'],['G'],['H']],[['I'],['J'],['K'],['L']]])
    -- print(foldLineUp 1 [["EI","FJ","GK","HL"],["A","B","C","D"]])
    -- print(fold 2 H [[['A'],['B'],['C'],['D']],[['E'],['F'],['G'],['H']],[['I'],['J'],['K'],['L']]])
    -- print(fold 2 B [[['A'],['B'],['C'],['D']],[['E'],['F'],['G'],['H']],[['I'],['J'],['K'],['L']]])
    -- print(transpose [[['A'],['B'],['C'],['D']],[['E'],['F'],['G'],['H']],[['I'],['J'],['K'],['L']]])
    -- print(transpose [["A","E","I"],["B","F","J"],["C","G","K"],["D","H","L"]])
    -- print(fold 1 G [[['A'],['B'],['C'],['D']],[['E'],['F'],['G'],['H']],[['I'],['J'],['K'],['L']]])
    -- print(fold 1 D [[['A'],['B'],['C'],['D']],[['E'],['F'],['G'],['H']],[['I'],['J'],['K'],['L']]])
    let
        init_paper = [[['A'],['B'],['C'],['D']],[['E'],['F'],['G'],['H']]]
        folding_1 = fold 2 D init_paper
        folding_2 = fold 1 H folding_1
        folding_3 = fold 1 G folding_2
    in
        do print(folding_3)


testPaperFold2 :: IO ()
testPaperFold2 = --do
    let
        init_paper = [[['A'],['B'],['C'],['D']],[['E'],['F'],['G'],['H']]]
        folding_1 = fold 1 G init_paper
        folding_2 = fold 1 G folding_1
        folding_3 = fold 1 G folding_2
        folding_4 = fold 1 H folding_3
    in
        do print(folding_4)


fusionCell :: [a] -> [a] -> [a]
fusionCell [] [] = []
fusionCell [x] [] = [x]
fusionCell [x] [y] = x:[y]
fusionCell [] [y] = [y]
fusionCell (x:xs) [] = reverse xs ++ [x]
fusionCell [] (y:ys) = y:ys
fusionCell (x:xs) (y:ys) = reverse xs ++ [x] ++ [y] ++ ys

splitlines :: Int -> [a] -> ([a], [a])
splitlines i xs = (take i xs, drop (i) xs)

fusionLine :: [[[a]]] -> [[[a]]] -> [[[a]]]
fusionLine as [] = as
fusionLine [] bs = bs
fusionLine (a:as) (b:bs) = 
    -- let cartesian_product = fusionCell <$> a <*> b
    zipWith fusionCell a b : fusionLine as bs 

foldLineUp :: Int -> [[[a]]] -> [[[a]]]
foldLineUp i xs = 
    let (ups, bot) = splitlines i xs
        rev_lines = reverse ups
        result = fusionLine rev_lines bot
    in result

data Part = H | B | G | D deriving (Show, Enum, Eq)

transpose:: [[a]]->[[a]]
transpose ([]:_) = []
transpose x = (map head x) : transpose (map tail x)


fold :: Int -> Part -> [[[a]]] -> [[[a]]]
fold i H paper = foldLineUp i paper
fold i B paper = 
    let rev_paper = reverse paper
        result = reverse (foldLineUp ((length rev_paper) - i) rev_paper)
    in result
fold i G paper = transpose (foldLineUp i (transpose paper))
fold i D paper = 
        let transposed_paper = transpose paper
            rev_paper = reverse transposed_paper
            result = reverse (foldLineUp ((length rev_paper) - i) rev_paper)
    in transpose result
