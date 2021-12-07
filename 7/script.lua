crap = {}
maxPos = 0
do
  local f = io.open(arg[1],"rb"):read("*all")
  for word in string.gmatch(f, '([^,]+)') do
    local c = tonumber(word,10)
    table.insert(crap,c)
    maxPos = math.max(maxPos,c)
  end
end

function gaus(n) 
  return (n*n+n)//2
end

assert(gaus(0)==0)
assert(gaus(1)==1)
assert(gaus(3)==6)

function calcCost(hole,arr,costf)
  local sum = 0
  for _,c in ipairs(arr) do
   sum = sum + costf(math.abs(c-hole))
  end
  return sum
end

minCost = math.maxinteger

if arg[2] == "part1" then
  crapFuelFunct = function(n) return n end
else 
  crapFuelFunct = function(n) return (n*n+n)//2 end
end

for pos = 0,maxPos do
  local cost = calcCost(pos,crap, crapFuelFunct)
  minCost = math.min(cost,minCost)
end

print("Solution: "..minCost)

