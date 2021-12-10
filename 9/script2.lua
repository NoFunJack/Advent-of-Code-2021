
local data = {};
local basinId = {}
local bCount = 0;
local score = 0;

function addLine(line) 
  table.insert(data,strToTable(line))
end

function strToTable(line)
  local re = {}
  for c in string.gmatch(line,"%d") do
    table.insert(re,tonumber(c,10))
    table.insert(basinId,{})
  end
  return re
end

function printData() 
  for i in ipairs(data) do
    for j,v in ipairs(data[i]) do
      if basinId[i] and basinId[i][j] then
        io.write("\27[38;5;"..(basinId[i][j]%250).."m"..v.."\27[38;5;7m")
      else 
        io.write(v)
      end
    end
    io.write("\n")
  end
end

function nextIter(x,y) 
  local dir = 0
  return function()
    dir = dir+1
    local a
    local b
    
    if dir == 1 then a=x;b=y-1 end
    if dir == 2 then a=x+1;b=y end
    if dir == 3 then a=x;b=y+1 end
    if dir == 4 then a=x-1;b=y end

    return a,b

  end
end

function changeBId(old,new) 
 for _,r in pairs(basinId) do
   for i,v in pairs(r) do
     if v == old then r[i] = new end 
   end
 end
end

function startSeach(x,y) 
  createB(x,y)
  flowdown(x,y)
end

function flowdown(x,y)
  local origin = data[x][y]
  for i,j in nextIter(x,y) do
    local n = data[i] and data[i][j]
    if n and n < origin then 
      if basinId[i][j]  then
        changeBId(basinId[i][j],bCount)
      else
        flowdown(i,j)
      end
      basinId[i][j] = bCount
    end
  end
end

function createB(x,y) 

  assert((basinId[x] and basinId[x][y]) == nil)
  bCount = bCount+1
  basinId[x][y] = bCount

end

function calcScore()
  counts = {}
  for i = 1,bCount do 
    local c = 0

   for _,r in pairs(basinId) do
     for _,v in pairs(r) do
       if v == i then c = c+1 end 
     end
   end
   table.insert(counts,c)
  end

  table.sort(counts)

  print("hightest:"..counts[#counts].."/"..counts[#counts-1].."/"..counts[#counts-2])
  print("score: "..counts[#counts]*counts[#counts-1]*counts[#counts-2])

end


for line in assert(io.open(arg[1], "r")):lines() do
  addLine(line)
  --io.write("\27[38;5;"..#data.."m"..line.."\27[40m")
end

-- work loop
for i in ipairs(data) do
  for j,v in ipairs(data[i]) do
    if v < 9 and basinId[i][j] == nil then 
      startSeach(i,j)
    end
  end
end

printData()
print("Bcount: "..bCount)
calcScore()
