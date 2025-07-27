<div align="center">

<img src="https://www.csoftware.dev.br/assets/logo.png" alt="CSoftware Logo" width="120"/>

# 🎰 c_lottery – Loteria Descentralizada by CSoftware
*Contrato inteligente principal (LotteryCore) – MVP 1*

</div>

---

## 🧠 Visão Geral

c_lottery é um projeto de *loteria descentralizada, modular, transparente e auditável, desenvolvido com **Ink!* e executado sobre a *Substrate Contracts Pallet*.  
Este repositório contém o *contrato núcleo*, responsável por:

- ✅ Registro de rodadas de loteria
- 🎟️ Venda de bilhetes
- 🔄 Sorteio com pseudoaleatoriedade (temporária)
- 🪙 Pagamento automático de prêmios
- 🔐 Controle de acesso por proprietário

---

## 📂 Estrutura do Projeto

contracts/ 
	└── lottery-core/ 
		├── Cargo.toml          
		# Configuração do contrato
		├── lib.rs             
		# Código principal Ink! 
		└── README.md           
		# Este arquivo

--

## 🚀 Instalação e Build

### 🧱 Pré-requisitos

- [Rust toolchain](https://rustup.rs)
- [cargo-contract](https://github.com/paritytech/cargo-contract)
- [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node)

### 🛠️ Comandos


```bash commands
# Instale o cargo-contract se ainda não tiver
cargo install cargo-contract --force

# Acesse o diretório do contrato
cd contracts/lottery-core

# Compile o contrato
cargo contract build