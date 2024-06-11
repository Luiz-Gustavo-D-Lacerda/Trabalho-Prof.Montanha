use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct MateriaPrima {
    pub nome: String,
}

#[derive(Deserialize, Debug)]
pub struct Produto {
    pub nome: String,
    pub materias_primas: Vec<String>, // lista de nomes de materias primas
    pub tempo_fabricacao: i32,
}

#[derive(Deserialize, Debug)]
pub struct Pedido {
    pub produto: String,  // Nome do produto para coincidir com o JSON fornecido
    pub data_entrega: String,
}

impl MateriaPrima {
    pub fn from_json(file_name: &str) -> Vec<MateriaPrima> {
        let materias_primas = fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        serde_json::from_str(&materias_primas).expect("Erro ao desserializar")
    }
}

impl Produto {
    pub fn from_json(file_name: &str) -> Vec<Produto> {
        let produtos = fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        serde_json::from_str(&produtos).expect("Erro ao desserializar")
    }
}

impl Pedido {
    pub fn from_json(file_name: &str) -> Vec<Pedido> {
        let pedidos = fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        serde_json::from_str(&pedidos).expect("Erro ao desserializar")
    }
}
