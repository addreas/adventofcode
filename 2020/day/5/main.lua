
function get_seat (line)
    _, _, rows, cols = string.find(line, "([FB]+)([LR]+)")
    return get_bsp(rows, "F", "B", 0, 127), get_bsp(cols, "L", "R", 0, 7)
end

function get_bsp (parts, low_char, hight_char, low, high)
    if low == high or parts == "" then
        return low
    end
    local mid = low + (high - low) / 2
    if string.sub(parts, 1, 1) == low_char then
        return get_bsp(string.sub(parts, 2), low_char, high_char, low, math.floor(mid))
    else
        return get_bsp(string.sub(parts, 2), low_char, high_char, math.ceil(mid), high)
    end
end

local seats = {}
for line in io.lines() do
    row, col = get_seat(line)
    table.insert(seats, row * 8 + col)
end
table.sort(seats)
local last_seat = table.remove(seats, 1)
repeat
    local seat = table.remove(seats, 1)
    if seat == last_seat + 2 then
        print(seat - 1)
        break
    end
    last_seat = seat
until last_seat == nil
