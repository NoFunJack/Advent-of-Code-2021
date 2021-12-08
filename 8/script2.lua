require('strset')
-- oop displays
-- using
--[[
 aaa
b   c
b   c
 ddd
e   f
e   f
 ggg
--]]
uniqueMap = {}
uniqueMap[2]=1
uniqueMap[4]=4
uniqueMap[3]=7
uniqueMap[7]=8

Display = {}
function Display:new(line) 
  local input = line:gsub("|.*","")
  self.input = {}
  for word in input:gmatch("%a+") do
    table.insert(self.input,word)
  end
  local output = line:gsub(".*|","")
  self.output = {}
  for word in output:gmatch("%a+") do
    table.insert(self.output,word)
  end


  self.codes = {}
  self.seg = {}
end

function Display:phase() 

  local fiver = {}
  local sixer = {}

  -- determine 1,4,7,8
  for _,code in ipairs(self.input) do
    local num = getNumForLength(#code)
    if num ~= nil then
      print("code",num,code)
      self.codes[num] = code
    elseif #code == 5 then table.insert(fiver,code) 
    elseif #code == 6 then table.insert(sixer,code) 
    else error("unknown number"..#code)
    end
  end

  -- determin segments
  self.seg.a = strset.diff(self.codes[7],self.codes[1])

  -- determine c+f by finding the sixer (6) with non empty intersect 
  for _,six in ipairs(sixer) do
    local c = strset.minus(self.codes[1],six)
    if #c == 1 then
      self.codes[6] = six
      self.seg.c = c
      self.seg.f = strset.inter(self.codes[1],c)
      print("code",6,six)
      break
    end
  end

  -- determine other sixer (9,0) by intersecting with 4
  for _,six in ipairs(sixer) do
      -- skip six
      if six ~= self.codes[6] then 
        local diff = strset.minus(self.codes[4],six)
        if #diff == 0 then
          self.codes[9] = six
          print("code",9,six)
        else
          self.codes[0] = six
          print("code",0,six)
          self.seg.d=diff
        end
    end
  end

  -- determine fiver
  -- 5 is the only one without c-segment
  for _,five in ipairs(fiver) do
    if not five:match(self.seg.c) then
      self.codes[5]=five
      print("code",5,five)
      break
    end
  end
  
  -- determin 2,3 by intersecting with 1
  for _,five in ipairs(fiver) do
    if five ~= self.codes[5] then
      local inter = strset.minus(self.codes[1],five)
      if #inter == 0 then
        self.codes[3]=five
        print("code",3,five)
      elseif #inter == 1 then 
        self.codes[2]=five
        print("code",2,five)
      end
    end
  end



  -- check
  assert(self.codes[0])
  assert(self.codes[1])
  assert(self.codes[2])
  assert(self.codes[3])
  assert(self.codes[4])
  assert(self.codes[5])
  assert(self.codes[6])
  assert(self.codes[7])
  assert(self.codes[8])
  assert(self.codes[9])
end


function Display:getValue()
  local re = ""
  for _,code in ipairs(self.output) do
    re = re..assert(self:decode(code),":[")
  end
  return tonumber(re)
end

function Display:decode(str) 
  for num,code in pairs(self.codes) do
    if #strset.diff(code,str) == 0 then return num end
  end
end

function getNumForLength(n) 
  if uniqueMap[n] ~= nil then
    return uniqueMap[n]
  end

end

--read data
sum = 0
for line in assert(io.open(arg[1], "r")):lines() do
  print(line)
  Display:new(line) 
  Display:phase()
  sum = sum + Display:getValue()
end

print("Solution: "..sum)
