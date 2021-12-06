-- load file
local fish = {}
do 
  local f = io.open(arg[1],"rb"):read("*all")
  for word in string.gmatch(f, '([^,]+)') do
    local c = tonumber(word,10)
    fish[c] = fish[c] and fish[c] + 1 or 1
  end
end

-- apply change to fish
function nextDay() 
  spawner = fish[0] or 0
  updateFish(8,0)
  -- spawn last
  spawnFish(spawner)
end


function updateFish(idx,oldval) 
  local next = fish[idx]
  fish[idx] = oldval
  if idx > 0 then
    updateFish(idx -1, next)
  end
end

function spawnFish(spawner)
  fish[8] = spawner
  local sixer = fish[6] or 0
  fish[6] = sixer + spawner 
end

function printFish() 
  for i = 0,8 do fish[i] = fish[i] or 0 end
  return fish[0]..", "..table.concat(fish,", ")
end

 function countFish()
  local sum =0
  for i = 0,8 do
    sum = sum + fish[i] or 0
  end
  return sum
 end

local maxDay = tonumber(assert(arg[2]))
--print("Initial: "..printFish())
for day = 1,maxDay do
  nextDay()
  --print("After Day "..day.." : "..printFish())
end
print("Solution: "..countFish())
