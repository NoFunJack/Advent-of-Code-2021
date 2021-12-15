data = {}

function printData()
  for _,v in ipairs(data) do
    for i,v2 in ipairs(v) do  io.write(v2.v.."("..v2.d..")") end
    io.write("\n")
  end
end

function dij(x,y)
  data[x][y].d = 0
  visitNext(x,y,0)
end

function visitNext(x,y,d)
  visit(x,y+1,d)
  visit(x+1,y,d)
end

function visit(x,y,d)
  n = data[x] and data[x][y] or nil
  if not n then return end

  tohere = d+n.v
  if tohere < n.d then 
    n.d = tohere
    visitNext(x,y,tohere)
  end

end

for line in assert(io.open(arg[1], "r")):lines() do
  row = {}
  for i = 1,#line do
    c = line:sub(i,i)
    table.insert(row,{v=tonumber(c),d=math.maxinteger})
  end
  table.insert(data,row)
end


dij(1,1)
printData()

print("Result: ",data[#data][#data[1]].d)
