use graphing_calc::Map;

fn main() {
    let func = vec![1, 4];
    let mut map = Map::new();
    map.draw(&func);
    map.print();
    //map.map[39][50] = "@ ";
}
