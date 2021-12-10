
local last;
local score = 0;

function addLine(line) 
  cur = strToTable(line)
  for i,e in ipairs(cur) do
    v = e.v
    -- mark horizontal low points
    if isLower(v,cur[i-1]) and isLower(v,cur[i+1])  then
      cur[i].m = true 
    end
    if last then
      --print(last[i].v,v,last[i].v<v)
      if last[i].v < v then 
        -- unmark if last lower
        cur[i].m = false
      else last[i].m = false
      end
    end
  printData(last and last or {})
  print(string.rep(" ",i-1).."v")
  printData(cur)
  end
  -- score last row
  if last then addScore(last) end
  last = cur
end

function strToTable(line)
  local re = {}
  for c in string.gmatch(line,"%d") do
    table.insert(re,{v=tonumber(c,10),m=false})
  end
  return re
end

function printData(row) 
  for _,v in ipairs(row) do
    if v.m then io.write("\27[41m"..v.v.."\27[40m")
    else io.write(v.v)  end
  end
  io.write("\n")
end

function isLower(a,b)
  if not b then return true 
  else return a < b.v
  end
end

function addScore(row)
  for i in ipairs(row) do
    if row[i].m  then score = score + row[i].v+1 end
  end
end

assert(isLower(1,{v=1})==false)


for line in assert(io.open(arg[1], "r")):lines() do
  addLine(line)
  --printData(last)
  print("-"..score.."-")
end

-- Italian
addScore(last)
print("score:"..score)
