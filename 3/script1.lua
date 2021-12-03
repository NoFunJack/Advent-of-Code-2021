mostCommonBits = {}
readLines = 0

function readLine(line)
  for i=1,#line do
    bit = line:sub(i,i)
    mostCommonBits[i] = mostCommonBits[i] or 0 
    mostCommonBits[i] = mostCommonBits[i] + bit
  end
  readLines = readLines + 1
end

function determ(value) 
  return value > readLines/2 
end


for line in assert(io.open(arg[1], "r")):lines() do
  bitl = #line
  readLine(line)
end

gamma = ""
beta = ""
for i in ipairs(mostCommonBits) do
  if determ(mostCommonBits[i]) then
   gamma = gamma..1
   beta = beta..0
 else
   gamma = gamma..0
   beta = beta..1
 end
end
print("gamma: "..gamma)
gamma = tonumber(gamma,2)
print("dez: "..gamma)
print("beta: "..beta)
beta = tonumber(beta,2)
print("dez: "..beta)

print("solution: "..gamma*beta)




