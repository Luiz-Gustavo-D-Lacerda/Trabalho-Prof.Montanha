use crate::pcp::{MateriaPrima, Produto, Pedido};

mod pcp;

fn main() {
    println!("Sistema de Planejamento e Controle de Produção (PCP)");

    let materias_primas: Vec<MateriaPrima> = pcp::MateriaPrima::from_json("materias_primas.json");
    let produtos: Vec<Produto> = pcp::Produto::from_json("produtos.json");
    let pedidos: Vec<Pedido> = pcp::Pedido::from_json("pedidos.json");

    println!("Matérias primas:");
    for mp in &materias_primas {
        println!("  - {:?}", mp);
    }

    println!("Produtos:");
    for produto in &produtos {
        println!("  - Nome: {}", produto.nome);
        println!("    Matérias Primas:");
        for mp in &produto.materias_primas {
            println!("      - {}", mp);
        }
        println!("    Tempo de Fabricação: {} horas", produto.tempo_fabricacao);
    }

    println!("Pedidos:");
    for pedido in &pedidos {
        println!("  - Produto: {}", pedido.produto);
        println!("    Data de Entrega: {}", pedido.data_entrega);
    }
}
