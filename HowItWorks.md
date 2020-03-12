# Main State

The Main State contains the global data of the game.
```
struct MainState {
    pub squares: Vec<Square>,
    pub inputs: HashMap<InputAction, InputState>,
    pub current_block: Block,
    pub update_timer: usize,
    pub held_block: Option<BlockType>,
    pub queue: [usize; 14],
    pub queued_queue: [usize; 14],
    pub block_index: usize,
    pub used_hold: bool,
    pub lines: usize,
}
```

`squares` is a list of all the squares on the board. Logic is separated between tetrominoes and squares for the most part.

`inputs` is a map of actions (e.g. moving the selected block left or hard dropping it) to InputStates, which store the relevant details of how an input should be handled. I mainly made this so that keys repeat faster when held, but it also helps to make binding new keys easier.

`current_block` is the current, moveable, tetromino

`update_timer` is a timer to map frames to game ticks

`held_block` is the cached block

`queue` is the current queue, `queued_queue` is the secondary queue. I needed two queues so that I could shuffle each set of seven blocks as a group instead of adding a random block each time, in accordance with standard Tetris.

`block_index` is the current index of the queue.

`used_hold` is a check on whether or not a tetromino has been cached since the last placement.

`lines` is the number of completed lines

After `MainState` is initialized, it runs `update()` and `draw()` every frame. `update()` first updates and handles inputs. Next, it checks if a sufficient number of frames has passed to do the next game tick.

In the game tick, it translates the current_block down, checks if the it should be placed or not, and then, if it should be placed, it places it and makes a new current block.

`draw()` draws all the squares on the board, the current block, the projected landing spot of the current block, and then the queue and the cached block.

# TODO
`Square`, `Block` explanations
