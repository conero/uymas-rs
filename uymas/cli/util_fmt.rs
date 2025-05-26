/// 列表格式化
pub fn list_format<T: std::fmt::Display>(list: Vec<T>) -> String {
    let list_len = list.len();
    let max_len = format!("{}", list_len).len();

    // 输出类型
    let mut result = vec![];
    for (index, value) in list.iter().enumerate() {
        result.push(format!("{:>width$}   {}", index, value, width = max_len));
    }
    result.join("\n").to_string()
}

pub fn table_format<T: std::fmt::Display>(table_list: Vec<Vec<T>>) -> String {
    let mut max_len = vec![];
    // 计算行中的列最大长度
    for (_, list) in table_list.iter().enumerate() {
        for (col_idx, value) in list.iter().enumerate() {
            let cur_len = format!("{}", value).len();
            if max_len.len() <= col_idx {
                max_len.push(cur_len);
            } else {
                max_len[col_idx] = std::cmp::max(max_len[col_idx], cur_len);
            }
        }
    }

    // 输出构造
    let mut result = vec![];
    for (_, list) in table_list.iter().enumerate() {
        let mut row_result = vec![];
        for (col_idx, value) in list.iter().enumerate() {
            row_result.push(format!("{:<width$}", value, width = max_len[col_idx]));
        }
        result.push(row_result.join("  "));
    }
    result.join("\n").to_string()
}
