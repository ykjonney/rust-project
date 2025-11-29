use std::{collections::HashMap, vec};


/*基于邻接矩阵实现的无向图类型 */
pub struct Graph_Matrix {
    vertices: Vec<i32>,        // 顶点列表
    adj_matrix: Vec<Vec<i32>>, // 邻接矩阵
}

impl Graph_Matrix {
    pub fn new(vertices: Vec<i32>, edges: Vec<[usize; 2]>) -> Self {
        let mut graph = Graph_Matrix {
            vertices: vec![],
            adj_matrix: vec![],
        };
        for v in vertices {
            graph.add_vertex(v);
        }
        for edge in edges {
            graph.add_edge(edge[0], edge[1]);
        }

        graph
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    pub fn add_vertex(&mut self, v: i32) {
        self.vertices.push(v);
        let n = self.size();
        for row in self.adj_matrix.iter_mut() {
            row.push(0);
        }
        let mut new_row = vec![0; n];
        self.adj_matrix.push(new_row);
    }

    pub fn remove_vertex(&mut self, index: usize) {
        if index >= self.size() {
            return;
        }
        self.vertices.remove(index);
        self.adj_matrix.remove(index);
        for row in self.adj_matrix.iter_mut() {
            row.remove(index);
        }
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        if i >= self.size() || j >= self.size() || i == j {
            return;
        }
        self.adj_matrix[i][j] = 1;
        self.adj_matrix[j][i] = 1; // 无向图
    }

    pub fn remove_edge(&mut self, i: usize, j: usize) {
        if i >= self.size() || j >= self.size() || i == j {
            return;
        }
        self.adj_matrix[i][j] = 0;
        self.adj_matrix[j][i] = 0; // 无向图
    }

    pub fn print_matrix(&self) {
        println!("顶点列表：{:?}", self.vertices);
        println!("邻接矩阵：");
        println!("[");
        for row in self.adj_matrix.iter() {
            println!("{:?}", row);
        }
        println!("]");
    }
}


/* 基于邻接表的实现*/
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vertex{
    pub value:i32,
}
pub struct Graph_List {
    pub adj_list: HashMap<Vertex, Vec<Vertex>>, // 邻接表
}

impl Graph_List {
    fn new(){

    }
    pub fn size(&self)->usize{
        self.adj_list.len()
    }
    pub fn add_vertex(&mut self,v:Vertex){
        if self.adj_list.contains_key(&v){
            return;
        }
        self.adj_list.insert(v, vec![]);
    }

    pub fn remove_vertex(&mut self,v:Vertex){
        if !self.adj_list.contains_key(&v){
            return;
        }
        self.adj_list.remove(&v);
        for (_key,neighbors) in self.adj_list.iter_mut(){
            neighbors.retain(|x| x!=&v);
        }
    }
    pub fn add_edge(&mut self,v1:Vertex,v2:Vertex){
        if !self.adj_list.contains_key(&v1) || !self.adj_list.contains_key(&v2){
            return;
        }
        self.adj_list.entry(v1).or_default().push(v2);
        self.adj_list.entry(v2).or_default().push(v1);

    }

    pub fn remove_edge(&mut self,v1:Vertex,v2:Vertex){
        if !self.adj_list.contains_key(&v1) || !self.adj_list.contains_key(&v2){
            return;
        }
        if let Some(neighbors) = self.adj_list.get_mut(&v1){
            neighbors.retain(|x| x!=&v2);
        }
        if let Some(neighbors) = self.adj_list.get_mut(&v2){
            neighbors.retain(|x| x!=&v1);
        }
    }
}