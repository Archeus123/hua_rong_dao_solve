use std::{collections::HashMap, rc::Rc};

#[cfg(test)]
mod level2;
#[cfg(test)]
mod level3;

mod utils;

fn main() -> anyhow::Result<()> {
    utils::init_log();

    let state = r#"
    vvxv
    vvxv
    vvcc
    vvcc
    pppp
    "#;
    let state = parse_state(state)?;
    let hrd = HRD::new(&state)?;
    let ret = hrd.solve(1024)?;
    let steps = step_messages(&ret)?;
    log::info!("{} steps", steps.len());
    for e in steps {
        log::info!("{}", e);
    }

    Ok(())
}

fn step_messages(node: &Node) -> anyhow::Result<Vec<String>> {
    anyhow::ensure!(node.len > 0, "node is last, no message");

    let mut steps: Vec<String> = Vec::with_capacity(node.len as usize);
    let mut current_node = node;
    let mut current_game = Game::new_unchecked(&current_node.val);

    while let Some(prev_node) = current_node.parent.as_deref() {
        let prev_game = Game::new_unchecked(&prev_node.val);
        let msg = prev_game.move_message(&current_game.state)?;
        steps.push(msg);
        current_node = prev_node;
        current_game = prev_game;
    }
    steps.reverse();
    return Ok(steps);
}

#[cfg(test)]
fn show_solve(state: &str, limit: usize) {
    let state = log_guard!(parse_state(state));
    let hrd = log_guard!(HRD::new(&state));
    let ret = log_guard!(hrd.solve(limit));
    let steps = log_guard!(step_messages(&ret));
    log::info!("{} steps", steps.len());
    for e in steps {
        log::info!("{}", e);
    }
}

