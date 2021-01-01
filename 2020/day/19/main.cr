rules, messages = STDIN.each_line.partition { |l| l.includes? ':' }

rulehash = {} of String => (String | Array(Array(String)))
rules
    .compact_map { |r| r.match /(\d+): ("(.*)"|(.*))/ }
    .each { |m|
        if m[3]?
            rulehash[m[1]] = m[3]
        else
            clauses = m[4].split(" | ").map { |c| c.split(" ") }
            rulehash[m[1]] = clauses
        end
    }

def join(rule, rulehash)
    item = rulehash[rule]

    case item
    when String
        item
    when Array(Array(String))
        outer = item.map { |clause|
            inner = clause.map { |part| join(part, rulehash).as(String) }.join
            "(#{inner})"
        }.join "|"
        "(#{outer})"
    end
end

pattern = Regex.new("^#{join("0", rulehash).as(String)}$")
matching = messages.select { |m| m.match pattern }
puts matching.size
