import Pkg; Pkg.add("Combinatorics")
using Combinatorics
lines = readlines("input.txt")
intlines = parse.(Int, lines)
first_index = 25 + findfirst([all([sum(x) for x = combinations(intlines[(i-25):(i-1)], 2)] .!= intlines[i]) for i = 26:length(intlines)])
println(intlines[first_index])
println(first(sum(extrema(intlines[i:j])) for i = 1:length(intlines) for j = 1:length(intlines) if sum(intlines[i:j]) == intlines[first_index]))
