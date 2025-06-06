// fec.rs
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use meval::Expr;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Formula(String), // 計算式として保持するための型
}

pub struct FuckExCel {
    cells: HashMap<String, Value>, // セル名と値のマップ
    dependencies: HashMap<String, HashSet<String>>, // セル -> 依存しているセル一覧（セルの計算式が参照するセル）
    dependents: HashMap<String, HashSet<String>>,   // セル -> それに依存しているセル一覧（変更時に再計算するセル）
}

impl FuckExCel {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    // セルオブジェクトを作る（セル名と自身への参照を渡す）
    pub fn cell(&mut self, name: &str) -> Cell {
        Cell {
            name: name.to_string(),
            book: self,
        }
    }

    // セルに値をセットし、依存関係を更新＆再計算する
    pub fn set_cell(&mut self, name: &str, val: Value) {
        self.cells.insert(name.to_string(), val.clone());

        self.update_dependencies(name, &val);    // 依存セルの登録更新
        self.recalculate_dependents(name);       // 依存セルの再計算を順次実行
    }

    // 計算式から依存しているセル名を抽出し、依存関係を更新
    fn update_dependencies(&mut self, name: &str, val: &Value) {
        // 既存依存情報をクリア
        if let Some(old_deps) = self.dependencies.remove(name) {
            for dep in old_deps {
                if let Some(dep_set) = self.dependents.get_mut(&dep) {
                    dep_set.remove(name);
                }
            }
        }

        // 式の場合、依存セルを抽出して依存関係をセット
        if let Value::Formula(expr) = val {
            let deps = Self::extract_cells_from_expr(expr);
            if !deps.is_empty() {
                self.dependencies.insert(name.to_string(), deps.clone());
                for dep in deps {
                    self.dependents
                        .entry(dep)
                        .or_default()
                        .insert(name.to_string());
                }
            }
        }
    }

    // 計算式からセル名っぽいトークンを抽出する単純なパーサー的処理
    fn extract_cells_from_expr(expr: &str) -> HashSet<String> {
        let mut set = HashSet::new();
        for token in expr.split(|c: char| !c.is_alphanumeric()) {
            if !token.is_empty() && token.chars().next().unwrap().is_alphabetic() {
                set.insert(token.to_string());
            }
        }
        set
    }

    // 変更されたセルに依存するセルを再帰的に再計算
    fn recalculate_dependents(&mut self, name: &str) {
        let mut to_recalc = vec![name.to_string()];
        let mut visited = HashSet::new();

        while let Some(cell) = to_recalc.pop() {
            if !visited.insert(cell.clone()) {
                continue; // 循環や重複の再計算防止
            }

            // 計算式のセルなら評価して値を更新
            if let Some(Value::Formula(expr)) = self.cells.get(&cell).cloned() {
                if let Some(val) = self.eval_expr(&expr) {
                    self.cells.insert(cell.clone(), val);
                }
            }

            // そのセルに依存するセルも再計算対象に追加
            if let Some(deps) = self.dependents.get(&cell) {
                for dep in deps {
                    to_recalc.push(dep.clone());
                }
            }
        }
    }

    // 式を評価してf64の値を得る（セル名を変数として渡す）
    pub fn eval_expr(&self, expr: &str) -> Option<Value> {
        let mut vars = HashMap::new();
        for (k, v) in &self.cells {
            if let Some(f) = v.as_float() {
                vars.insert(k.as_str(), f);
            } else if let Some(i) = v.as_int() {
                vars.insert(k.as_str(), i as f64);
            }
        }
        let parsed = expr.parse::<Expr>().ok()?;
        let val = parsed.eval_with_context(&vars).ok()?;
        Some(Value::Float(val))
    }
}

pub struct Cell<'a> {
    name: String,
    book: &'a mut FuckExCel,
}

impl<'a> Cell<'a> {
    // セルに値をセット（本体のset_cell呼び出し）
    pub fn set(&mut self, val: Value) {
        self.book.set_cell(&self.name, val);
    }

    // セルの値を参照
    pub fn get(&self) -> Option<&Value> {
        self.book.cells.get(&self.name)
    }
}

impl Value {
    pub fn as_int(&self) -> Option<i64> {
        if let Value::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
}
