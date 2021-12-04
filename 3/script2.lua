rawData = {}
row_len = 0
for line in assert(io.open(arg[1], "r")):lines() do
  table.insert(rawData,tonumber(line,2))
  row_len = #line
end

print("total data: "..#rawData)

function reduce(arr,det) 
  local filter = {}
  local numRemoved = 0

  for row= 1,row_len do
    local common = mostCommon(row,filter,numRemoved)
    for i,line in filterIter(filter) do
      if (not filter[i]) and det(bitOnPos(line,row), common) then
         filter[i] = true
         numRemoved = numRemoved + 1
         if numRemoved >= #rawData-1 then return firstNotRemoved(filter) end
      end
    end 
  end

  error("list could not be reduced to one")
end

function mostCommon(idx,filter,numRemoved) 
  local ones = 0
  for i,line in filterIter(filter) do
    -- print(i..": "..string.format("%x",line))
    ones = ones + bitOnPos(line,idx) 
  end
  return (ones >= (#rawData-numRemoved)/2) and 1 or 0
end

function bitOnPos(line,pos)
   local mask = 1 << row_len >> pos
   return ((line & mask) ~= 0 and 1 or 0)
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

local oxy = reduce(rawData,function(bit,common) return bit ~= common end )
print("oxy: "..oxy)

local scrub = reduce(rawData,function(bit,common) return bit == common end )
print("scrub: "..scrub)

print("soluton: "..(oxy*scrub))
