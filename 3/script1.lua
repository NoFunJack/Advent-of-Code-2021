local mostCommonBits = {}
local readLines = 0

function readLine(line)
  for i=1,#line do
    bit = line:sub(i,i)
    mostCommonBits[i] = mostCommonBits[i] or 0 
    mostCommonBits[i] = mostCommonBits[i] + bit
  end
  readLines = readLines + 1
end

function determ(value) 
  return value > readLines/2 
end


for line in assert(io.open(arg[1], "r")):lines() do
  bitl = #line
  readLine(line)
end

local gamma = 0
local mask = 1

-- reverse becase i = 1 is the most valuable bit
for i = #mostCommonBits,1,-1 do
 if determ(mostCommonBits[i]) then
   gamma = gamma | mask
 end
 mask = mask << 1
end

local beta = ~gamma % (1 << #mostCommonBits);

print("gamma: "..gamma)
print("beta: "..beta)

print("solution: "..gamma*beta)
