local file = io.open(arg[1], "r");
local arr = {}
for line in file:lines() do
   table.insert (arr, tonumber(line));
end

Buffer = {bufferPos = 0}

function Buffer:addToBuffer(n) 
  self[self.bufferPos] = n
  self.bufferPos = (self.bufferPos + 1) % 3
end

function Buffer:sum() 
  return self[0] + self[1] + self[2]
end

local sumInc = 0


for i,n in ipairs(arr) do

  if i > 3 then
    --io.write("\n",n)
    local oldVal = Buffer:sum()
    Buffer:addToBuffer(n)
    if Buffer:sum() > oldVal then
      --io.write(" inc")
      sumInc = sumInc + 1
    end
   else
    Buffer:addToBuffer(n)
  end
end

print("found "..sumInc.." incs")
