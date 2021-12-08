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



if arg[2] == "part1" then
  crapFuelFunct = function(n) return n end
else 
  crapFuelFunct = function(n) return (n*n+n)//2 end
end

function calcCrapPos(pos)
   return {pos=pos,val=calcCost(pos,crap,crapFuelFunct)}
end

function step(stepSize,prev)
  if stepSize == 0 then return prev end

  local l = calcCrapPos(prev.pos-stepSize)
  local u = calcCrapPos(prev.pos+stepSize)

  stepSize = stepSize//2
  if l.val < prev.val then
    return step(stepSize,l)
  elseif u.val < prev.val then
    return step(stepSize,u)
  else
    return step(stepSize,prev)
  end
 
end

local middle = maxPos//2
local init = calcCrapPos(middle)
print("Solution: "..step(middle//2,init).val)

