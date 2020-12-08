instructions = ARGF.readlines.map do |l| l.split() end

def run(instructions)
  accumilator = 0
  pos = 0
  visited = []
  while !visited.include?(pos) and pos < instructions.length do
    visited.append(pos)
    inst = instructions[pos]
    case inst[0]
      in "acc"
        accumilator += Integer(inst[1])
        pos += 1
      in "jmp"
        pos += Integer(inst[1])
      else
        pos += 1
    end
  end
  [accumilator, visited, pos]
end

def switch_inst(instructions, pos)
  ret = Array.new instructions
  ret[pos] = Array.new ret[pos]
  case ret[pos][0]
    in "jmp"
      ret[pos][0] = "nop"
    in "nop"
      ret[pos][0] = "jmp"
    else
  end
  ret
end

res = run(instructions)
puts res[0]
for pos in 0..instructions.length do
  switched = switch_inst(instructions, pos)
  res2 = run(switched)
  if !res2[1].include?(res2[2])
    puts res2[0]
    break
  end
end

