-module(main).
-export([start/0]).


req() -> [byr, iyr, eyr, hgt, hcl, ecl, pid].
between(Val, Lower, Upper) -> Val >= Lower andalso Val =< Upper.

validate(byr, Val) -> between(binary_to_integer(Val), 1920, 2002);
validate(iyr, Val) -> between(binary_to_integer(Val), 2010, 2020);
validate(eyr, Val) -> between(binary_to_integer(Val), 2020, 2030);
validate(hgt, <<Val:3/binary, "cm">>) -> between(binary_to_integer(Val), 150, 193);
validate(hgt, <<Val:2/binary, "in">>) -> between(binary_to_integer(Val), 59, 76);
validate(hcl, <<"#", Val:6/binary>>) -> sets:is_subset(sets:from_list(binary_to_list(Val)), sets:from_list("0123456789abcdef"));
validate(ecl, Val) -> sets:is_element(binary_to_atom(Val), sets:from_list([amb, blu, brn, gry, grn, hzl, oth]));
validate(pid, <<Val:9/binary>>) -> lists:all(fun(I) -> between(I, $0, $9) end, binary_to_list(Val));
validate(cid, _) -> true;
validate(_, _) -> false.

main(_) -> start().
start() -> 
    {ok, F} = file:read_file("input.txt"),
    L = string:split(F, "\n\n", all),
    LL = lists:map(fun(PS) ->
                           lists:map(fun(PF) -> 
                                             [FN, FV] = string:split(PF, ":"),
                                             [binary_to_atom(FN), FV]
                                     end,
                                     string:lexemes(PS, " \n"))
                   end,
                   L),
    LLL = lists:filter(fun(PS) ->
                               FS = sets:from_list(lists:map(fun([FN, _]) -> FN end, PS)),
                               Req = sets:from_list(req()),
                               sets:is_subset(Req, FS)
                       end,
                       LL),
    LLLL = lists:filter(fun(PS) -> lists:all(fun([FN, FV]) -> validate(FN, FV) end, PS) end, LLL),
    erlang:display([length(LLL), length(LLLL)]).
