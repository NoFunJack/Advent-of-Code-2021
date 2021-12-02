
local pos = {f=0, d=0}
local aim = 0

function readLine(line)
  addMov(line,"forward", function(x)  pos.f = pos.f + x; pos.d = pos.d + (aim*x) end )
  addMov(line,"down", function(x)  aim = aim + x end )
  addMov(line,"up", function(x)  aim = aim - x end )
end

function addMov(line,keyword,fx) 

  local _,_,v =  string.find(line,keyword.." (%d+)")
  if v then fx(v) end

end

for line in io.open(arg[1], "r"):lines() do
  readLine(line)
 end

 print("horizontal: "..pos.f.." depth: "..pos.d)
 print("Solution: "..pos.f*pos.d)