fn parse_state(state: &str) -> anyhow::Result<NodeValue> {
    let mut blocks: NodeValue = Default::default();
    let mut row = 0;
    let mut col = 0;
    for line in state.lines().map(|e| e.trim()).filter(|e| !e.is_empty()) {
        col = 0;
        for c in line.chars() {
            blocks[row][col] = match c {
                'c' => Some(BlockType::CaoCao),
                'h' => Some(BlockType::Horizontal),
                'v' => Some(BlockType::Vertical),
                'p' => Some(BlockType::Pawn),
                'x' => None,
                _ => anyhow::bail!("unknown token {}", c),
            };
            col += 1;
        }
        row += 1;
    }
    anyhow::ensure!(row == HEIGHT && col == WIDTH, "size error {}x{}", row, col);

    return Ok(blocks);
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum BlockType {
    CaoCao,
    Horizontal,
    Vertical,
    Pawn,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Block {
    ty: BlockType,
    x: usize,
    y: usize,
}

const WIDTH: usize = 4;
const HEIGHT: usize = 5;

type NodeValue = [[Option<BlockType>; WIDTH]; HEIGHT];

#[derive(Debug)]
struct Node {
    val: NodeValue,
    parent: Option<Rc<Node>>,
    len: u32,
}

impl Node {
    fn is_finish(&self) -> bool {
        self.val[HEIGHT - 1][1] == Some(BlockType::CaoCao)
            && self.val[HEIGHT - 1][2] == Some(BlockType::CaoCao)
    }
}

struct Game {
    blocks: [Block; 10],

    state: NodeValue,
    empty_cell: [(usize, usize); 2],
}

impl Game {
    fn new_unchecked(state: &NodeValue) -> Self {
        let mut blocks = [Block {
            ty: BlockType::Pawn,
            x: 0,
            y: 0,
        }; 10];
        let mut block_idx = 0;

        let mut visited = [[false; WIDTH]; HEIGHT];

        let mut empty_cell = [(0, 0); 2];
        let mut empty_cell_idx = 0;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match state[y][x] {
                    Some(BlockType::CaoCao) => {
                        if visited[y][x] == false {
                            blocks[block_idx] = Block {
                                ty: BlockType::CaoCao,
                                x,
                                y,
                            };
                            visited[y][x] = true;
                            visited[y][x + 1] = true;
                            visited[y + 1][x] = true;
                            visited[y + 1][x + 1] = true;
                            block_idx += 1;
                        }
                    }
                    Some(BlockType::Horizontal) => {
                        if visited[y][x] == false {
                            blocks[block_idx] = Block {
                                ty: BlockType::Horizontal,
                                x,
                                y,
                            };
                            visited[y][x] = true;
                            visited[y][x + 1] = true;
                            block_idx += 1;
                        }
                    }
                    Some(BlockType::Vertical) => {
                        if visited[y][x] == false {
                            blocks[block_idx] = Block {
                                ty: BlockType::Vertical,
                                x,
                                y,
                            };
                            visited[y][x] = true;
                            visited[y + 1][x] = true;
                            block_idx += 1;
                        }
                    }
                    Some(BlockType::Pawn) => {
                        blocks[block_idx] = Block {
                            ty: BlockType::Pawn,
                            x,
                            y,
                        };
                        visited[y][x] = true;
                        block_idx += 1;
                    }
                    None => {
                        empty_cell[empty_cell_idx] = (x, y);
                        empty_cell_idx += 1;
                    }
                }
            }
        }

        Self {
            blocks,
            state: state.clone(),
            empty_cell,
        }
    }

    fn new(state: &NodeValue) -> anyhow::Result<Self> {
        let mut blocks = Vec::with_capacity(10);
        let mut visited = [[false; WIDTH]; HEIGHT];
        let mut empty_cell = Vec::with_capacity(2);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match state[y][x] {
                    Some(BlockType::CaoCao) => {
                        if visited[y][x] == false {
                            anyhow::ensure!(visited[y][x + 1] == false);
                            anyhow::ensure!(visited[y + 1][x] == false);
                            anyhow::ensure!(visited[y + 1][x + 1] == false);
                            blocks.push(Block {
                                ty: BlockType::CaoCao,
                                x,
                                y,
                            });

                            visited[y][x] = true;
                            visited[y][x + 1] = true;
                            visited[y + 1][x] = true;
                            visited[y + 1][x + 1] = true;
                        }
                    }
                    Some(BlockType::Horizontal) => {
                        if visited[y][x] == false {
                            anyhow::ensure!(visited[y][x + 1] == false);
                            blocks.push(Block {
                                ty: BlockType::Horizontal,
                                x,
                                y,
                            });
                            visited[y][x] = true;
                            visited[y][x + 1] = true;
                        }
                    }
                    Some(BlockType::Vertical) => {
                        if visited[y][x] == false {
                            anyhow::ensure!(visited[y + 1][x] == false);
                            blocks.push(Block {
                                ty: BlockType::Vertical,
                                x,
                                y,
                            });

                            visited[y][x] = true;
                            visited[y + 1][x] = true;
                        }
                    }
                    Some(BlockType::Pawn) => {
                        anyhow::ensure!(visited[y][x] == false);
                        blocks.push(Block {
                            ty: BlockType::Pawn,
                            x,
                            y,
                        });
                        visited[y][x] = true;
                    }
                    None => {
                        empty_cell.push((x, y));
                    }
                }
            }
        }
        let cc_num = blocks.iter().filter(|b| b.ty == BlockType::CaoCao).count();
        if cc_num != 1 {
            anyhow::bail!("There must be exactly one CaoCao block, found {}", cc_num);
        }

        let blocks = <[Block; 10]>::try_from(blocks)
            .map_err(|e| anyhow::anyhow!("block must be 10, but get {}", e.len()))?;

        let empty_cell = <[(usize, usize); 2]>::try_from(empty_cell)
            .map_err(|e| anyhow::anyhow!("empty cell must be 2, but get {}", e.len()))?;

        Ok(Self {
            blocks,
            state: state.clone(),
            empty_cell,
        })
    }

    fn next_nodes(&self, ret: &mut Vec<NodeValue>) {
        for e in self.blocks.iter() {
            let x = e.x;
            let y = e.y;
            match e.ty {
                BlockType::CaoCao => {
                    //上移
                    if y >= 1 && self.empty_cell == [(x, y - 1), (x + 1, y - 1)] {
                        let mut node = self.state.clone();
                        node[y - 1][x] = Some(e.ty);
                        node[y - 1][x + 1] = Some(e.ty);
                        node[y + 1][x] = None;
                        node[y + 1][x + 1] = None;
                        ret.push(node);
                    }

                    //下移
                    if y < HEIGHT - 2 && self.empty_cell == [(x, y + 2), (x + 1, y + 2)] {
                        let mut node = self.state.clone();
                        node[y + 2][x] = Some(e.ty);
                        node[y + 2][x + 1] = Some(e.ty);
                        node[y][x] = None;
                        node[y][x + 1] = None;
                        ret.push(node);
                    }

                    //左移
                    if x >= 1 && self.empty_cell == [(x - 1, y), (x - 1, y + 1)] {
                        let mut node = self.state.clone();
                        node[y][x - 1] = Some(e.ty);
                        node[y + 1][x - 1] = Some(e.ty);
                        node[y][x + 1] = None;
                        node[y + 1][x + 1] = None;
                        ret.push(node);
                    }

                    //右移
                    if x < WIDTH - 2 && self.empty_cell == [(x + 2, y), (x + 2, y + 1)] {
                        let mut node = self.state.clone();
                        node[y][x + 2] = Some(e.ty);
                        node[y + 1][x + 2] = Some(e.ty);
                        node[y][x] = None;
                        node[y + 1][x] = None;
                        ret.push(node);
                    }
                }
                BlockType::Horizontal => {
                    //上移
                    if y >= 1 && self.empty_cell == [(x, y - 1), (x + 1, y - 1)] {
                        let mut node = self.state.clone();
                        node[y - 1][x] = Some(e.ty);
                        node[y - 1][x + 1] = Some(e.ty);
                        node[y][x] = None;
                        node[y][x + 1] = None;
                        ret.push(node);
                    }

                    //下移
                    if y < HEIGHT - 1 && self.empty_cell == [(x, y + 1), (x + 1, y + 1)] {
                        let mut node = self.state.clone();
                        node[y + 1][x] = Some(e.ty);
                        node[y + 1][x + 1] = Some(e.ty);
                        node[y][x] = None;
                        node[y][x + 1] = None;
                        ret.push(node);
                    }

                    //左移一格
                    if x >= 1 && self.empty_cell.contains(&(x - 1, y)) {
                        let mut node = self.state.clone();
                        node[y][x - 1] = Some(e.ty);
                        node[y][x + 1] = None;
                        ret.push(node);
                    }

                    //左移二格
                    if x >= 2 && self.empty_cell == [(x - 2, y), (x - 1, y)] {
                        let mut node = self.state.clone();
                        node[y][x - 2] = Some(e.ty);
                        node[y][x - 1] = Some(e.ty);
                        node[y][x] = None;
                        node[y][x + 1] = None;
                        ret.push(node);
                    }

                    //右移一格
                    if x < WIDTH - 2 && self.empty_cell.contains(&(x + 2, y)) {
                        let mut node = self.state.clone();
                        node[y][x + 2] = Some(e.ty);
                        node[y][x] = None;
                        ret.push(node);
                    }

                    //右移二格
                    if x < WIDTH - 3 && self.empty_cell == [(x + 2, y), (x + 3, y)] {
                        let mut node = self.state.clone();
                        node[y][x + 2] = Some(e.ty);
                        node[y][x + 3] = Some(e.ty);
                        node[y][x] = None;
                        node[y][x + 1] = None;
                        ret.push(node);
                    }
                }
                BlockType::Vertical => {
                    //上移一格
                    if y >= 1 && self.empty_cell.contains(&(x, y - 1)) {
                        let mut node = self.state.clone();
                        node[y - 1][x] = Some(e.ty);
                        node[y + 1][x] = None;
                        ret.push(node);
                    }

                    //上移二格
                    if y >= 2 && self.empty_cell == [(x, y - 2), (x, y - 1)] {
                        let mut node = self.state.clone();
                        node[y - 2][x] = Some(e.ty);
                        node[y - 1][x] = Some(e.ty);
                        node[y][x] = None;
                        node[y + 1][x] = None;
                        ret.push(node);
                    }

                    //下移一格
                    if y < HEIGHT - 2 && self.empty_cell.contains(&(x, y + 2)) {
                        let mut node = self.state.clone();
                        node[y + 2][x] = Some(e.ty);
                        node[y][x] = None;
                        ret.push(node);
                    }

                    //下移二格
                    if y < HEIGHT - 3 && self.empty_cell == [(x, y + 2), (x, y + 3)] {
                        let mut node = self.state.clone();
                        node[y + 2][x] = Some(e.ty);
                        node[y + 3][x] = Some(e.ty);
                        node[y][x] = None;
                        node[y + 1][x] = None;
                        ret.push(node);
                    }

                    //左移
                    if x >= 1 && self.empty_cell == [(x - 1, y), (x - 1, y + 1)] {
                        let mut node = self.state.clone();
                        node[y][x - 1] = Some(e.ty);
                        node[y + 1][x - 1] = Some(e.ty);
                        node[y][x] = None;
                        node[y + 1][x] = None;
                        ret.push(node);
                    }

                    //右移
                    if x < WIDTH - 1 && self.empty_cell == [(x + 1, y), (x + 1, y + 1)] {
                        let mut node = self.state.clone();
                        node[y][x + 1] = Some(e.ty);
                        node[y + 1][x + 1] = Some(e.ty);
                        node[y][x] = None;
                        node[y + 1][x] = None;
                        ret.push(node);
                    }
                }
                BlockType::Pawn => {
                    //上移一格
                    if y >= 1 && self.empty_cell.contains(&(x, y - 1)) {
                        let mut node = self.state.clone();
                        node[y - 1][x] = Some(e.ty);
                        node[y][x] = None;
                        ret.push(node);
                    }

                    //上移二格
                    if y >= 2 && self.empty_cell == [(x, y - 2), (x, y - 1)] {
                        let mut node = self.state.clone();
                        node[y - 2][x] = Some(e.ty);
                        node[y][x] = None;
                        ret.push(node);
                    }

                    //下移一格
                    if y < HEIGHT - 1 && self.empty_cell.contains(&(x, y + 1)) {
                        let mut node = self.state.clone();
                        node[y + 1][x] = Some(e.ty);
                        node[y][x] = None;
                        ret.push(node);
                    }

                    //下移二格
                    if y < HEIGHT - 2 && self.empty_cell == [(x, y + 1), (x, y + 2)] {
                        let mut node = self.state.clone();
                        node[y + 2][x] = Some(e.ty);
                        node[y][x] = None;
                        ret.push(node);
                    }

                    //左移一格
                    if x >= 1 && self.empty_cell.contains(&(x - 1, y)) {
                        let mut node = self.state.clone();
                        node[y][x - 1] = Some(e.ty);
                        node[y][x] = None;
                        ret.push(node);
                    }

                    //左移二格
                    if x >= 2 && self.empty_cell == [(x - 2, y), (x - 1, y)] {
                        let mut node = self.state.clone();
                        node[y][x - 2] = Some(e.ty);
                        node[y][x] = None;
                        ret.push(node);
                    }

                    //右移一格
                    if x < WIDTH - 1 && self.empty_cell.contains(&(x + 1, y)) {
                        let mut node = self.state.clone();
                        node[y][x + 1] = Some(e.ty);
                        node[y][x] = None;
                        ret.push(node);
                    }

                    //右移二格
                    if x < WIDTH - 2 && self.empty_cell == [(x + 1, y), (x + 2, y)] {
                        let mut node = self.state.clone();
                        node[y][x + 2] = Some(e.ty);
                        node[y][x] = None;
                        ret.push(node);
                    }
                }
            }
        }
    }

    fn move_message(&self, next: &NodeValue) -> anyhow::Result<String> {
        let next_blocks = Game::new(next)?.blocks;
        let block0 = self
            .blocks
            .iter()
            .filter(|e| !next_blocks.contains(e))
            .collect::<Vec<_>>();
        let block1 = next_blocks
            .iter()
            .filter(|e| !self.blocks.contains(e))
            .collect::<Vec<_>>();

        anyhow::ensure!(
            block0.len() == 1 && block1.len() == 1 && block0[0].ty == block1[0].ty,
            "2个状态无法通过一次移动完成转化"
        );

        let block0 = block0[0];
        let block1 = block1[0];

        let dx = block1.x as i32 - block0.x as i32;
        let dy = block1.y as i32 - block0.y as i32;

        let dir = match (dx, dy) {
            (0, 1) => "下",
            (0, 2) => "下2",
            (0, -1) => "上",
            (0, -2) => "上2",
            (-1, 0) => "左",
            (-2, 0) => "左2",
            (1, 0) => "右",
            (2, 0) => "右2",
            _ => anyhow::bail!(
                "unknown move ({},{}) => ({},{})",
                block0.x,
                block0.y,
                block1.x,
                block1.y
            ),
        };

        let ret = format!("({},{}) {}", block0.x, block0.y, dir);
        return Ok(ret);
    }
}

