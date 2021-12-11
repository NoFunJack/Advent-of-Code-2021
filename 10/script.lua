local Stack = {}
local score = 0

local rating = {[")"]=3,["]"]=57,["}"]=1197,[">"]=25137}
local closer = {["("]=")",["["]="]",["{"]="}",["<"]=">"}   

function Stack:new ()
  local o = {["("]=")",["["]="]",["{"]="}",["<"]=">"}   
  setmetatable(o, self)
  self.__index = self
  return o
end

function Stack:print()
  for k,v in pairs(self) do
    io.write(k..": "..v.." ")
  end
  io.write("\n")
end

function addScore(c) 
  print("sc",c,rating[c])
  score = score+(rating[c] or 0)
end

function checkLine(line)
   validateChunk(line)
end

 function validateChunk(chunk,ender) 
   print(chunk)


   local sc = chunk:sub(1,1)
   local ec = closer[sc]
   if not ec then addScore(chunk:sub(1,1)); return end

   local i = 0
   local pos = 0
   for c,p in chunk:gmatch(".") do
     pos=pos+1

     --print(c,pos,i)
     if c == sc then i=i+1 end
     if c == ec then i=i-1 end
     -- chunk closes
     if i == 0 then 
       print("-"..sc..ec.."-")
       if pos > 2 and not validateChunk(chunk:sub(2,pos-1),ec) then return false end
       if #chunk > pos and not validateChunk(chunk:sub(pos+1,-1)) then return false end
       return
     end
   end

   --non closed start
   if #chunk > 1 then 
     if ender then 
       print("expected "..ec.." but found "..ender)
       addScore(ender)
       return false
     end
     print("nc")
     validateChunk(chunk:sub(2,-1))
   end

   return true

 end


for line in assert(io.open(arg[1], "r")):lines() do
  print(line.."+++++")
  checkLine(line)
end

print("score: "..score)

