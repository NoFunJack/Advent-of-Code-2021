function phase(input) 
  local l,r = input:match("%b[(%d),(%d)]")
  print(l,r)

end


local input = arg[1]
phase(input)
