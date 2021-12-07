crap = {}
maxPos = 0
do
  local f = io.open(arg[1],"rb"):read("*all")
  for word in string.gmatch(f, '([^,]+)') do
    local c = tonumber(word,10)
    crap[c] = crap[c] and crap[c] + 1 or 1
    maxPos = math.max(maxPos,c)
  end
end

function calcCost(hole,arr,costf)
  local sum = 0
  for p,c in pairs(arr) do
   sum = sum + (c*costf(math.abs(p-hole)))
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