struct HRD {
    open_set: HashMap<NodeValue, Rc<Node>>,
    close_set: HashMap<NodeValue, Rc<Node>>,
}

impl HRD {
    fn new(state: &NodeValue) -> anyhow::Result<Self> {
        let start = Game::new(state)?;
        let mut open_set = HashMap::new();
        open_set.insert(
            start.state.clone(),
            Rc::new(Node {
                val: start.state,
                parent: None,
                len: 0,
            }),
        );
        Ok(Self {
            open_set,
            close_set: Default::default(),
        })
    }

    fn solve(mut self, limit: usize) -> anyhow::Result<Node> {
        let mut next_nodes: Vec<NodeValue> = Vec::with_capacity(16);
        loop {
            let node = self
                .open_set
                .values()
                .min_by_key(|e| e.len)
                .ok_or_else(|| anyhow::anyhow!("can't find solve"))?;

            let node = Rc::clone(node);
            if node.is_finish() {
                self.open_set.clear();
                self.close_set.clear();
                return Ok(Rc::try_unwrap(node).unwrap());
            }

            if self.open_set.len() + self.close_set.len() >= limit {
                anyhow::bail!("node size exceed {}", limit);
            }

            self.open_set.remove(&node.val);
            self.close_set.insert(node.val.clone(), Rc::clone(&node));

            let game = Game::new_unchecked(&node.val);
            game.next_nodes(&mut next_nodes);

            for e in next_nodes.drain(..) {
                if self.close_set.contains_key(&e) {
                    continue;
                }
                if self.open_set.contains_key(&e) {
                    continue;
                }
                self.open_set.insert(
                    e.clone(),
                    Rc::new(Node {
                        val: e,
                        parent: Some(Rc::clone(&node)),
                        len: node.len + 1,
                    }),
                );
            }
        }
    }
}
