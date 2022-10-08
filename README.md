# snake
a tui snake game made with rust

![demo](assets/demo.gif)

## Usage

```
Usage: snake [OPTIONS]

Options:
      --width <SIZE>     Width of the game area [default: 30]
      --height <SIZE>    Height of the game area [default: 20]
  -s, --speed <SPEED>    Movement speed of the snake [default: 10]
  -x, --head-x <COORD>   Initial x coordinate of the snake's head [default: 3]
  -y, --head-y <COORD>   Initial y coordinate of the snake's head [default: 3]
  -l, --length <LENGTH>  Initial length of the snake [default: 3]
  -d, --dir <DIRECTION>  Initial direction of the snake [default: right] [possible values: left, right, up, down]
      --no-border        Disable borders
      --self-play        Run the game in self playing mode
  -p, --path-alg <ALG>   Shortest path algorithm used for self playing mode [default: bfs] [possible values: astar, bfs]
  -h, --help             Print help information
```

## References

- Hamilton Solver Implementation (https://github.com/chuyangliu/snake/blob/master/docs/algorithms.md#hamilton-solver)
