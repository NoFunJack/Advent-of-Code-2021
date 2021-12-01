local file = io.open(arg[1], "r");
local arr = {}
for line in file:lines() do
   table.insert (arr, tonumber(line));
 end

function buildInc()
  local last
  return function(n)  
    local re
    if last and last < n then 
      re =  1
    else 
      re = 0
    end
    last = n
    return re
  end
end

local last;
local isInc = buildInc()
local sumInc = 0

for _,n in ipairs(arr) do
  --  io.write(n," ", isInc(n) or "+" and "!+","\n")
  sumInc = sumInc + isInc(n)
end

print("found "..sumInc.." incs")
