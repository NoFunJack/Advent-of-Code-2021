data = {}
MAX_TILE = 5

function printData()
  for _,v in ipairs(data) do
    --for i,v2 in ipairs(v) do  io.write(v2.v.."("..v2.d..")") end
    for i,v2 in ipairs(v) do  io.write(v2.v) end
    io.write("\n")
  end
end

function dij(x,y)
  data[x][y].d = 0
  visitNext(x,y,0,0)
end

function visitNext(x,y,t,d)
  visit(x,y+1,t,d)
  visit(x+1,y,t,d)
end

function visit(x,y,t,dist)
  local n = data[x] and data[x][y] or nil
  if not n then 
    tx = (x-1)//xMax
    ty = (y-1)//yMax
    if math.max(tx,ty) >= MAX_TILE then return end
    xorg = orgIdx(x,xMax)
    yorg = orgIdx(y,yMax)
    nextV = data[xorg][yorg].v+tx+ty
    if nextV > 9 then nextV = nextV - 9 end
    data[x] = data[x] or {}
    data[x][y] = {v=nextV,d=math.maxinteger}
    n = data[x][y]
  end

  tohere = dist+n.v

  if tohere < n.d then 
    n.d = tohere
    visitNext(x,y,t,tohere)
  end

end

function orgIdx(i,max)
  n = i % max
  if n == 0 then return max 
  else return n end
end

assert(orgIdx(1,3)==1)
assert(orgIdx(2,3)==2)
assert(orgIdx(3,3)==3)

for line in assert(io.open(arg[1], "r")):lines() do
  row = {}
  for i = 1,#line do
    c = line:sub(i,i)
    table.insert(row,{v=tonumber(c),d=math.maxinteger})
  end
  table.insert(data,row)
  xMax = #data
  yMax = #row
end


dij(1,1)
printData()

print("Result: ",data[#data][#data[1]].d)
