# 🐾 LinuxPET

Um aplicativo de **Desktop Pet** leve, flutuante e de alto desempenho para Linux, desenvolvido em **Rust** utilizando aceleração de hardware nativa.

> 🚀 **Objetivo:** Trazer um companheiro interativo e reativo para a sua área de trabalho, consumindo o mínimo de recursos do sistema.

---

## 📸 Sobre o Projeto

O **LinuxPET** é construído do zero utilizando a stack de baixa abstração do ecossistema Rust (`tao` para gerenciamento de janelas com suporte a transparência e `pixels`/`wgpu` para renderização direta na GPU).

### ✨ Funcionalidades Atuais
- [x] **Janela Transparente & Flutuante:** Renderização nativa com suporte a compositor (X11 e Wayland).
- [x] **Modo Always-on-Top:** O mascote permanece visível acima das outras janelas sem atrapalhar o fluxo de trabalho.
- [x] **Alta Performance:** Baixíssimo uso de CPU/RAM graças à renderização otimizada com `pixels` e `wgpu`.

---

## 🗺️ Roadmap de Desenvolvimento (Em Breve)

O projeto está em desenvolvimento ativo. As próximas atualizações trarão mecânicas de física, reatividade e inteligência ao pet:

- [ ] **Movimentação do Pet:**
  - Animações de caminhada e pulo ao longo da borda inferior da tela.
  - Sistema de patrulha e física básica.
- [ ] **Rastreamento do Cursor (Seguir o Mouse):**
  - A cabeça e o olhar do mascote acompanharão a posição do ponteiro do mouse em tempo real.
- [ ] **Sistema de Fadiga & Reatividade:**
  - Monitoramento de inputs do sistema: o mascote demonstrará cansaço/sono quando detectar digitação prolongada ou atividade intensa do usuário.
- [ ] **Interatividade por Clique:**
  - Suporte a arrastar e soltar (Drag & Drop) e interações contextuais ao clicar no mascote.

---

## 🛠️ Tecnologias Utilizadas

- **Linguagem:** [Rust](https://www.rust-lang.org/) 🦀
- **Janelamento:** [`tao`](https://crates.io/crates/tao) (Fork do `winit` mantido pelo time do Tauri)
- **Renderização:** [`pixels`](https://crates.io/crates/pixels) & [`wgpu`](https://wgpu.rs/) (Aceleração por hardware)

---

## 🚀 Como Compilar e Rodar

### Pré-requisitos

Garantir que você tem a ferramenta do Rust (`cargo`) e as bibliotecas do sistema para compilação gráfica instaladas (como `pkg-config`, `libx11`, `libxrandr`, etc.).

### Passos

1. **Clone o repositório:**
   ```bash
   git clone [https://github.com/luizgcarmo/LinuxPET.git](https://github.com/luizgcarmo/LinuxPET.git)
   cd LinuxPET
