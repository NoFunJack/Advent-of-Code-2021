rawData = {}
row_len = 0
for line in assert(io.open(arg[1], "r")):lines() do
  table.insert(rawData,line)
  row_len = #line
end

print("total data: "..#rawData)

function reduce(arr,det) 
  local filter = {}
  local row = 1
  local removed = 0

  while row <= row_len do
    local common = mostCommon(row,filter,removed)
    print("C: "..common)
    for i=1,#rawData do
      if (not filter[i]) and det(rawData[i]:sub(row,row), common) then
         filter[i] = true
         removed = removed + 1
         if removed >= #rawData-1 then return firstNotRemoved(filter) end
      end
    end 
    row = row + 1
    print("---"..row)
  end

  return firstNotRemoved(filter)
end

function mostCommon(idx,filter,numRemoved) 
  local ones = 0
  for i,line in filterIter(filter) do
    print(i..": "..line)
    ones = ones + line:sub(idx,idx)
  end
  print("ones"..ones)
  print("#f: "..numRemoved)
  return (ones >= (#rawData-numRemoved)/2) and "1" or "0"
end

function firstNotRemoved(filter) 
  for i = 1,#rawData do
    if not filter[i] then
      return rawData[i]
    end
  end
end

function filterIter(filter) 
  local raw_i = 0
  return function() 
    repeat raw_i = raw_i+1 until filter[raw_i] == nil 
    return rawData[raw_i] and raw_i, rawData[raw_i]
  end

end

local oxy = reduce(rawData,function(c,common) return c ~= common end )
print("oxy: "..oxy)
oxy = tonumber(oxy,2)
print("dez: "..oxy)

local scrub = reduce(rawData,function(c,common) return c == common end )
print("scrub: "..scrub)
scrub = tonumber(scrub,2)
print("scrub"..scrub)

print("soluton: "..(oxy*scrub))
