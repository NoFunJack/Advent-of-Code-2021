local file = assert(io.open(arg[1], "r"))

-- read numbers
local callsStr  = file:read("l")
local calls = {}
callsStr:gsub("%d+",function(s) calls[#calls+1]=tonumber(s) end)
print("calls read: "..#calls)
-- discard first blank line
file:read("l")

-- create boards
local boards = {} 
local b = {}

for line in file:lines() do
  if line ~= "" then
    line:gsub("%d+", function(s)
      b[#b+1] = tonumber(s)
    end)
  else
    assert(#b==25)
    boards[#boards+1] = b
    b = {}
  end
end

-- last one at file end
assert(#b==25)
boards[#boards+1] = b

print("boards found:"..#boards)
file:close()

-- board methods
function printBoards() 
  allBoards(printBoard)
end

function printBoard(b) 
    print(
      string.format(
      string.rep("%02d %02d %02d %02d %02d",5,"\n")
      ,table.unpack(b))
    )
    print("")
end

function markNumber(num)
  allBoards(function(b)
    for i,v in ipairs(b) do
      b[i] = (v == num) and -1 or v 
    end
  end)
end

function findWinner()
  return allBoards(isWinner)
end

function isWinner(b) 
  for r=1,5 do
    -- horizontal 
    local sumH = 0
    local sumV = 0
    for i=0,4 do
      sumH = sumH + b[(r-1)+(i+1)]
      sumV = sumV + b[r+i*5]
    end
    if sumH == -5 or sumV == -5 then return b end
  end
end

function score(b) 
  local s= 0;
  for _,v in ipairs(b) do
    if v ~= -1 then s = s + v end
  end
  return s
end

function allBoards(f) 
  for _,b in ipairs(boards) do
    local result = f(b) 
    if result then return result end
  end
end

local numIdx = 0
function drawNumber() 
  numIdx = numIdx+1
  local num = calls[numIdx]
--  print("**draw number: "..num.."**")
  markNumber(num)
end

--printBoards()
repeat  
  drawNumber()
--  printBoards()
  winner = findWinner()
until(winner)

print("Winning board:\n")
printBoard(winner)
print("Score: "..score(winner))
print("last number: "..calls[numIdx])
print("solution: "..score(winner)*calls[numIdx])
