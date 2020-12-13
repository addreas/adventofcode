use "buffered"
use "itertools"

primitive North //means to move by the given value.
primitive South //means to move by the given value.
primitive East //means to move by the given value.
primitive West //means to move by the given value.
primitive Left //means to turn the given number of degrees.
primitive Right //means to turn the given number of degrees.
primitive Forward //means to move by the given value in the direction the ship is instructionsly facing.


type Action is (North | South | East | West | Left | Right | Forward)
type ActionValue is (Action, F64)

actor Main
  new create(env: Env) =>
    let runner = Runner(env.out)

    env.input(object iso is InputNotify
      let buf: Reader iso = Reader

      fun ref apply(data: Array[U8] iso) =>
        buf.append(consume data)
        while buf.size() > 0 do
          try
            runner.push(parse(buf.line(false)?)?)
          else
            break
          end
        end

      fun ref parse(data: String): ActionValue val ? =>
        let a: Action val = match data.substring(0, 1)
          | "N" => North
          | "S" => South
          | "E" => East
          | "W" => West
          | "L" => Left
          | "R" => Right
          | "F" => Forward
        else
          error
        end
        (a, data.substring(1).f64()?)

      fun ref dispose() =>
        runner.run()

    end, 512)

actor Runner
  let out: OutStream
  var instructions: Array[ActionValue] ref = []

  new create(_out: OutStream) =>
    out = _out

  be push(instruction: ActionValue val) =>
    instructions.push(instruction)

  be run() =>
    // unable to match tuples in arguments?
    // Iter[ActionValue](instructions.values())
    // .fold[(F64, F64, F64)]((0, 0, 0), {
    //   ((ns, ew, dir), (action, value)) =>
    //       match action
    //       | North => (ns + value.f64(), ew, dir)
    //       | South => (ns - value.f64(), ew, dir)
    //       | East => (ns, ew + value.f64(), dir)
    //       | West => (ns, ew - value.f64(), dir)
    //       | Right => (ns, ew, dir - (value.f64() * (F64.pi() / 180)))
    //       | Left => (ns, ew, dir + (value.f64() * (F64.pi() / 180)))
    //       | Forward => (ns + (value.f64() * dir.cos()), ew + (value.f64() * dir.sin()), dir)
    //     end
    //   })

    var ns: F64 = 0
    var ew: F64 = 0
    var dir: F64 = 0
    for (action, value) in instructions.values() do
      match action
        | North => ns = ns + value
        | South => ns = ns - value
        | East => ew = ew + value
        | West => ew = ew - value
        | Right => dir = dir - (value * (F64.pi() / 180))
        | Left => dir = dir + (value * (F64.pi() / 180))
        | Forward =>
            ns = ns + (value * dir.cos())
            ew = ew + (value * dir.sin())
      end
    end
    out.print((ns.abs() + ew.abs()).string())


    ns = 1
    ew = 10
    var posns: F64 = 0
    var posew: F64 = 0
    for (action, value) in instructions.values() do
      match action
        | North => ns = ns + value
        | South => ns = ns - value
        | East => ew = ew + value
        | West => ew = ew - value
        | Right =>
            let d = -(value * (F64.pi() / 180))
            let ewp = (ew * d.cos()) - (ns * d.sin())
            let nsp = (ns * d.cos()) + (ew * d.sin())
            ns = nsp
            ew = ewp
        | Left =>
            let d = (value * (F64.pi() / 180))
            let ewp = (ew * d.cos()) - (ns * d.sin())
            let nsp = (ns * d.cos()) + (ew * d.sin())
            ns = nsp
            ew = ewp
        | Forward =>
            posns = posns + (ns * value)
            posew = posew + (ew * value)
      end
    end

    out.print((posns.abs() + posew.abs()).string())

