local arr = {}
for line in io.open(arg[1], "r"):lines() do
   table.insert (arr, tonumber(line));
end

function getSumRange(startIdx) 
  local sum = 0
  for i = startIdx, startIdx+2 do
    sum = sum + arr[i]
  end
  return sum;
end

local sumInc = 0
for i=1, #arr-3 do
 if getSumRange(i) < getSumRange(i+1) then
   sumInc = sumInc + 1
 end
end

print("found "..sumInc.." incs")
