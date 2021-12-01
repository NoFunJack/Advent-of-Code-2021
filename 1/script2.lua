function inputnumbers() 
  local i = 0
  return function()  
    i = i + 1
    local num =  io.read("*number")
    return  num and i, num 
  end
end


Buffer = {}
function Buffer:new ()
  o = {bufferPos = 0}   -- create object if user does not provide one
  setmetatable(o, self)
  self.__index = self
  return o
end

function Buffer:addToBuffer(n) 
  self[self.bufferPos] = n
  self.bufferPos = self.bufferPos + 1
end

function Buffer:sum() 
  return self[0] + self[1] + self[2]
end

function buildInc()
  local last 
  local bufferPos = 0


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

local isInc = buildInc()
local sumInc = 0
local buffList = {}

function feedBuffers(i,n) 
  buffList[i] = Buffer:new()

  incBuffNr(i-2,n)
  incBuffNr(i-1,n)
  incBuffNr(i-0,n)
  
  --del buffer
  buffList[i-3] = nil
end

function incBuffNr(idx,n) 
  if buffList[idx] then
    buffList[idx]:addToBuffer(n)
  end
end

for i,n in inputnumbers() do

  feedBuffers(i,n)

  if buffList[i-2] then
    local incVal = isInc(buffList[i-2]:sum())
    --io.write(n," ", incVal ,"\n")
    sumInc = sumInc + incVal
  end
end

print("found "..sumInc.." incs")
