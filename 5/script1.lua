Map = {}
h = 0
w = 0

function addLine(line)

  local _,_,x1,y1,x2,y2 = line:find("(%d+),(%d+)%D+(%d+),(%d+)")
  x1 = tonumber(x1)
  x2 = tonumber(x2)
  y1 = tonumber(y1)
  y2 = tonumber(y2)

  w = math.max(w,x1,x2)
  h = math.max(h,y1,y2)

  -- points
  if x1==x2 and y1==y2 then
    addPoint(x1,y1)
    return
  end

  -- horizontal
  if x1==x2 then 
    for y_i = math.min(y1,y2), math.max(y1,y2) do
      addPoint(x1,y_i)
    end
    return
  end

  -- vertical
  if y1==y2 then 
    for x_i = math.min(x1,x2), math.max(x1,x2) do
      addPoint(x_i,y1)
    end
    return 
  end

  -- part2 points
  if arg[2] ~= "part2" then return end

  local dx = x2-x1
  local dy = y2-y1
  local xmin = math.min(x1,x2)
  local ymin = math.min(y1,y2)
  local ymax = math.max(y1,y2)


  -- filter non-diagonals
  if math.abs(dx) ~= math.abs(dy) then return end

  -- positive diagonal
  if char(dx)==char(dy) then 
    for i = 0,math.abs(dx) do
      addPoint(xmin+i,ymin+i)
    end
    return 
  end

  -- other diagonal
  if char(dx)~=char(dy) then 
    for i = 0,math.abs(dx) do
      addPoint(xmin+i,ymax-i)
    end
    return 
  end
end

function char(x) 
  if x >= 0 then return 1
  else return -1
  end
end

function addPoint(x,y)
  Map[x] = Map[x] or {}
  Map[x][y] = Map[x][y] and Map[x][y] + 1 or 1
end

function Map:print()
  for y = 0,h do
    for x = 0, w do
      if Map[x] then
        io.write(Map[x][y] or ".")
      else 
        io.write(".")
      end
    end
    io.write("\n")
  end
end

function Map:countDanger(dval)
  local c = 0
  for y = 0,h do
    for x = 0, w do
      points = points+1
      if self[x] and self[x][y] then
        if self[x][y] >= dval then c = c+1 end
      end
    end
  end
  return c
end

for line in assert(io.open(arg[1], "r")):lines() do
  addLine(line)
end

--Map:print()

print("h: "..h.." w: "..w)
print("Solution: "..Map:countDanger(2))
