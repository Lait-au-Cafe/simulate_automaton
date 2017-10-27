mod data;
mod node;
mod log;

use std::env;

fn disp_usage() {
    println!("Usage: command [init_statue(g/w)] [data files...]");
}

fn main() {
   
    // シミュレートに使うオートマトン
    let mut node_list: node::NodeList = Vec::new();
    
    let mut args = env::args();
    args.next(); // コマンドを消費

    match args.next(){
        None => {
            print!("1 : ");
            disp_usage();
            return;
        },
        Some(s) => {
            let mut cs = s.chars();
            let init_stat: char;
            match cs.next() {
                None => {
                    print!("2 : ");
                    disp_usage();
                    return;
                },
                Some(c) if (c == 'g') || (c == 'w') => {
                    init_stat = c;
                },
                Some(_) => {
                    print!("3 : ");
                    disp_usage();
                    return;
                },
            }
            
            // 初期状態を追加
            node_list.push(node::Node::new(init_stat));
        }
    }                
            
    // ログ用
    let mut log_list: log::LogList = Vec::new();
    
    'exec: for arg in args {
        // データの読み取り
        let data_list: data::DataList;
        match data::load_data(&arg) {
            Ok(v) => { data_list = v; },
            Err(msg) => { 
                println!("Failed to load data: {}", msg);
                continue; 
            },
        }
    
        let mut node_ptr: usize = 0;

        for (i, data) in data_list.iter().enumerate() {

            match node_list[node_ptr].get_path(data.input) {
                Some(dest) => {
                    if node_list[dest].state != data.output {
                        let mut msg = "\nDatas contradict with each other!!\n".to_owned();
                        // ログをさかのぼって相手を探す
                        let log = log_list.iter()
                            .filter(|x| (x.node_num == node_ptr) && (x.condition == data.input))
                            .next();
                        match log {
                            None => {
                                msg += "Cannot find log...";
                            },
                            Some(data) => {
                                msg += &format!("First: {} L{}\n", data.filename, data.line);
                                msg += &format!("Second: {} L{}\n", arg, i);
                            },
                        }
                        // panic!("{}", msg);
                        println!("{}", msg);
                        break 'exec;
                    }
                    node_ptr = dest;
                },
                None => {
                    // 問答無用で新しくノードを作る
                    let new_ptr = node_list.len();
                    node_list.push(node::Node::new(data.output));
                    node_list[node_ptr]
                        .set_path(data.input, new_ptr);
                    log_list.push(log::LogData{
                        node_num: node_ptr,
                        condition: data.input,
                        filename: arg.clone(),
                        line: i});
                    node_ptr = new_ptr;
                },
            }

        }
    }

    for (i, node) in node_list.iter().enumerate() {
        println!("{}:{}", i, node.to_string());
    }
}
