function inputnumbers() 
  return function()  
    return io.read("*number")
  end
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

for n in inputnumbers() do
  --  io.write(n," ", isInc(n) or "+" and "!+","\n")
  sumInc = sumInc + isInc(n)
end

print("found "..sumInc.." incs")
