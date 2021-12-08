function countUniques(line)
  local count = 0
  -- 1
  count = count + findNLetterInputs(line,2)
  -- 4
  count = count + findNLetterInputs(line,4)
  -- 7
  count = count + findNLetterInputs(line,3)
  -- 8
  count = count + findNLetterInputs(line,7)
  return count
end

function findNLetterInputs(line,n)
  local line = line:gsub(".*|","") 
  local count= 0
  for word in string.gmatch(line, "%a+") do
    if #word == n then count=count+1 end
  end
  return count
end

function countMatches(line,matchstr) 
  --print("countinput:"..line)
end

--read data
data = {}
Count = 0
for line in assert(io.open(arg[1], "r")):lines() do
  Count = Count + countUniques(line)
end
print(Count)
