
strset = {}

strset.minus = function(s1,s2) 
  if #s1 == 0 then return "" end
  local re = ""
  for c in s1:gmatch("[^"..s2.."]") do
    re = re..c
  end
  return re
end

assert(strset.minus("","")=="")
assert(strset.minus("abc","abc")=="")
assert(strset.minus("abc","ac")=="b")
assert(strset.minus("ab","abc")=="")


strset.diff = function(s1,s2) 
  if #s2 == 0 then return "" end

  local re = ""
  for c in s1:gmatch("[^"..s2.."]") do
    re = re..c
  end
  for c in s2:gmatch("[^"..s1.."]") do
    re = re..c
  end
  return re
end

assert(strset.diff("","")=="")
assert(strset.diff("abc","abc")=="")
assert(strset.diff("abc","ac")=="b")
assert(strset.diff("ab","abc")=="c")

strset.inter = function(s1,s2) 

  if #s1 == 0 then return s2 end

  local re = ""
  for c in s1:gmatch("["..s2.."]") do
    re = re..c
  end
  return re
end

assert(strset.inter("","")=="")
assert(strset.inter("abc","abc")=="abc")
assert(strset.inter("abc","ac")=="ac")
assert(strset.inter("ab","abc")=="ab")

